use anyhow::{anyhow, Result};
use regex::Regex;

use crate::types::Mal;

pub struct Reader<'a> {
    position: usize,
    tokens: Vec<Token<'a>>,
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

pub fn read_str(input: &str) -> Result<Mal> {
    let tokens = tokenize(input)?;
    let mut reader = Reader::new(tokens);

    match read_form(&mut reader) {
        Ok(res) => Ok(res),
        Err(err) => Err(err),
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let re =
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###)?;
    for capture in re.captures_iter(input) {
        if let Some(m) = capture.get(1) {
            tokens.push(Token(m.as_str()));
        }
    }

    Ok(tokens)
}

fn read_form(reader: &mut Reader) -> Result<Mal> {
    match reader.peek() {
        Some(Token("(")) => Ok(read_list(reader)?),
        _ => Ok(read_atom(reader).unwrap()),
    }
}

fn read_list(reader: &mut Reader) -> Result<Mal> {
    reader.next();
    let mut list = vec![];
    while let Some(Token(c)) = reader.peek() {
        if *c == ")" {
            reader.next();
            return Ok(Mal::List(list));
        }
        list.push(read_form(reader)?);
    }
    Err(anyhow!("unbalanced"))
}

fn read_atom(reader: &mut Reader) -> Result<Mal> {
    if let Some(Token(content)) = reader.next() {
        if let Ok(num) = content.parse::<i32>() {
            return Ok(Mal::Int(num));
        }

        match *content {
            "true" => return Ok(Mal::True),
            "false" => return Ok(Mal::False),
            "nil" => return Ok(Mal::Nil),
            _ => return Ok(Mal::Sym(content.to_string())),
        }
    }
    Ok(Mal::Nil)
}
