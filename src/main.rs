#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn do_exit(args: &[&str]) {
    if args.len() != 1 {
        println!("exit: too many arguments");
        return;
    }

    match args[0].parse::<i32>() {
        Ok(n) => process::exit(n),
        Err(_) => println!("exit: received non-integer argument")
    }

}

fn process_command(cmd: &str, args: &[&str]) {
    match cmd {
        "exit" => do_exit(args),
        _ => println!("{}: command not found", cmd),
    }
}


fn main() {

    loop {

        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!");


        let input_split: Vec<_> = input.trim().split_whitespace().collect();


        match input_split.split_first() {
            Some((f, r)) => process_command(f, r),
            None => (),
        };
    }
}
