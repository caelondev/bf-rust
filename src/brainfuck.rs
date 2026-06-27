use std::io::{self, Read, Write};

pub struct BrainfuckRust {
    source: Vec<char>,
    tape: Vec<u8>,
    pc: usize,
    cursor: usize,
    loop_stack: Vec<usize>,
}

impl BrainfuckRust {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.chars().collect(),
            tape: vec![0u8; 256],
            pc: 0,
            cursor: 0,
            loop_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let instructions = self.clean_source();

        while self.pc < instructions.len() {
            let inst = instructions[self.pc];
            match inst {
                '+' => {
                    self.tape[self.cursor] = self.tape[self.cursor].wrapping_add(1);
                }
                '-' => {
                    self.tape[self.cursor] = self.tape[self.cursor].wrapping_sub(1);
                }
                '>' => {
                    self.right();
                }
                '<' => {
                    self.left()?;
                }
                '[' => {
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
                                '[' => depth += 1,
                                ']' => depth -= 1,
                                _ => {}
                            }
                        }
                    } else {
                        self.loop_stack.push(self.pc);
                    }
                }
                ']' => {
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
                '.' => {
                    let ch = self.tape[self.cursor] as char;
                    print!("{ch}");
                }

                ',' => {
                    io::stdout().flush().ok();

                    let mut buf = [0u8; 1];
                    match io::stdin().read_exact(&mut buf) {
                        Ok(()) => self.tape[self.cursor] = buf[0],
                        Err(_) => self.tape[self.cursor] = 0,
                    }
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

    fn right(&mut self) {
        self.cursor += 1;

        if self.cursor >= self.tape.len() {
            self.tape.resize(self.cursor + 1, 0);
        }
    }

    fn left(&mut self) -> Result<(), String> {
        if self.cursor == 0 {
            return Err("Tape underflow".into());
        }

        self.cursor -= 1;
        Ok(())
    }
}
