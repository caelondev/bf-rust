pub struct BrainfuckRust {
    source: Vec<char>,
    tape: Vec<u8>,
    cursor: usize,
}

impl BrainfuckRust {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.chars().collect(),
            tape: Vec::new(),
            cursor: 0,
        }
    }

    pub fn run(&mut self) {
        let instructions = self.clean_source();
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

    fn left(&mut self) {
        if self.cursor >= self.tape.len() {
            let 
            self.tape.resize(self.cursor + 1, 0);
        }
    }
}
