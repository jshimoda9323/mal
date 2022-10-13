use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod maltypes;
mod reader;
mod printer;

use crate::printer::pr_str;

fn print(buffer: String) {
    println!("{}", buffer)
}

fn eval(buffer: &String) -> String {
    match reader::read_str(buffer) {
        Ok(obj) => pr_str(&obj),
        Err(err_str) => err_str.to_string(),
    }
}

fn main() {
    loop {
        print!("user> ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("");
                    break;
                }
                //println!("main: buffer='{}'",&buffer);
                let outline = eval(&buffer.trim_end().to_string());
                print(outline)
            },
            Err(_) => continue,
        }
    }
}
