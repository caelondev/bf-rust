use core::result::Result::{Err, Ok};
use std::{
    env, fs,
    io::{self, Write},
    process,
};

use bf_rust::BrainfuckRust;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "inst" {
            if args.len() < 3 {
                printerr("no filepath given");
            }

            let src = match fs::read_to_string(&args[2]) {
                Ok(s) => s,
                Err(e) => printerr(&e.to_string()),
            };
            let interpreter = BrainfuckRust::new(&src);
            let instructions = match interpreter.compile() {
                Ok(ops) => ops,
                Err(e) => printerr(&e),
            };

            for inst in instructions {
                println!("{inst:?}");
            }
            return;
        }

        if args[1] == "run" {
            if args.len() < 3 {
                printerr("no filepath given");
            }

            let src = match fs::read_to_string(&args[2]) {
                Ok(s) => s,
                Err(e) => printerr(&e.to_string()),
            };

            let mut interpreter = BrainfuckRust::new(&src);
            let instructions = match interpreter.compile() {
                Ok(ops) => ops,
                Err(e) => printerr(&e),
            };
            match interpreter.run(&instructions) {
                Ok(()) => {}
                Err(e) => printerr(&e),
            }
            return;
        }
    }

    loop {
        let mut src = String::new();

        print!("> ");
        io::stdout().flush().expect("Cannot flush stdout");

        io::stdin().read_line(&mut src).expect("Cannot read stdin");

        let mut interpreter = BrainfuckRust::new(&src);
        let instructions = match interpreter.compile() {
            Ok(ops) => ops,
            Err(e) => {
                eprintln!("error: {e}");
                continue;
            }
        };
        match interpreter.run(&instructions) {
            Ok(()) => {}
            Err(e) => eprintln!("error: {e}"),
        }
    }
}

fn printerr(err: &str) -> ! {
    eprintln!("error: {err}");
    process::exit(1);
}
