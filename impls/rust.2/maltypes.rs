pub enum MalType {
    List(Vec<MalType>),
    Number(i64),
    Symbol(String),
    Str(String),
}

