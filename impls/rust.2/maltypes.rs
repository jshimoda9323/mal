

pub type MalNumber = i64;
pub type MalOperatorType = fn(MalNumber, MalNumber) -> MalNumber;

pub enum MalType {
    List(Vec<MalType>),
    Number(MalNumber),
    Symbol(String),
    Str(String),
    Operator(MalOperatorType),
}

impl MalType {
    pub fn print(&self) {
        match self {
            MalType::List(list) => {
                println!("malprinter: Got a list");
                for sexpr in list {
                    sexpr.print()
                }
            }
            MalType::Number(n) => println!("malprinter: Got a number: {}", n),
            MalType::Symbol(sym) => println!("malprinter: Got a symbol: {}", sym),
            MalType::Str(s) => println!("malprinter: Got a string: {}", s),
            MalType::Operator(_) => println!("malprinter: Got an operator"),
        }
    }
}
