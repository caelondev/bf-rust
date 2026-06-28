use std::io::{self, Read, Write};

pub mod wasm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Op {
    Add(u32),
    Sub(u32),
    Right(u32),
    Left(u32),
    Output,
    Input,
    SetZero,
    ScanRight(u32),
    ScanLeft(u32),
    MoveMul { offset: i32, factor: u8 },
    LoopStart(usize),
    LoopEnd(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawTok {
    Add(u32),
    Sub(u32),
    Right(u32),
    Left(u32),
    Output,
    Input,
    LoopOpen,
    LoopClose,
}

pub struct BrainfuckRust {
    source: Vec<char>,
    tape: Vec<u8>,
    pc: usize,
    cursor: usize,
    write_fn: Box<dyn FnMut(char)>,
    read_fn: Box<dyn FnMut() -> u8>,
}

impl BrainfuckRust {
    pub fn new(src: &str) -> Self {
        Self::with_io(
            src,
            |ch| print!("{ch}"),
            || {
                io::stdout().flush().ok();
                let mut buf = [0u8; 1];
                match io::stdin().read_exact(&mut buf) {
                    Ok(()) => buf[0],
                    Err(_) => 0, // EOF or read error
                }
            },
        )
    }

    pub fn with_io(
        src: &str,
        write_fn: impl FnMut(char) + 'static,
        read_fn: impl FnMut() -> u8 + 'static,
    ) -> Self {
        Self {
            source: src.chars().collect(),
            tape: vec![0u8; 256],
            pc: 0,
            cursor: 0,
            write_fn: Box::new(write_fn),
            read_fn: Box::new(read_fn),
        }
    }

    pub fn compile(&self) -> Result<Vec<Op>, String> {
        let raw = tokenize(&self.source);
        lower(&raw)
    }

    pub fn run(&mut self, ops: &[Op]) -> Result<(), String> {
        let mut pc = self.pc;
        let mut cursor = self.cursor;

        macro_rules! bail {
            ($($arg:tt)*) => {{
                self.pc = pc;
                self.cursor = cursor;
                return Err(format!($($arg)*));
            }};
        }

        while pc < ops.len() {
            match ops[pc] {
                Op::Add(n) => unsafe {
                    let cell = self.tape.get_unchecked_mut(cursor);
                    *cell = cell.wrapping_add(n as u8);
                },
                Op::Sub(n) => unsafe {
                    let cell = self.tape.get_unchecked_mut(cursor);
                    *cell = cell.wrapping_sub(n as u8);
                },
                Op::Right(n) => {
                    cursor += n as usize;
                    grow_tape(&mut self.tape, cursor);
                }
                Op::Left(n) => {
                    let n = n as usize;
                    if cursor < n {
                        bail!("Tape underflow at instruction {pc}");
                    }
                    cursor -= n;
                }
                Op::Output => {
                    let ch = unsafe { *self.tape.get_unchecked(cursor) } as char;
                    (self.write_fn)(ch);
                }
                Op::Input => {
                    let v = (self.read_fn)();
                    unsafe {
                        *self.tape.get_unchecked_mut(cursor) = v;
                    }
                }
                Op::SetZero => unsafe {
                    *self.tape.get_unchecked_mut(cursor) = 0;
                },
                Op::ScanRight(stride) => {
                    let stride = stride as usize;
                    while unsafe { *self.tape.get_unchecked(cursor) } != 0 {
                        cursor += stride;
                        grow_tape(&mut self.tape, cursor);
                    }
                }
                Op::ScanLeft(stride) => {
                    let stride = stride as usize;
                    while unsafe { *self.tape.get_unchecked(cursor) } != 0 {
                        if cursor < stride {
                            bail!("Tape underflow at instruction {pc}");
                        }
                        cursor -= stride;
                    }
                }
                Op::MoveMul { offset, factor } => {
                    let val = unsafe { *self.tape.get_unchecked(cursor) };
                    if val != 0 {
                        let target = cursor as isize + offset as isize;
                        if target < 0 {
                            bail!("Tape underflow at instruction {pc}");
                        }
                        let target = target as usize;
                        grow_tape(&mut self.tape, target);
                        let add = val.wrapping_mul(factor);
                        unsafe {
                            let t = self.tape.get_unchecked_mut(target);
                            *t = t.wrapping_add(add);
                            *self.tape.get_unchecked_mut(cursor) = 0;
                        }
                    }
                }
                Op::LoopStart(end_idx) => {
                    if unsafe { *self.tape.get_unchecked(cursor) } == 0 {
                        pc = end_idx;
                    }
                }
                Op::LoopEnd(start_idx) => {
                    if unsafe { *self.tape.get_unchecked(cursor) } != 0 {
                        pc = start_idx;
                    }
                }
            }

            pc += 1;
        }

        self.pc = pc;
        self.cursor = cursor;
        Ok(())
    }
}

#[inline(always)]
fn grow_tape(tape: &mut Vec<u8>, idx: usize) {
    if idx >= tape.len() {
        let new_len = (tape.len() * 2).max(idx + 1);
        tape.resize(new_len, 0);
    }
}

fn tokenize(source: &[char]) -> Vec<RawTok> {
    let mut raw: Vec<RawTok> = Vec::with_capacity(source.len());

    for &c in source {
        let tok = match c {
            '+' => RawTok::Add(1),
            '-' => RawTok::Sub(1),
            '>' => RawTok::Right(1),
            '<' => RawTok::Left(1),
            '.' => RawTok::Output,
            ',' => RawTok::Input,
            '[' => RawTok::LoopOpen,
            ']' => RawTok::LoopClose,
            _ => continue,
        };

        match (raw.last_mut(), tok) {
            (Some(RawTok::Add(n)), RawTok::Add(_)) => *n += 1,
            (Some(RawTok::Sub(n)), RawTok::Sub(_)) => *n += 1,
            (Some(RawTok::Right(n)), RawTok::Right(_)) => *n += 1,
            (Some(RawTok::Left(n)), RawTok::Left(_)) => *n += 1,
            _ => raw.push(tok),
        }
    }

    raw
}

fn lower(raw: &[RawTok]) -> Result<Vec<Op>, String> {
    let mut ops: Vec<Op> = Vec::with_capacity(raw.len());
    let mut open_stack: Vec<usize> = Vec::new();
    let mut i = 0;

    while i < raw.len() {
        match raw[i] {
            RawTok::LoopOpen => {
                let t1 = raw.get(i + 1).copied();
                let t2 = raw.get(i + 2).copied();
                let t3 = raw.get(i + 3).copied();
                let t4 = raw.get(i + 4).copied();
                let t5 = raw.get(i + 5).copied();

                if matches!(t1, Some(RawTok::Add(_)) | Some(RawTok::Sub(_)))
                    && t2 == Some(RawTok::LoopClose)
                {
                    ops.push(Op::SetZero);
                    i += 3;
                    continue;
                }

                if let (Some(RawTok::Right(n)), Some(RawTok::LoopClose)) = (t1, t2) {
                    ops.push(Op::ScanRight(n));
                    i += 3;
                    continue;
                }

                if let (Some(RawTok::Left(n)), Some(RawTok::LoopClose)) = (t1, t2) {
                    ops.push(Op::ScanLeft(n));
                    i += 3;
                    continue;
                }

                if let (
                    Some(RawTok::Sub(1)),
                    Some(mv1),
                    Some(RawTok::Add(k)),
                    Some(mv2),
                    Some(RawTok::LoopClose),
                ) = (t1, t2, t3, t4, t5)
                {
                    let offset = match (mv1, mv2) {
                        (RawTok::Right(n1), RawTok::Left(n2)) if n1 == n2 => Some(n1 as i32),
                        (RawTok::Left(n1), RawTok::Right(n2)) if n1 == n2 => Some(-(n1 as i32)),
                        _ => None,
                    };

                    if let Some(offset) = offset {
                        ops.push(Op::MoveMul {
                            offset,
                            factor: (k % 256) as u8,
                        });
                        i += 6;
                        continue;
                    }
                }

                open_stack.push(ops.len());
                ops.push(Op::LoopStart(0));
                i += 1;
            }
            RawTok::LoopClose => {
                let open_idx = open_stack
                    .pop()
                    .ok_or_else(|| format!("Unmatched ']' at token {i}"))?;
                let close_idx = ops.len();
                ops.push(Op::LoopEnd(open_idx));
                ops[open_idx] = Op::LoopStart(close_idx);
                i += 1;
            }
            RawTok::Add(n) => {
                ops.push(Op::Add(n));
                i += 1;
            }
            RawTok::Sub(n) => {
                ops.push(Op::Sub(n));
                i += 1;
            }
            RawTok::Right(n) => {
                ops.push(Op::Right(n));
                i += 1;
            }
            RawTok::Left(n) => {
                ops.push(Op::Left(n));
                i += 1;
            }
            RawTok::Output => {
                ops.push(Op::Output);
                i += 1;
            }
            RawTok::Input => {
                ops.push(Op::Input);
                i += 1;
            }
        }
    }

    if !open_stack.is_empty() {
        return Err(format!(
            "Unterminated loop(s) starting at instruction(s): {:?}",
            open_stack
        ));
    }

    Ok(ops)
}
