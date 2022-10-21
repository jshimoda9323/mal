use regex::Regex;
use crate::maltypes::MalType;
use std::collections::HashMap;

struct Reader<'a> {
    tokens: Vec<&'a str>,
    pos: usize,
    //errors: Vec<String>
}

impl Reader<'_> {
    fn next<'a>(&'a mut self) {
        self.pos += 1;
    }

    fn peek<'a>(&'a self) -> &str {
        match self.tokens.get(self.pos) {
            Some(t) => return t,
            None => return &""
        }
    }

    fn is_eof<'a>(&'a self) -> bool {
        match self.tokens.get(self.pos) {
            Some(_) => false,
            None => true
        }
    }
}

fn read_dict(reader: &mut Reader) -> Result<MalType, &'static str> {
    let end_val: &str;
    let mut found_end_val = false;
    match reader.peek() {
        "{" => end_val = "}",
        _   => end_val = "",
    }
    reader.next();
    let mut str_dict: HashMap<String, MalType> = HashMap::new();
    let mut key_dict: HashMap<String, MalType> = HashMap::new();
    let mut expect_key = true;
    let mut last_key = MalType::Str(String::new());
    while reader.is_eof() == false {
        if reader.peek() == end_val {
            reader.next();
            found_end_val = true;
            break;
        }
        match read_form(reader) {
            Ok(maltype) => {
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
            Err(err_str) => return Err(err_str)
        }
    }
    if !found_end_val {
        return Err("unbalanced hashmap");
    }
    if !expect_key {
        return Err("unbalanced hashmap");
    }
    match end_val {
        "}" => return Ok(MalType::Dictionary(str_dict, key_dict)),
        _ => {},
    }
    return Err("Internal Error: unknown end_val")
}

fn read_list(reader: &mut Reader) -> Result<MalType, &'static str> {
    let end_val: &str;
    let mut found_end_val = false;
    match reader.peek() {
        "(" => end_val = ")",
        "[" => end_val = "]",
        _   => end_val = "",
    }
    reader.next();
    let mut list: Vec<MalType> = Vec::new();
    while reader.is_eof() == false {
        if reader.peek() == end_val {
            reader.next();
            found_end_val = true;
            break;
        }
        match read_form(reader) {
            Ok(maltype) => list.push(maltype),
            Err(err_str) => return Err(err_str)
        }
    }
    if !found_end_val {
        return Err("unbalanced list");
    }
    match end_val {
        ")" => return Ok(MalType::List(list)),
        "]" => return Ok(MalType::Vector(list)),
        _ => {},
    }
    return Err("Internal Error: unknown end_val")
}

fn read_atom(reader: &mut Reader) -> Result<MalType, &'static str> {
    //let atom = MalType::Atom(String::from(reader.peek()));
    let new_token = reader.peek().to_string();
    reader.next();
    match &new_token[0..1] {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            match new_token.parse::<i64>() {
                Ok(val) => Ok(MalType::Number(val)),
                Err(_parse_int_error) => Err("failed to convert string to integer")
            }
        }
        "f" => {
            if new_token == "false" {
                Ok(MalType::Boolean(false))
            } else {
                Ok(MalType::Symbol(new_token))
            }
        }
        "n" => {
            if new_token == "nil" {
                Ok(MalType::NoValue)
            } else {
                Ok(MalType::Symbol(new_token))
            }
        }
        "t" => {
            if new_token == "true" {
                Ok(MalType::Boolean(true))
            } else {
                Ok(MalType::Symbol(new_token))
            }
        }
        ":" => {
            let sl = &new_token[1..];
            Ok(MalType::Keyword(sl.to_string()))
        }
        "-" => {
            if new_token.len() > 1 {
                match new_token.parse::<i64>() {
                    Ok(val) => Ok(MalType::Number(val)),
                    Err(_parse_int_error) => Err("failed to convert string to integer")
                }
            } else {
                Ok(MalType::Symbol(new_token))
            }
        }
        "\"" => {
            match new_token.len() {
                1 => Err("unbalanced string"),
                2 => match &new_token[new_token.len()-1..new_token.len()] {
                    "\"" => Ok(MalType::Str(String::new())),
                    _    => Err("unbalanced string")
                }
                _ => {
                    let sl = &new_token[1..new_token.len()];
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
                                    if c_count == new_token.len()-2 {
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
                //_ => match &new_token[new_token.len()-2..new_token.len()] {
                //    "\\\"" => Err("unbalanced string"),
                //    _      => match &new_token[new_token.len()-1..new_token.len()] {
                //        "\"" => Ok(MalType::Str(new_token)),
                //        _    => Err("unbalanced string")
                //    }
                //}
            }
        }
        _   => Ok(MalType::Symbol(new_token)),
    }
}

fn read_form(reader: &mut Reader) -> Result<MalType, &'static str> {
    let token = reader.peek();
    //println!("read_form: token={}",&token);
    match &token[0..1] {
        "(" => read_list(reader),
        "[" => read_list(reader),
        "{" => read_dict(reader),
        _ => read_atom(reader),
    }
}

fn tokenize<'a>(buffer: &'a str) -> Vec<&'a str> {
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    let mut ret = Vec::new();
    for captures in re.captures_iter(buffer) {
        match captures.get(1) {
            Some(s) => ret.push(s.as_str()),
            None => {}
        };
    }
    return ret
}

pub fn read_str(buffer: String) -> Result<MalType, &'static str> {
    let tokens = tokenize(&buffer);
    //let mut reader = Reader{tokens:tokens,pos:0,errors:Vec::new()};
    let mut reader = Reader{tokens:tokens,pos:0};
    return read_form(&mut reader);
}
