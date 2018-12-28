use std::env;
use std::io;
use std::io::Write;
use std::fs;
use std::process;
use lumos::vm;
use lumos::vm::{ InterpretResult };

fn main() {
    // let vm = VM::new();

    let mut args = env::args();

    // Runs for a file if supplied, else REPL
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        args.next();

        let path = match args.next() {
            Some(t) => t,
            None => String::new(),
        };

        run_file(&path);
    } else {
        println!("Usage: lumos [PATH]");
        process::exit(64);
    }
}


fn repl() {
    loop {
        // Prints out arrows for the user to write next to
        print!(">>> ");
        io::stdout().flush()
            .expect("Could not flush output stream.");

        // Reads user input
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read REPL input.");

        // Interprets the line
        vm::interpret(&line);   
    }
}

fn run_file(path: &String) {
    // Reads the file
    let source = fs::read_to_string(path)
        .expect("Failed to read file.");

    // Interprets the source code
    let result = vm::interpret(&source);

    // Exits if there was an error
    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => (),
    }
}
