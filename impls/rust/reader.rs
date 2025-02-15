use regex::Regex;

use crate::types::Mal;

pub struct Reader<'a> {
    pub position: usize,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Reader<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Reader {
            position: 0,
            tokens,
        }
    }

    pub fn next(&mut self) -> Option<&Token<'a>> {
        if let Some(token) = self.tokens.get(self.position) {
            self.position += 1;
            return Some(token);
        }
        None
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if let Some(token) = self.tokens.get(self.position) {
            return Some(token);
        }
        return None;
    }
}

#[derive(Debug)]
pub struct Token<'a>(&'a str);

pub fn read_str(input: &str) -> Mal {
    let tokens = tokenize(input);

    let mut reader = Reader::new(tokens);
    read_form(&mut reader)
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let re =
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###)
            .unwrap();
    for capture in re.captures_iter(input) {
        if let Some(m) = capture.get(1) {
            tokens.push(Token(m.as_str()));
        }
    }

    tokens
}

fn read_form(reader: &mut Reader) -> Mal {
    match reader.peek() {
        Some(Token("(")) => read_list(reader),
        _ => read_atom(reader).unwrap(),
    }
}

fn read_list(reader: &mut Reader) -> Mal {
    let mut list = vec![];
    reader.next();
    while let Some(Token(c)) = reader.peek() {
        if *c == ")" {
            break;
        }
        list.push(read_form(reader));
    }
    reader.next();
    Mal::List(list)
}

fn read_atom(reader: &mut Reader) -> Option<Mal> {
    if let Some(Token(content)) = reader.next() {
        if let Ok(num) = content.parse::<i32>() {
            return Some(Mal::Int(num));
        }

        return Some(Mal::Sym(content.to_string()));
    }
    None
}
