use std::io::{self, Write};

use printer::pr_str;
use reader::read_str;
use types::Mal;
mod printer;
mod reader;
mod types;

fn main() -> io::Result<()> {
    let mut input = String::new();
    loop {
        print!("user> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        if !input.is_empty() {
            println!("{}", rep(&input));
        }
        input.clear();
    }
}

fn rep(instr: &str) -> String {
    print(eval(read(instr)))
}

fn read(instr: &str) -> Mal {
    read_str(instr)
}

fn eval(instr: Mal) -> Mal {
    instr
}

fn print(instr: Mal) -> String {
    pr_str(instr)
}
