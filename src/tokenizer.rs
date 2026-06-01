#[derive(Debug)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Bang,
    BangEquals,
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Literal(Literal),
    Identifier(String),
    ParenStart,
    ParenEnd,
    BraceStart,
    BraceEnd,
    Comma,
    Period,
    Semicolon,
    Operator(Operator),
    Eof,
}

#[derive(Debug)]
pub struct TokenMeta {
    pub token: Token,
    pub line_number: usize,
}

#[derive(Debug)]
pub enum TokenErrorType {
    UnexpectedCharacter(char),
}

#[derive(Debug)]
pub struct TokenError {
    pub error: TokenErrorType,
    pub line_number: usize,
}

pub fn tokenize(input: &str) -> Result<Vec<TokenMeta>, Vec<TokenError>> {
    let mut tokenizer = Tokenizer {
        source: input.chars().collect(),
        tokens: Vec::new(),
        errors: None,
        current_line_number: 0,
        current_char_index: 0,
    };

    while tokenizer.is_input_end() {
        let c: char = tokenizer.get_current_char();
        match c {
            '(' => tokenizer.add_token(Token::ParenStart),
            ')' => tokenizer.add_token(Token::ParenEnd),
            '{' => tokenizer.add_token(Token::BraceStart),
            '}' => tokenizer.add_token(Token::BraceEnd),
            ',' => tokenizer.add_token(Token::Comma),
            '.' => tokenizer.add_token(Token::Period),
            '-' => tokenizer.add_token(Token::Operator(Operator::Minus)),
            '+' => tokenizer.add_token(Token::Operator(Operator::Plus)),
            '*' => tokenizer.add_token(Token::Operator(Operator::Asterisk)),
            ';' => tokenizer.add_token(Token::Semicolon),
            '!' => {
                let next_char_index = tokenizer.current_char_index + 1;
                if tokenizer.is_input_end_at(next_char_index) {
                    match tokenizer.source[next_char_index] {
                        '=' => {
                            tokenizer.add_token(Token::Operator(Operator::BangEquals));
                            tokenizer.advance();
                        },
                        _ => tokenizer.add_token(Token::Operator(Operator::Bang)),
                    }
                }
            }
            '\n' => tokenizer.current_line_number += 1,
            _ => {
                let error = TokenError {
                    error: TokenErrorType::UnexpectedCharacter(c),
                    line_number: tokenizer.current_line_number,
                };
                tokenizer.errors.get_or_insert_with(Vec::new).push(error);
            },
        }
        tokenizer.advance();
    }

    match tokenizer.errors {
        Some(errors) => Err(errors),
        None => Ok(tokenizer.tokens),
    }
}

struct Tokenizer {
    source: Vec<char>,
    tokens: Vec<TokenMeta>,
    errors: Option<Vec<TokenError>>,
    current_line_number: usize,
    current_char_index: usize,
}

impl Tokenizer {
    fn add_token(&mut self, token: Token) {
        let meta = TokenMeta {
            token,
            line_number: self.current_line_number,
        };
        self.tokens.push(meta);
    }

    fn is_input_end(&self) -> bool {
        self.current_char_index < self.source.len()
    }

    fn is_input_end_at(&self, index: usize) -> bool {
        index < self.source.len()
    }

    fn advance(&mut self) {
        self.current_char_index += 1;
    }

    fn get_current_char(&self) -> char {
        self.source[self.current_char_index]
    }
}
