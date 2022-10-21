
pub type MalNumber = i64;
pub type MalOperatorType = fn(MalNumber, MalNumber) -> MalNumber;

pub enum MalType {
    Boolean(bool),
    List(Vec<MalType>),
    NoValue,
    Number(MalNumber),
    Operator(MalOperatorType),
    Str(String),
    Symbol(String),
    Vector(Vec<MalType>),
}

impl Clone for MalType {
    fn clone(&self) -> Self {
        match self {
            MalType::Boolean(b) => MalType::Boolean(*b),
            MalType::List(list) => {
                let mut new_list = Vec::<MalType>::new();
                for maltype in list.iter() {
                    new_list.push(maltype.clone());
                }
                MalType::List(new_list)
            }
            MalType::NoValue => MalType::NoValue,
            MalType::Number(n) => MalType::Number(*n),
            MalType::Operator(op) => MalType::Operator(*op),
            MalType::Str(s) => MalType::Str(s.clone()),
            MalType::Symbol(s) => MalType::Symbol(s.clone()),
            MalType::Vector(list) => {
                let mut new_list = Vec::<MalType>::new();
                for maltype in list.iter() {
                    new_list.push(maltype.clone());
                }
                MalType::Vector(new_list)
            }
        }
    }
}

impl MalType {
    pub fn print(&self) {
        match self {
            MalType::Boolean(b) => println!("malprinter: Got a boolean: {}", b),
            MalType::List(list) => {
                println!("malprinter: Got a list");
                for sexpr in list {
                    sexpr.print()
                }
                println!("] end list");
            }
            MalType::Vector(list) => {
                println!("malprinter: Got a vector [");
                for sexpr in list {
                    sexpr.print()
                }
                println!("] end vector");
            }
            MalType::Number(n) => println!("malprinter: Got a number: {}", n),
            MalType::Symbol(sym) => println!("malprinter: Got a symbol: {}", sym),
            MalType::Str(s) => println!("malprinter: Got a string: {}", s),
            MalType::Operator(_) => println!("malprinter: Got an operator"),
            MalType::NoValue => println!("malprinter: Got a NoValue"),
        }
    }
}

pub fn mal_add(a: MalNumber, b: MalNumber) -> MalNumber { a + b }
pub fn mal_sub(a: MalNumber, b: MalNumber) -> MalNumber { a - b }
pub fn mal_mul(a: MalNumber, b: MalNumber) -> MalNumber { a * b }
pub fn mal_div(a: MalNumber, b: MalNumber) -> MalNumber { a / b }

