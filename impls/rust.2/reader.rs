use regex::Regex;
use crate::maltypes::MalType;

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

fn read_list(reader: &mut Reader) -> Result<MalType, &'static str> {
    let end_val: &str;
    let mut found_end_val = false;
    match reader.peek() {
        "(" => end_val = ")",
        "[" => end_val = "]",
        "{" => end_val = "}",
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
    //println!("read_list returning list len={}",list.len());
    if !found_end_val {
        // TODO GEnerate Error messages!
        return Err("unbalanced list");
    }
    return Ok(MalType::List(list));
}

fn read_atom(reader: &mut Reader) -> Result<MalType, &'static str> {
    //let atom = MalType::Atom(String::from(reader.peek()));
    let new_token = reader.peek().to_string();
    reader.next();
    match &new_token[0..1] {
        "0" => Ok(MalType::Number(new_token)),
        "1" => Ok(MalType::Number(new_token)),
        "2" => Ok(MalType::Number(new_token)),
        "3" => Ok(MalType::Number(new_token)),
        "4" => Ok(MalType::Number(new_token)),
        "5" => Ok(MalType::Number(new_token)),
        "6" => Ok(MalType::Number(new_token)),
        "7" => Ok(MalType::Number(new_token)),
        "8" => Ok(MalType::Number(new_token)),
        "9" => Ok(MalType::Number(new_token)),
        "\"" => {
            match new_token.len() {
                1 => Err("unbalanced string"),
                2 => match &new_token[new_token.len()-1..new_token.len()] {
                    "\"" => Ok(MalType::Str(new_token)),
                    _    => Err("unbalanced string")
                }
                _ => match &new_token[new_token.len()-2..new_token.len()] {
                    "\\\"" => Err("unbalanced string"),
                    _      => match &new_token[new_token.len()-1..new_token.len()] {
                        "\"" => Ok(MalType::Str(new_token)),
                        _    => Err("unbalanced string")
                    }
                }
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
        "{" => read_list(reader),
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

pub fn read_str(buffer: &str) -> Result<MalType, &'static str> {
    let tokens = tokenize(buffer);
    //let mut reader = Reader{tokens:tokens,pos:0,errors:Vec::new()};
    let mut reader = Reader{tokens:tokens,pos:0};
    return read_form(&mut reader);
}
