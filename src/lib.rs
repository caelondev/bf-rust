use std::io::{self, Read, Write};

pub mod wasm;

pub struct BrainfuckRust {
    source: Vec<char>,
    tape: Vec<u8>,
    pc: usize,
    cursor: usize,
    loop_stack: Vec<usize>,
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
                    Err(_) => 0, // EOF or read error -> convention: set to 0
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
            loop_stack: Vec::new(),
            write_fn: Box::new(write_fn),
            read_fn: Box::new(read_fn),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let tokens = self.clean_source();
        let instructions = self.compress_tokens(tokens);

        while self.pc < instructions.len() {
            let inst: (char, u8) = instructions[self.pc];
            match inst {
                ('+', c) => {
                    self.tape[self.cursor] = self.tape[self.cursor].wrapping_add(c);
                }
                ('-', c) => {
                    self.tape[self.cursor] = self.tape[self.cursor].wrapping_sub(c);
                }
                ('>', c) => {
                    self.right(c);
                }
                ('<', c) => {
                    self.left(c)?;
                }
                ('[', _) => {
                    if self.tape[self.cursor] == 0 {
                        let start = self.pc + 1;
                        let mut depth = 1;
                        while depth != 0 {
                            self.pc += 1;
                            if self.pc >= instructions.len() {
                                return Err(format!(
                                    "Unterminated loop starting at instruction {start}"
                                ));
                            }
                            match instructions[self.pc] {
                                ('[', _) => depth += 1,
                                (']', _) => depth -= 1,
                                _ => {}
                            }
                        }
                    } else {
                        self.loop_stack.push(self.pc);
                    }
                }
                (']', _) => {
                    if self.loop_stack.is_empty() {
                        return Err(format!(
                            "Unmatched closing bracket at instruction {}",
                            self.pc + 1
                        ));
                    }

                    if self.tape[self.cursor] != 0 {
                        self.pc = *self.loop_stack.last().unwrap();
                    } else {
                        self.loop_stack.pop();
                    }
                }
                ('.', _) => {
                    let ch = self.tape[self.cursor] as char;
                    (self.write_fn)(ch);
                }

                (',', _) => {
                    self.tape[self.cursor] = (self.read_fn)();
                }
                _ => unreachable!("source is always clean unless lexer messed up"),
            };

            self.pc += 1;
        }

        if !self.loop_stack.is_empty() {
            return Err(format!(
                "Unterminated loop(s) starting at instruction(s): {:?}",
                self.loop_stack
            ));
        }

        Ok(())
    }

    fn clean_source(&self) -> Vec<char> {
        let mut chars: Vec<char> = vec![];

        for c in &self.source {
            match c {
                '+' | '-' | '>' | '<' | '[' | ']' | '.' | ',' => chars.push(*c),
                _ => continue,
            };
        }

        chars
    }

    fn compress_tokens(&self, tokens: Vec<char>) -> Vec<(char, u8)> {
        let mut rle_tokens = Vec::new();

        for token in tokens {
            if matches!(token, '<' | '>' | '+' | '-') {
                if let Some((last_token, count)) = rle_tokens.last_mut() {
                    if *last_token == token {
                        *count += 1;
                        continue;
                    }
                }
            }

            rle_tokens.push((token, 1));
        }

        rle_tokens
    }

    fn right(&mut self, count: u8) {
        self.cursor += count as usize;

        if self.cursor >= self.tape.len() {
            self.tape.resize(self.cursor + 1, 0);
        }
    }

    fn left(&mut self, count: u8) -> Result<(), String> {
        if self.cursor == 0 {
            return Err("Tape underflow".into());
        }

        self.cursor -= count as usize;
        Ok(())
    }
}
