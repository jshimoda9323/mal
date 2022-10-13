use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn print(buffer: String) {
    println!("{}", buffer)
}

fn eval(buffer: String) -> String {
    buffer
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
                let outline = eval(buffer.trim_end().to_string());
                print(outline)
            },
            Err(_) => continue,
        }
    }
}
