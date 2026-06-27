use bf_rust::BrainfuckRust;

fn main() {
    let src = r"#
        #";

    let mut interpreter = BrainfuckRust::new(src);
    match interpreter.run() {
        Ok(()) => {}
        Err(e) => eprintln!("{e}"),
    }
}
