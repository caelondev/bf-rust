use bf_rust::BrainfuckRust;

fn main() {
    let src = r"#
    ++++++++++[>+++++++>++++++++++>+++>+<<<<-]
>++.                    H
>+.                     e
+++++++..+++.           llo
>++.                    (space)
<<+++++++++++++++.      W
>.                      o
+++.------.--------.    rld
>+.                     !
>.                      \n
        #";

    let mut interpreter = BrainfuckRust::new(src);
    match interpreter.run() {
        Ok(()) => {}
        Err(e) => eprintln!("{e}"),
    }
}
