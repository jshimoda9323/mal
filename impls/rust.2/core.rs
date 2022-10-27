use crate::env::{MalEnv};
use crate::printer::pr_str;
use crate::maltypes::{MalType, MalErr};

pub fn mal_intrinsic_add(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() == 2 {
        let n1 = if let MalType::Number(n) = arg_list[0] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[1].prt_type().to_string(), "argument 1 of intrinsic '+' function".to_string())) };
        let n2 = if let MalType::Number(n) = arg_list[1] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[2].prt_type().to_string(), "argument 2 of intrinsic '+' function".to_string())) };
        Ok(MalType::Number(n1 + n2))
    } else {
        Err(MalErr::ElementErr1("2 arguments to intrinsic '+' function".to_string(), format!("{} arguments", arg_list.len())))
    }
}

pub fn mal_intrinsic_sub(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() == 2 {
        let n1 = if let MalType::Number(n) = arg_list[0] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[1].prt_type().to_string(), "argument 1 of intrinsic '-' function".to_string())) };
        let n2 = if let MalType::Number(n) = arg_list[1] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[2].prt_type().to_string(), "argument 2 of intrinsic '-' function".to_string())) };
        Ok(MalType::Number(n1 - n2))
    } else {
        Err(MalErr::ElementErr1("2 arguments to intrinsic '-' function".to_string(), format!("{} arguments", arg_list.len())))
    }
}

pub fn mal_intrinsic_mul(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() == 2 {
        let n1 = if let MalType::Number(n) = arg_list[0] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[1].prt_type().to_string(), "argument 1 of intrinsic '*' function".to_string())) };
        let n2 = if let MalType::Number(n) = arg_list[1] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[2].prt_type().to_string(), "argument 2 of intrinsic '*' function".to_string())) };
        Ok(MalType::Number(n1 * n2))
    } else {
        Err(MalErr::ElementErr1("2 arguments to intrinsic '*' function".to_string(), format!("{} arguments", arg_list.len())))
    }
}

pub fn mal_intrinsic_div(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() == 2 {
        let n1 = if let MalType::Number(n) = arg_list[0] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[1].prt_type().to_string(), "argument 1 of intrinsic '/' function".to_string())) };
        let n2 = if let MalType::Number(n) = arg_list[1] { n } else { return Err(MalErr::TypeErr1("number".to_string(), arg_list[2].prt_type().to_string(), "argument 2 of intrinsic '/' function".to_string())) };
        Ok(MalType::Number(n1 * n2))
    } else {
        Err(MalErr::ElementErr1("2 arguments to intrinsic '/' function".to_string(), format!("{} arguments", arg_list.len())))
    }
}

pub fn mal_intrinsic_prn(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() > 0 {
        println!("{}", pr_str(&arg_list[0], true));
    }
    Ok(MalType::NoValue)
}

pub fn mal_intrinsic_mk_list(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    Ok(MalType::List(arg_list.clone()))
}

pub fn mal_intrinsic_is_list(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 1 { return Err(MalErr::ElementErr1("1 argument to intrinsic 'list?' function".to_string(), format!("{} arguments", arg_list.len()))) }
    if let MalType::List(_) = &arg_list[0] {
        Ok(MalType::Boolean(true))
    } else {
        Ok(MalType::Boolean(false))
    }
}

pub fn mal_intrinsic_is_empty(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 1 { return Err(MalErr::ElementErr1("1 argument to intrinsic 'empty?' function".to_string(), format!("{} arguments", arg_list.len()))) }
    match &arg_list[0] {
        MalType::List(l) => {
            if l.len() == 0 { Ok(MalType::Boolean(true)) }
            else { Ok(MalType::Boolean(false)) }
        }
        MalType::Vector(v) => {
            if v.len() == 0 { Ok(MalType::Boolean(true)) }
            else { Ok(MalType::Boolean(false)) }
        }
        _ => Err(MalErr::TypeErr1("list or vector".to_string(), arg_list[0].prt_type().to_string(), "argument 1 to intrinsic 'empty?' function".to_string()))
    }
}

pub fn mal_intrinsic_count(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 1 { return Err(MalErr::ElementErr1("1 argument to intrinsic 'count' function".to_string(), format!("{} arguments", arg_list.len()))) }
    match &arg_list[0] {
        MalType::List(l) => Ok(MalType::Number(l.len() as i64)),
        MalType::Vector(v) => Ok(MalType::Number(v.len() as i64)),
        _ => Err(MalErr::TypeErr1("list or vector".to_string(), arg_list[0].prt_type().to_string(), "argument 1 to intrinsic 'count' function".to_string()))
    }
}

fn mal_intrinsic_equal(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 2 { return Err(MalErr::ElementErr1("2 arguments to intrinsic '=' function".to_string(), format!("{} arguments", arg_list.len()))) }
    let a = &arg_list[0];
    let b = &arg_list[1];
    match a {
        MalType::Boolean(value_a) => {
            if let MalType::Boolean(value_b) = b {
                if value_a == value_b { Ok(MalType::Boolean(true)) }
                else { Ok(MalType::Boolean(false)) }
            } else {
                Err(MalErr::TypeErr1(format!("{}", a.prt_type().to_string()), format!("{}", b.prt_type().to_string()), "intrinsic '=' function".to_string()))
            }
        }
        MalType::Dictionary(_, _) => Err(MalErr::Generic1("Dictionary comparison not implemented".to_string())),
        MalType::Function(_, _) => Err(MalErr::Generic1("Function comparison not implemented".to_string())),
        MalType::Intrinsic(_) => Err(MalErr::Generic1("Intrinsic function comparision not implemented".to_string())),
        MalType::Keyword(_) => Err(MalErr::Generic1("Keyword comparison not implemented".to_string())),
        MalType::List(list_a) => {
            if let MalType::List(list_b) = b {
                if list_a.len() == list_b.len() {
                    for (item_a, item_b) in list_a.iter().zip(list_b.iter()) {
                        let item_result = mal_intrinsic_equal(&vec![item_a.clone(), item_b.clone()])?;
                        if let MalType::Boolean(result_val) = item_result {
                            if !result_val {
                                return Ok(MalType::Boolean(false));
                            }
                        } else {
                            return Err(MalErr::InternalErr2("mal_intrinsic_val did not return a MalType::Boolean"))
                        }
                    }
                    Ok(MalType::Boolean(true))
                } else {
                    Err(MalErr::ElementErr1(format!("{} list elements", list_a.len()), format!("{} list elements", list_b.len())))
                }
            } else {
                Err(MalErr::TypeErr1(format!("{}", a.prt_type().to_string()), format!("{}", b.prt_type().to_string()), "intrinsic '=' function".to_string()))
            }
        }
        MalType::Vector(list_a) => {
            if let MalType::Vector(list_b) = b {
                if list_a.len() == list_b.len() {
                    for (item_a, item_b) in list_a.iter().zip(list_b.iter()) {
                        let item_result = mal_intrinsic_equal(&vec![item_a.clone(), item_b.clone()])?;
                        if let MalType::Boolean(result_val) = item_result {
                            if !result_val {
                                return Ok(MalType::Boolean(false));
                            }
                        } else {
                            return Err(MalErr::InternalErr2("mal_intrinsic_val did not return a MalType::Boolean"))
                        }
                    }
                    Ok(MalType::Boolean(true))
                } else {
                    Err(MalErr::ElementErr1(format!("{} vector elements", list_a.len()), format!("{} vector elements", list_b.len())))
                }
            } else {
                Err(MalErr::TypeErr1(format!("{}", a.prt_type().to_string()), format!("{}", b.prt_type().to_string()), "intrinsic '=' function".to_string()))
            }
        }
        MalType::NoValue => {
            if let MalType::NoValue = b {
                Ok(MalType::Boolean(true))
            } else {
                Err(MalErr::TypeErr1(format!("{}", a.prt_type().to_string()), format!("{}", b.prt_type().to_string()), "intrinsic '=' function".to_string()))
            }
        }
        MalType::Number(n1) => {
            if let MalType::Number(n2) = b {
                if n1 == n2 {
                    Ok(MalType::Boolean(true))
                } else {
                    Ok(MalType::Boolean(false))
                }
            } else {
                Err(MalErr::TypeErr1(format!("{}", a.prt_type().to_string()), format!("{}", b.prt_type().to_string()), "intrinsic '=' function".to_string()))
            }
        }
        MalType::Str(s1) => {
            if let MalType::Str(s2) = b {
                if s1 == s2 {
                    Ok(MalType::Boolean(true))
                } else {
                    Ok(MalType::Boolean(false))
                }
            } else {
                Err(MalErr::TypeErr1(format!("{}", a.prt_type().to_string()), format!("{}", b.prt_type().to_string()), "intrinsic '=' function".to_string()))
            }
        }
        MalType::Symbol(_) => Err(MalErr::Generic1("Symbol comparison not implemented".to_string())),
    }
}

fn mal_intrinsic_gt(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 2 { return Err(MalErr::ElementErr1("2 arguments to intrinsic '>' function".to_string(), format!("{} arguments", arg_list.len()))) }
    if let MalType::Number(n1) = &arg_list[0] {
        if let MalType::Number(n2) = &arg_list[1] {
            if n1 > n2 {
                Ok(MalType::Boolean(true))
            } else {
                Ok(MalType::Boolean(false))
            }
        } else {
            Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[1].prt_type().to_string()), "intrinsic '>' function".to_string()))
        }
    } else {
        Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[0].prt_type().to_string()), "intrinsic '>' function".to_string()))
    }
}

fn mal_intrinsic_gte(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 2 { return Err(MalErr::ElementErr1("2 arguments to intrinsic '>=' function".to_string(), format!("{} arguments", arg_list.len()))) }
    if let MalType::Number(n1) = &arg_list[0] {
        if let MalType::Number(n2) = &arg_list[1] {
            if n1 >= n2 {
                Ok(MalType::Boolean(true))
            } else {
                Ok(MalType::Boolean(false))
            }
        } else {
            Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[1].prt_type().to_string()), "intrinsic '>=' function".to_string()))
        }
    } else {
        Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[0].prt_type().to_string()), "intrinsic '>=' function".to_string()))
    }
}

fn mal_intrinsic_lt(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 2 { return Err(MalErr::ElementErr1("2 arguments to intrinsic '<' function".to_string(), format!("{} arguments", arg_list.len()))) }
    if let MalType::Number(n1) = &arg_list[0] {
        if let MalType::Number(n2) = &arg_list[1] {
            if n1 < n2 {
                Ok(MalType::Boolean(true))
            } else {
                Ok(MalType::Boolean(false))
            }
        } else {
            Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[1].prt_type().to_string()), "intrinsic '<' function".to_string()))
        }
    } else {
        Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[0].prt_type().to_string()), "intrinsic '<' function".to_string()))
    }
}

fn mal_intrinsic_lte(arg_list: &Vec<MalType>) -> Result<MalType, MalErr> {
    if arg_list.len() != 2 { return Err(MalErr::ElementErr1("2 arguments to intrinsic '<=' function".to_string(), format!("{} arguments", arg_list.len()))) }
    if let MalType::Number(n1) = &arg_list[0] {
        if let MalType::Number(n2) = &arg_list[1] {
            if n1 <= n2 {
                Ok(MalType::Boolean(true))
            } else {
                Ok(MalType::Boolean(false))
            }
        } else {
            Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[1].prt_type().to_string()), "intrinsic '<=' function".to_string()))
        }
    } else {
        Err(MalErr::TypeErr1("number".to_string(), format!("{}", &arg_list[0].prt_type().to_string()), "intrinsic '<=' function".to_string()))
    }
}

pub fn initialize_env(repl_env: &mut MalEnv) {
    repl_env.set("+",        MalType::Intrinsic(mal_intrinsic_add));
    repl_env.set("-",        MalType::Intrinsic(mal_intrinsic_sub));
    repl_env.set("*",        MalType::Intrinsic(mal_intrinsic_mul));
    repl_env.set("/",        MalType::Intrinsic(mal_intrinsic_div));
    repl_env.set("prn",      MalType::Intrinsic(mal_intrinsic_prn));
    repl_env.set("list",     MalType::Intrinsic(mal_intrinsic_mk_list));
    repl_env.set("list?",    MalType::Intrinsic(mal_intrinsic_is_list));
    repl_env.set("empty?",   MalType::Intrinsic(mal_intrinsic_is_empty));
    repl_env.set("count",    MalType::Intrinsic(mal_intrinsic_count));
    repl_env.set("=",        MalType::Intrinsic(mal_intrinsic_equal));
    repl_env.set(">",        MalType::Intrinsic(mal_intrinsic_gt));
    repl_env.set(">=",       MalType::Intrinsic(mal_intrinsic_gte));
    repl_env.set("<",        MalType::Intrinsic(mal_intrinsic_lt));
    repl_env.set("<=",       MalType::Intrinsic(mal_intrinsic_lte));
}
