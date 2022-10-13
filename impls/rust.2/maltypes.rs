pub enum MalType {
    List(Vec<MalType>),
    Number(String),
    Symbol(String),
    Str(String),
}

