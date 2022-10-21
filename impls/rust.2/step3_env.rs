use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod maltypes;
mod reader;
mod printer;
mod env;

use crate::env::{MalEnv};
use crate::printer::pr_str;
use crate::maltypes::{MalType};
use crate::reader::read_str;
use std::collections::HashMap;

fn print(mt: MalType) -> String {
    return pr_str(&mt, true)
}

fn eval_ast(mt: &MalType, repl_env: &mut MalEnv) -> Result<MalType, &'static str> {
    match mt {
        MalType::Boolean(b) => Ok(MalType::Boolean(*b)),
        MalType::Dictionary(str_dict, key_dict) => {
            let mut new_str_dict = HashMap::<String, MalType>::new();
            let mut new_key_dict = HashMap::<String, MalType>::new();
            for (key, val) in str_dict.iter() {
                match eval(val, repl_env) {
                    Ok(result) => { let _ = new_str_dict.insert(key.to_string(), result); }
                    Err(err_str) => return Err(err_str)
                }
            }
            for (key, val) in key_dict.iter() {
                match eval(val, repl_env) {
                    Ok(result) => { let _ = new_key_dict.insert(key.to_string(), result); }
                    Err(err_str) => return Err(err_str)
                }
            }
            Ok(MalType::Dictionary(new_str_dict, new_key_dict))
        }
        MalType::Keyword(k) => Ok(MalType::Keyword(k.to_string())),
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
        MalType::NoValue => Ok(MalType::NoValue),
        MalType::Number(n) => Ok(MalType::Number(*n)),
        MalType::Operator(op) => Ok(MalType::Operator(*op)),
        MalType::Str(s) => Ok(MalType::Str(s.to_string())),
        MalType::Symbol(sym) => {
            match repl_env.get(sym) {
                Some(value) => Ok(value),
                None => Err("symbol not found")
            }
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
    }
}

fn apply(op: &MalType, arg1: &MalType, arg2: &MalType) -> Result<MalType, &'static str> {
    match op {
        MalType::Operator(f) => match arg1 {
            MalType::Number(n1) => match arg2 {
                MalType::Number(n2) => Ok(MalType::Number(f(*n1, *n2))),
                _ => Err("Argument 2 is not a number")
            }
            _ => Err("Argument 1 is not a number")
        }
        _ => Err("Function name is not an operator")
    }
}

fn handle_let_bindings(mt: &MalType, repl_env: &mut MalEnv) -> Result<bool, &'static str> {
    match mt {
        MalType::List(list) => {
            match list.len() % 2 {
                0 => {
                    for pair in list.chunks(2) {
                        match eval(&pair[1], repl_env) {
                            Ok(result) => {
                                match &pair[0] {
                                    MalType::Symbol(sym) => {
                                        repl_env.set(sym, result);
                                    }
                                    _ => return Err("Expected symbol in let binding")
                                }
                            }
                            Err(err_str) => return Err(err_str)
                        }
                    }
                }
                _ => return Err("let* binding list has non-even number of arguments")
            }
        }
        MalType::Vector(vec) => {
            match vec.len() % 2 {
                0 => {
                    for pair in vec.chunks(2) {
                        match eval(&pair[1], repl_env) {
                            Ok(result) => {
                                match &pair[0] {
                                    MalType::Symbol(sym) => {
                                        repl_env.set(sym, result);
                                    }
                                    _ => return Err("Expected symbol in let binding")
                                }
                            }
                            Err(err_str) => return Err(err_str)
                        }
                    }
                }
                _ => return Err("let* binding list has non-even number of arguments")
            }
        }
        _ => return Err("Expected list of let* bindings")
    }
    Ok(true)
}

fn eval(mt: &MalType, repl_env: &mut MalEnv) -> Result<MalType, &'static str> {
    match mt {
        MalType::List(list) => match list.len() {
            0 => {
                let new_list: Vec<MalType> = Vec::new();
                Ok(MalType::List(new_list))
            }
            _ => {
                match &list[0] {
                    MalType::Symbol(sym) => match sym.as_str() {
                        "def!" => {
                            match list.len() {
                                3 => match eval(&list[2], repl_env) {
                                    Ok(evald_second) => match &list[1] {
                                        MalType::Symbol(def_sym) => {
                                            repl_env.set(def_sym, evald_second.clone());
                                            Ok(evald_second)
                                        }
                                        _ => Err("Argument 1 to def! must be a symbol")
                                    }
                                    Err(err_str) => Err(err_str)
                                }
                                _ => Err("Expected 2 arguments for def!")
                            }
                        }
                        "let*" => {
                            match list.len() {
                                3 => {
                                    repl_env.new_env();
                                    match handle_let_bindings(&list[1], repl_env) {
                                        Err(err_str) => {
                                            repl_env.drop_env();
                                            return Err(err_str);
                                        }
                                        _ => {}
                                    }
                                    match eval(&list[2], repl_env) {
                                        Ok(result) => {
                                            repl_env.drop_env();
                                            Ok(result)
                                        }
                                        Err(err_str) => {
                                            repl_env.drop_env();
                                            Err(err_str)
                                        }
                                    }
                                }
                                _ => Err("Expected 2 arguments for let*")
                            }
                        }
                        _ => match eval_ast(mt, repl_env) {
                            Ok(result) => match result {
                                MalType::List(evald_list) => apply(&evald_list[0], &evald_list[1], &evald_list[2]),
                                _ => Err("Internal error: list not returned")
                            }
                            Err(err_str) => Err(err_str)
                        }
                    }
                    _ => match eval_ast(mt, repl_env) {
                        Ok(result) => match result {
                            MalType::List(evald_list) => apply(&evald_list[0], &evald_list[1], &evald_list[2]),
                            _ => Err("Internal error: list not returned")
                        }
                        Err(err_str) => Err(err_str)
                    }
                }
            }
        }
        _ => eval_ast(mt, repl_env),
    }
}

fn read(buffer: String) -> Result<MalType, &'static str> {
    return read_str(buffer);
}

fn rep(buffer: String, repl_env: &mut MalEnv) -> Result<String, &'static str> {
    match read(buffer) {
        Ok(mt) => match eval(&mt, repl_env) {
            Ok(result) => Ok(print(result)),
            Err(err_str) => Err(err_str)
        }
        Err(err_str) => Err(err_str)
    }
}

fn main() {
    let mut repl_env = MalEnv::new();
    loop {
        //print!("{}", repl_env.prt());
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
                match rep(buffer.trim_end().to_string(), &mut repl_env) {
                    Ok(out_str) => println!("{}", out_str),
                    Err(err_str) => println!("Error: {}", err_str)
                }
            },
            Err(_) => continue,
        }
    }
}
