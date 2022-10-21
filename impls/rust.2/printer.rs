use crate::maltypes::MalType;

fn pr_mallist(mallist: &Vec<MalType>) -> String {
    let mut vals: Vec<String> = Vec::new();
    //println!("pr_mallist here 1");
    for types in &mallist[0..mallist.len()]{
        vals.push(pr_str(types));
    }
    let ret: String = vals.join(" ");
    return ret;
}

pub fn pr_str(maltype: &MalType) -> String {
    match maltype {
        MalType::Boolean(b) => match b {
            true => String::from("true"),
            _ => String::from("false"),
        }
        MalType::List(l) => {
            //println!("pr_str: Found start of list");
            String::from("(")+&pr_mallist(&l)+")"
            //println!("pr_str: Found List: {}", &r);
        },
        MalType::Vector(v) => {
            String::from("[")+&pr_mallist(&v)+"]"
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
            //println!("pr_str: Found atom: {}", &a);
            a.to_string()
        }
        MalType::Operator(_) => {
            String::from("internal function-call")
        }
        MalType::NoValue => {
            String::from("nil")
        }
    }
}

