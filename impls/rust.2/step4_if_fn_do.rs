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
                let result = eval(val, repl_env)?;
                let _ = new_str_dict.insert(key.to_string(), result);
            }
            for (key, val) in key_dict.iter() {
                let result = eval(val, repl_env)?;
                let _ = new_key_dict.insert(key.to_string(), result);
            }
            Ok(MalType::Dictionary(new_str_dict, new_key_dict))
        }
        MalType::Function(parms, body) => Ok(MalType::Function(parms.clone(), body.clone())),
        MalType::Keyword(k) => Ok(MalType::Keyword(k.to_string())),
        MalType::List(list) => {
            let mut new_list: Vec<MalType> = Vec::new();
            for sexpr in list {
                let result = eval(sexpr, repl_env)?;
                new_list.push(result);
            }
            Ok(MalType::List(new_list))
        }
        MalType::NoValue => Ok(MalType::NoValue),
        MalType::Number(n) => Ok(MalType::Number(*n)),
        MalType::Operator(op) => Ok(MalType::Operator(*op)),
        MalType::Str(s) => Ok(MalType::Str(s.to_string())),
        MalType::Symbol(sym) => repl_env.get(sym).ok_or(MalErr::SymbolErr1(sym.to_string())),
        MalType::Vector(vec) => {
            let mut new_vec: Vec<MalType> = Vec::new();
            for sexpr in vec {
                let result = eval(sexpr, repl_env)?;
                new_vec.push(result);
            }
            Ok(MalType::Vector(new_vec))
        }
    }
}

fn apply(list: &Vec<MalType>, repl_env: &mut MalEnv) -> Result<MalType, MalErr> {
    if list.len() < 1 { return Err(MalErr::ElementErr1("at least 1 list element for function call".to_string(), format!("{} elements", list.len()))) }
    match &list[0] {
        MalType::Function(fparms, fbody) => {
            if list.len()-1 != fparms.len() { return Err(MalErr::ElementErr1(format!("{} arguments for function call", fparms.len()), format!("{} arguments", list.len()-1))) }
            //let evald_arg_list = eval(&list[1], repl_env)?;
            //let args = if let MalType::List(arg_list) = evald_arg_list { arg_list } else { return Err(MalErr::TypeErr1("list".to_string(), evald_arg_list.prt_type().to_string(), "function argument list".to_string())) };
            //if fparms.len() != args.len() { return Err(MalErr::ElementErr1(format!("{} arguments", fparms.len()), format!("{} arguments for function call", args.len()))) }
            let mut env_pairs = Option::None;
            if list.len() > 1 {
                let mut pair_list = Vec::<(String, MalType)>::new();
                for pair in list[1..].iter().zip(fparms.iter()) {
                //for pair in fparms.iter().zip(args.iter()) {
                    if let MalType::Symbol(sym) = pair.1 {
                        pair_list.push((sym.to_string(), pair.0.clone()));
                    } else { return Err(MalErr::TypeErr1("symbol".to_string(), format!("{}", pair.1.prt_type().to_string()), "parameter of function call".to_string())) }
                }
                env_pairs = Some(pair_list);
            }
            repl_env.new_env(env_pairs);
            let ret_val = eval(fbody, repl_env)?;
            repl_env.drop_env();
            Ok(ret_val)
        }
        MalType::Operator(op) => {
            if list.len() != 3 { return Err(MalErr::ElementErr1("3 list elements for operator function".to_string(), format!("{} list elements", list.len()))) }
            let n1 = if let MalType::Number(n) = &list[1] { n } else { return Err(MalErr::TypeErr1("number".to_string(), list[1].prt_type().to_string(), "argument 1 of operator call".to_string())) };
            let n2 = if let MalType::Number(n) = &list[2] { n } else { return Err(MalErr::TypeErr1("number".to_string(), list[2].prt_type().to_string(), "argument 2 of operator call".to_string())) };
            Ok(MalType::Number(op(*n1, *n2)))
        }
        _ => Err(MalErr::TypeErr1("function or operator".to_string(), list[0].prt_type().to_string(), "first element of list".to_string()))
    }
}

fn handle_let_bindings_internal(list: &Vec<MalType>, repl_env: &mut MalEnv) -> Result<bool, MalErr> {
    if list.len() % 2 == 0 {
        for pair in list.chunks(2) {
            let result = eval(&pair[1], repl_env)?;
            let sym = if let MalType::Symbol(s) = &pair[0] { s } else { return Err(MalErr::TypeErr1("symbol".to_string(),pair[0].prt_type().to_string(),"let* binding".to_string())) };
            repl_env.set(sym, result);
        }
        Ok(true)
    } else {
        Err(MalErr::ElementErr1("an even number of list elements".to_string(), format!("{} list elements", list.len()).to_string()))
    }
}

fn handle_let_bindings(mt: &MalType, repl_env: &mut MalEnv) -> Result<bool, MalErr> {
    match mt {
        MalType::List(list) => handle_let_bindings_internal(list, repl_env),
        MalType::Vector(vec) => handle_let_bindings_internal(vec, repl_env),
        _ => Err(MalErr::TypeErr1("list or vector".to_string(), mt.prt_type().to_string(), "let* binding list".to_string()))
    }
}

fn eval_def(list: &Vec<MalType>, repl_env: &mut MalEnv) -> Result<MalType, MalErr> {
    if list.len() != 3 { return Err(MalErr::ElementErr1("2 arguments for def!".to_string(), format!("{} argument(s)", list.len()-1).to_string())) };
    let evald_second = eval(&list[2], repl_env)?;
    let def_sym = if let MalType::Symbol(sym) = &list[1] { sym } else { return Err(MalErr::TypeErr1("symbol".to_string(), list[1].prt_type().to_string(), "argument 1 to def!".to_string())) };
    repl_env.set(def_sym, evald_second.clone());
    Ok(evald_second)
}

fn eval_let(list: &Vec<MalType>, repl_env: &mut MalEnv) -> Result<MalType, MalErr> {
    if list.len() != 3 { return Err(MalErr::ElementErr1("2 arguments for let*".to_string(), format!("{} argument(s)", list.len()-1))) }
    repl_env.new_env(None);
    let result = handle_let_bindings(&list[1], repl_env);
    if let Err(malerr) = result {
        repl_env.drop_env();
        return Err(malerr);
    }
    let eval_result = eval(&list[2], repl_env);
    repl_env.drop_env();
    eval_result
}

fn eval_if(list: &Vec<MalType>, repl_env: &mut MalEnv) -> Result<MalType, MalErr> {
    if list.len() != 3 && list.len() != 4 { return Err(MalErr::ElementErr1("3 or 4 arguments for if".to_string(), format!("{} arguments", list.len()))) }
    let cond_result = eval(&list[1], repl_env)?;
    match cond_result {
        MalType::Boolean(b) => if !b && list.len() == 4 { eval(&list[3], repl_env) } else { Ok(MalType::NoValue) }
        MalType::NoValue => if list.len() == 4 { eval(&list[3], repl_env) } else { Ok(MalType::NoValue) }
        _ => { eval(&list[2], repl_env) }
    }

}

fn eval(mt: &MalType, repl_env: &mut MalEnv) -> Result<MalType, MalErr> {
    match mt {
        MalType::List(list) => {
            if list.len() == 0 {
                let new_list: Vec<MalType> = Vec::new();
                Ok(MalType::List(new_list))
            } else {
                match &list[0] {
                    MalType::Symbol(sym) => match sym.as_str() {
                        "def!" => eval_def(list, repl_env),
                        "do" => {
                            for elem in list[1..list.len()-1].iter() {
                                let _ = eval_ast(elem, repl_env)?;
                            }
                            eval_ast(&list[list.len()-1], repl_env)
                        }
                        "fn*" => {
                            if list.len() != 3 { return Err(MalErr::ElementErr1("3 elements for a function definition".to_string(), format!("{} elements", list.len()))) }
                            let parms = if let MalType::List(lp) = &list[1] { lp.clone() } else { return Err(MalErr::TypeErr1("list".to_string(), list[1].prt_type().to_string(), "list of parameter names".to_string())) };
                            Ok(MalType::Function(parms, Box::new(list[2].clone())))
                        }
                        "if" => eval_if(list, repl_env),
                        "let*" => eval_let(list, repl_env),
                        _ => {
                            let result = eval_ast(mt, repl_env)?;
                            let evald_list = if let MalType::List(l) = result { l } else { return Err(MalErr::InternalErr2("list not returned")) };
                            apply(&evald_list, repl_env)
                        }
                    }
                    _ => {
                        let result = eval_ast(mt, repl_env)?;
                        let evald_list = if let MalType::List(l) = result { l } else { return Err(MalErr::InternalErr2("list not returned")) };
                        apply(&evald_list, repl_env)
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
    let read_result = read(buffer);
    if let Err(err_str) = read_result { return Err(MalErr::Generic1(err_str.to_string())) }
    let mt = read_result.unwrap();
    let eval_result = eval(&mt, repl_env)?;
    Ok(print(eval_result))
}

fn main() {
    let mut repl_env = MalEnv::new();
    loop {
        //print!("{}", repl_env.prt());
        print!("user> ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        if let Ok(n) = stdin().read_line(&mut buffer) {
            /* Catch a CTRL-d */
            if n == 0 {
                println!("\nEOF");
                break;
            }
            match rep(buffer.trim_end().to_string(), &mut repl_env) {
                Ok(out_str) => println!("{}", out_str),
                Err(malerr) => println!("{}", malerr)
            }
        }
    }
}
