use std::num::ParseIntError;

use regex::Regex;

pub enum MalType {
    MalList(Vec<MalType>),
    MalInteger(i32),
    MalSymbol(String),
}

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

    pub fn peek(&mut self) -> &Token {
        &self.tokens[self.position]
    }
}

pub struct Token<'a>(&'a str);

fn read_str(input: &str) {
    let tokens = tokenize(input);

    let mut reader = Reader::new(tokens);
    read_form(&mut reader);
}

fn tokenize(input: &str) -> Vec<Token> {
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
        .unwrap();
    let tokens: Vec<Token> = re.find_iter(input).map(|m| Token(m.as_str())).collect();
    tokens
}

fn read_form(reader: &mut Reader) -> MalType {
    let res = match reader.peek() {
        Token("(") => read_list(reader),
        _ => read_atom(reader).unwrap(),
    };
    res
}

fn read_list(reader: &mut Reader) -> MalType {
    let mut list = vec![];
    while let Some(token) = reader.next() {
        let res = match token {
            Token(")") => return MalType::MalList(list),
            _ => read_form(reader),
        };
        list.push(res);
    }
    MalType::MalList(list)
}

#[derive(Debug)]
enum ReadAtomError {
    ParseInt(ParseIntError),
    UnexpectedEOF,
}

impl From<ParseIntError> for ReadAtomError {
    fn from(value: ParseIntError) -> Self {
        ReadAtomError::ParseInt(value)
    }
}

fn read_atom(reader: &mut Reader) -> Result<MalType, ReadAtomError> {
    if let Some(Token(content)) = reader.next() {
        if let Ok(parsed) = content.parse::<i32>() {
            return Ok(MalType::MalInteger(parsed));
        } else {
            return Ok(MalType::MalSymbol(content.to_string()));
        }
    } else {
        return Err(ReadAtomError::UnexpectedEOF);
    }
}
