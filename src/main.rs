use crate::brainfuck::BrainfuckRust;

pub mod brainfuck;

fn main() {
    let src = r"#
    +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.> A
    ++++++++++. \n
        #";

    let mut interpreter = BrainfuckRust::new(src);
    interpreter.run().unwrap()
}
