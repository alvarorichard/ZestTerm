use std::io::{stdout, Write};
use std::path::Path;
use std::{io::stdin, process::Command};

fn main() {
    loop {
        print!(">");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = std::env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                let mut child = Command::new(command).args(args).spawn().unwrap();

                child.wait();
            }
        }
    }
}
