use regex::Regex;
use crate::maltypes_step2::MalType;
use std::collections::HashMap;

struct Reader<'a> {
    tokens: Vec<&'a str>,
    pos: usize,
}

impl Reader<'_> {
    fn next(&mut self) -> Option<&str> {
        self.pos += 1;
        match self.tokens.get(self.pos) {
            Some(token) => Some(token),
            None => None
        }
    }
    fn peek(&mut self) -> Option<&str> {
        match self.tokens.get(self.pos+1) {
            Some(token) => Some(token),
            None => None
        }
    }
}

fn read_dict(reader: &mut Reader) -> Result<MalType, &'static str> {
    let end_val = "}";
    let mut found_end_val = false;
    let mut str_dict: HashMap<String, MalType> = HashMap::new();
    let mut key_dict: HashMap<String, MalType> = HashMap::new();
    let mut expect_key = true;
    let mut last_key = MalType::Str(String::new());
    loop {
        match reader.peek() {
            Some(token) => {
                if token == end_val {
                    let _ = reader.next();
                    found_end_val = true;
                    break;
                }
                let maltype = read_form(reader)?;
                if expect_key {
                    expect_key = false;
                    match maltype {
                        MalType::Str(_) => {},
                        MalType::Keyword(_) => {},
                        _ => return Err("Expecting string or keyword for hash key")
                    }
                    last_key = maltype;
                } else {
                    expect_key = true;
                    match &last_key {
                        MalType::Str(s) => {
                            str_dict.insert(s.to_string(), maltype);
                        }
                        MalType::Keyword(k) => {
                            key_dict.insert(k.to_string(), maltype);
                        }
                        _ => return Err("Internal Error: last_key invalid type")
                    }
                }
            }
            None => { break; }
        }
    }
    if !found_end_val {
        return Err("unbalanced hashmap");
    }
    if !expect_key {
        return Err("unbalanced hashmap");
    }
    Ok(MalType::Dictionary(str_dict, key_dict))
}

fn read_list_internal (reader: &mut Reader, end_val: &str) -> Result<Vec<MalType>, &'static str> {
    let mut list: Vec<MalType> = Vec::new();
    let mut found_end_val = false;
    loop {
        match reader.peek() {
            Some(token) => {
                if token == end_val {
                    let _ = reader.next();
                    found_end_val = true;
                    break;
                } else {
                    let maltype = read_form(reader)?;
                    list.push(maltype);
                }
            }
            None => { break; }
        }
    }
    if !found_end_val {
        return Err("unbalanced list");
    }
    Ok(list)
}

fn read_vector(reader: &mut Reader) -> Result<MalType, &'static str> {
    let result = read_list_internal(reader, "]")?;
    Ok(MalType::Vector(result))
}

fn read_list(reader: &mut Reader) -> Result<MalType, &'static str> {
    let result = read_list_internal(reader, ")")?;
    Ok(MalType::List(result))
}

fn read_atom(token: &str) -> Result<MalType, &'static str> {
    match &token[0..1] {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            match token.parse::<i64>() {
                Ok(val) => Ok(MalType::Number(val)),
                Err(_) => Err("failed to convert string to integer")
            }
        }
        "f" => {
            if token == "false" {
                Ok(MalType::Boolean(false))
            } else {
                Ok(MalType::Symbol(token.to_string()))
            }
        }
        "n" => {
            if token == "nil" {
                Ok(MalType::NoValue)
            } else {
                Ok(MalType::Symbol(token.to_string()))
            }
        }
        "t" => {
            if token == "true" {
                Ok(MalType::Boolean(true))
            } else {
                Ok(MalType::Symbol(token.to_string()))
            }
        }
        ":" => {
            let sl = &token[1..];
            Ok(MalType::Keyword(sl.to_string()))
        }
        "-" => {
            if token.len() > 1 {
                match token.parse::<i64>() {
                    Ok(val) => Ok(MalType::Number(val)),
                    Err(_) => Err("failed to convert string to integer")
                }
            } else {
                Ok(MalType::Symbol(token.to_string()))
            }
        }
        "\"" => {
            match token.len() {
                1 => Err("unbalanced string"),
                2 => match &token[token.len()-1..token.len()] {
                    "\"" => Ok(MalType::Str(String::new())),
                    _    => Err("unbalanced string")
                }
                _ => {
                    let sl = &token[1..token.len()];
                    let mut is_escape = false;
                    let mut new_string = String::new();
                    let mut c_count = 0;
                    let mut found_end = false;
                    for c in sl.as_bytes() {
                        match c {
                            b'n' => match is_escape {
                                true => {
                                    new_string.push_str("\n");
                                    is_escape = false;
                                }
                                _ => new_string.push_str("n"),
                            }
                            b'"' => match is_escape {
                                true => {
                                    new_string.push_str("\"");
                                    is_escape = false;
                                }
                                _ => {
                                    if c_count == token.len()-2 {
                                        found_end = true;
                                    } else {
                                        return Err("Internal Error: found doublequote in middle of string");
                                    }
                                }
                            }
                            b'\\' => match is_escape {
                                true => {
                                    new_string.push_str("\\");
                                    is_escape = false;
                                }
                                _ => is_escape = true,
                            }
                            _ => {
                                is_escape = false;
                                new_string.push(*c as char);
                            }
                        }
                        c_count += 1;
                    }
                    if found_end {
                        Ok(MalType::Str(new_string))
                    } else {
                        Err("unbalanced string: no terminating doublequote found for string")
                    }
                }
            }
        }
        _   => Ok(MalType::Symbol(token.to_string())),
    }
}

fn read_form(reader: &mut Reader) -> Result<MalType, &'static str> {
    match reader.next() {
        Some(token) => {
            if token.len() < 1 { return Ok(MalType::NoValue) }
            match &token[0..1] {
                "(" => read_list(reader),
                "[" => read_vector(reader),
                "{" => read_dict(reader),
                _ => read_atom(token),
            }
        }
        None => Err("Internal Error: no more tokens")
    }
}

fn tokenize<'a>(buffer: &'a str) -> Vec<&'a str> {
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    let mut ret = Vec::from([""]);
    for captures in re.captures_iter(buffer) {
        if let Some(s) = captures.get(1) {
            ret.push(s.as_str());
        }
    }
    ret
}

pub fn read_str(buffer: String) -> Result<MalType, &'static str> {
    let tokens = tokenize(&buffer);
    let mut reader = Reader{tokens:tokens,pos:0};
    read_form(&mut reader)
}
