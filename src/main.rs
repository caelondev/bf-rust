use crate::brainfuck::BrainfuckRust;

pub mod brainfuck;

fn main() {
    let src = r"#
    ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
    ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++. hello world
        #";

    let mut interpreter = BrainfuckRust::new(src);
    interpreter.run()
}
