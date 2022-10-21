use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod maltypes;
mod reader;
mod printer;

use crate::printer::pr_str;
use crate::maltypes::{MalType, mal_add, mal_sub, mal_mul, mal_div};
use crate::reader::read_str;
use std::collections::HashMap;

fn print(mt: MalType) -> String {
    return pr_str(&mt)
}

fn eval_ast(mt: &MalType, repl_env: &HashMap<String, MalType>) -> Result<MalType, &'static str> {
    match mt {
        MalType::Symbol(sym) => {
            match repl_env.get(sym) {
                Some(value) => match value {
                    MalType::Number(v) => Ok(MalType::Number(*v)),
                    MalType::Operator(op) => Ok(MalType::Operator(*op)),
                    _ => Err("symbol value not handled"),
                }
                None => Err("symbol not defined")
            }
        }
        MalType::List(list) => {
            let mut new_list: Vec<MalType> = Vec::new();
            for sexpr in list {
                match eval(sexpr, repl_env) {
                    Ok(result) => new_list.push(result),
                    Err(err_str) => return Err(err_str)
                }
            }
            Ok(MalType::List(new_list))
        }
        MalType::Vector(vec) => {
            let mut new_vec: Vec<MalType> = Vec::new();
            for sexpr in vec {
                match eval(sexpr, repl_env) {
                    Ok(result) => new_vec.push(result),
                    Err(err_str) => return Err(err_str)
                }
            }
            Ok(MalType::Vector(new_vec))
        }
        MalType::Number(n) => Ok(MalType::Number(*n)),
        MalType::Str(s) => Ok(MalType::Str(s.to_string())),
        MalType::Operator(op) => Ok(MalType::Operator(*op)),
        MalType::NoValue => Ok(MalType::NoValue),
    }
}

fn eval(mt: &MalType, repl_env: &HashMap<String, MalType>) -> Result<MalType, &'static str> {
    match mt {
        MalType::List(list) => match list.len() {
            0 => {
                let new_list: Vec<MalType> = Vec::new();
                Ok(MalType::List(new_list))
            }
            _ => {
                match eval_ast(mt, repl_env) {
                    Ok(result) => match result {
                        MalType::List(elist) =>  match elist[0] {
                            MalType::Operator(f) => {
                                if let MalType::Number(a) = elist[1] {
                                    if let MalType::Number(b) = elist[2] {
                                        Ok(MalType::Number(f(a, b)))
                                    } else {
                                        Err("Third argument is not a number")
                                    }
                                } else {
                                    Err("Second argument is not a number")
                                }
                            }
                            _ => Err("First argument to list is not an operator")
                        }
                        _ => Err("Not an operator!")
                    }
                    Err(err_str) => Err(err_str)
                }
            }
        }
        _ => eval_ast(mt, repl_env),
    }
}

fn read(buffer: String) -> Result<MalType, &'static str> {
    return read_str(buffer);
}

fn rep(buffer: String) -> Result<String, &'static str> {
    let mut repl_env = HashMap::<String, MalType>::new();
    repl_env.insert("+".to_string(),MalType::Operator(mal_add));
    repl_env.insert("-".to_string(),MalType::Operator(mal_sub));
    repl_env.insert("*".to_string(),MalType::Operator(mal_mul));
    repl_env.insert("/".to_string(),MalType::Operator(mal_div));
    match read(buffer) {
        Ok(mt) => match eval(&mt, &repl_env) {
            Ok(result) => Ok(print(result)),
            Err(err_str) => Err(err_str)
        }
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
