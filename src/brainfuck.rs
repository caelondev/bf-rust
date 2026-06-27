pub struct BrainfuckRust {
    source: Vec<char>,
    tape: Vec<u8>,
    cursor: usize,
}

impl BrainfuckRust {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.chars().collect(),
            tape: vec![0u8; 256],
            cursor: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let instructions = self.clean_source();

        for inst in instructions {
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
                // '[' => {}
                // ']' => {}
                '.' => {
                    let ch = self.tape[self.cursor] as char;
                    print!("{ch}");
                }
                // ',' => {}
                _ => unreachable!("source is always clean unless lexer messed up"),
            };
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
