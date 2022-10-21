use crate::maltypes::MalType;

fn pr_mallist(mallist: &Vec<MalType>, print_readably: bool) -> String {
    let mut vals: Vec<String> = Vec::new();
    //println!("pr_mallist here 1");
    for types in &mallist[0..mallist.len()]{
        vals.push(pr_str(types, print_readably));
    }
    let ret: String = vals.join(" ");
    return ret;
}

pub fn pr_str(maltype: &MalType, print_readably: bool) -> String {
    match maltype {
        MalType::Boolean(b) => match b {
            true => String::from("true"),
            _ => String::from("false"),
        }
        MalType::Keyword(k) => {
            String::from(":")+k.as_str()
        }
        MalType::List(l) => {
            //println!("pr_str: Found start of list");
            String::from("(")+&pr_mallist(&l, print_readably)+")"
            //println!("pr_str: Found List: {}", &r);
        },
        MalType::Vector(v) => {
            String::from("[")+&pr_mallist(&v, print_readably)+"]"
        }
        MalType::Number(a) => {
            //println!("pr_str: Found atom: {}", &a);
            a.to_string()
        }
        MalType::Symbol(a) => {
            //println!("pr_str: Found atom: {}", &a);
            a.to_string()
        }
        MalType::Str(a) => {
            if print_readably {
                let mut ret_str = String::from("\"");
                for c in a.as_bytes() {
                    match c {
                        b'\n' => ret_str.push_str("\\n"),
                        b'\\' => ret_str.push_str("\\\\"),
                        b'"'  => ret_str.push_str("\\\""),
                        _ => ret_str.push(*c as char),
                    }
                }
                ret_str.push_str("\"");
                ret_str
            } else {
                a.to_string()
            }
        }
        MalType::Operator(_) => {
            String::from("internal function-call")
        }
        MalType::NoValue => {
            String::from("nil")
        }
    }
}

