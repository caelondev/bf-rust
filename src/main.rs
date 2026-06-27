use std::{
    env,
    io::{self, Write},
};

use bf_rust::BrainfuckRust;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // TODO: This is for reading from a file
        // but im too lazy to implement it today
        return;
    }

    loop {
        let mut src = String::new();

        print!("> ");
        io::stdout().flush().expect("Cannot flush stdout");

        io::stdin().read_line(&mut src).expect("Cannot read stdin");

        let mut interpreter = BrainfuckRust::new(&src);
        match interpreter.run() {
            Ok(()) => {}
            Err(e) => eprintln!("{e}"),
        }
    }
}
