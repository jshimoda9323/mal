use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod maltypes;
mod reader;
mod printer;

use crate::printer::pr_str;
use crate::maltypes::MalType;
use crate::reader::read_str;

fn print(mt: MalType) -> String {
    return pr_str(&mt)
}

fn eval(mt: MalType) -> MalType {
    return mt
}

fn read(buffer: String) -> Result<MalType, &'static str> {
    return read_str(buffer);
}

fn rep(buffer: String) -> Result<String, &'static str> {
    match read(buffer) {
        Ok(mt) => Ok(print(eval(mt))),
        Err(err_str) => Err(err_str)
    }
}

fn main() {
    loop {
        print!("user> ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(n) => {
                /* Catch a CTRL-d */
                if n == 0 {
                    println!("");
                    break;
                }
                match rep(buffer.trim_end().to_string()) {
                    Ok(out_str) => println!("{}", out_str),
                    Err(err_str) => println!("Error: {}", err_str)
                }
            },
            Err(_) => continue,
        }
    }
}
