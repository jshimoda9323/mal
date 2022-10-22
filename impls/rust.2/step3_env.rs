use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod maltypes;
mod reader;
mod printer;
mod env;

use crate::env::{MalEnv};
use crate::printer::pr_str;
use crate::maltypes::{MalType, MalErr};
use crate::reader::read_str;
use std::collections::HashMap;

fn print(mt: MalType) -> String {
    return pr_str(&mt, true)
}

fn eval_ast(mt: &MalType, repl_env: &mut MalEnv) -> Result<MalType, MalErr> {
    match mt {
        MalType::Boolean(b) => Ok(MalType::Boolean(*b)),
        MalType::Dictionary(str_dict, key_dict) => {
            let mut new_str_dict = HashMap::<String, MalType>::new();
            let mut new_key_dict = HashMap::<String, MalType>::new();
            for (key, val) in str_dict.iter() {
                match eval(val, repl_env) {
                    Ok(result) => { let _ = new_str_dict.insert(key.to_string(), result); }
                    Err(malerr) => return Err(malerr)
                }
            }
            for (key, val) in key_dict.iter() {
                match eval(val, repl_env) {
                    Ok(result) => { let _ = new_key_dict.insert(key.to_string(), result); }
                    Err(malerr) => return Err(malerr)
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
                    Err(malerr) => return Err(malerr)
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
                None => Err(MalErr::SymbolErr1(sym.to_string()))
            }
        }
        MalType::Vector(vec) => {
            let mut new_vec: Vec<MalType> = Vec::new();
            for sexpr in vec {
                match eval(sexpr, repl_env) {
                    Ok(result) => new_vec.push(result),
                    Err(malerr) => return Err(malerr)
                }
            }
            Ok(MalType::Vector(new_vec))
        }
    }
}

fn apply(op: &MalType, arg1: &MalType, arg2: &MalType) -> Result<MalType, MalErr> {
    match op {
        MalType::Operator(f) => match arg1 {
            MalType::Number(n1) => match arg2 {
                MalType::Number(n2) => Ok(MalType::Number(f(*n1, *n2))),
                _ => Err(MalErr::TypeErr1("number".to_string(), arg2.prt_type().to_string(), "argument 2".to_string()))
            }
            _ => Err(MalErr::TypeErr1("number".to_string(), arg2.prt_type().to_string(), "argument 1".to_string()))
        }
        _ => Err(MalErr::TypeErr1("operator".to_string(), arg2.prt_type().to_string(), "a function call".to_string()))
    }
}

fn handle_let_bindings(mt: &MalType, repl_env: &mut MalEnv) -> Result<bool, MalErr> {
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
                                    _ => return Err(MalErr::TypeErr1("symbol".to_string(),pair[0].prt_type().to_string(),"let* binding".to_string()))
                                }
                            }
                            Err(malerr) => return Err(malerr)
                        }
                    }
                }
                _ => return Err(MalErr::ElementErr1("an even number of list elements".to_string(), format!("{} list elements", list.len()).to_string()))
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
                                    _ => return Err(MalErr::TypeErr1("symbol".to_string(),pair[0].prt_type().to_string(),"let* binding".to_string()))
                                }
                            }
                            Err(malerr) => return Err(malerr)
                        }
                    }
                }
                _ => return Err(MalErr::ElementErr1("an even number of list elements".to_string(), format!("{} list elements", vec.len()).to_string()))
            }
        }
        _ => return Err(MalErr::TypeErr1("list or vector".to_string(), mt.prt_type().to_string(), "let* binding list".to_string()))
    }
    Ok(true)
}

fn eval(mt: &MalType, repl_env: &mut MalEnv) -> Result<MalType, MalErr> {
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
                                        _ => Err(MalErr::TypeErr1("symbol".to_string(), list[1].prt_type().to_string(), "argument 1 to def!".to_string()))
                                    }
                                    Err(malerr) => Err(malerr)
                                }
                                _ => Err(MalErr::ElementErr1("2 arguments for def!".to_string(), format!("{} argument(s)", list.len()-1).to_string()))
                            }
                        }
                        "let*" => {
                            match list.len() {
                                3 => {
                                    repl_env.new_env();
                                    match handle_let_bindings(&list[1], repl_env) {
                                        Err(malerr) => {
                                            repl_env.drop_env();
                                            return Err(malerr);
                                        }
                                        _ => {}
                                    }
                                    match eval(&list[2], repl_env) {
                                        Ok(result) => {
                                            repl_env.drop_env();
                                            Ok(result)
                                        }
                                        Err(malerr) => {
                                            repl_env.drop_env();
                                            Err(malerr)
                                        }
                                    }
                                }
                                _ => Err(MalErr::ElementErr1("2 arguments for let*".to_string(), format!("{} argument(s)", list.len()-1)))
                            }
                        }
                        _ => match eval_ast(mt, repl_env) {
                            Ok(result) => match result {
                                MalType::List(evald_list) => apply(&evald_list[0], &evald_list[1], &evald_list[2]),
                                _ => Err(MalErr::InternalErr2("list not returned"))
                            }
                            Err(err_str) => Err(err_str)
                        }
                    }
                    _ => match eval_ast(mt, repl_env) {
                        Ok(result) => match result {
                            MalType::List(evald_list) => apply(&evald_list[0], &evald_list[1], &evald_list[2]),
                            _ => Err(MalErr::InternalErr2("list not returned"))
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

fn rep(buffer: String, repl_env: &mut MalEnv) -> Result<String, MalErr> {
    match read(buffer) {
        Ok(mt) => match eval(&mt, repl_env) {
            Ok(result) => Ok(print(result)),
            Err(malerr) => Err(malerr)
        }
        Err(err_str) => Err(MalErr::Generic1(err_str.to_string()))
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
                    Err(malerr) => println!("{}", malerr)
                }
            },
            Err(_) => continue,
        }
    }
}
