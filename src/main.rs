use crate::brainfuck::BrainfuckRust;

pub mod brainfuck;

fn main() {
    let src = r"#
    ,.> ask input, print and goto next tape
    ++++++++++. newline
        #";

    let mut interpreter = BrainfuckRust::new(src);
    match interpreter.run() {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {e}"),
    }
}
