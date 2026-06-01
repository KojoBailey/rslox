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
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    
    // Literals
    StringLiteral(String),
    NumberLiteral(f64),

    // Brackets
    ParenStart,
    ParenEnd,
    BraceStart,
    BraceEnd,

    // Misc Symbols & Operators
    Comma,
    Period,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    BangEquals,
    Equals,
    DoubleEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

#[derive(Debug)]
pub struct TokenMeta {
    pub token: Token,
    pub line_number: usize,
}

#[derive(Debug)]
pub enum TokenErrorType {
    UnexpectedCharacter(char),
    UnterminatedString(String),
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

    while !tokenizer.is_at_end() {
        let current_char = tokenizer.get_current_char();
        let next_char = tokenizer.get_next_char();
        match (current_char, next_char) {
            ('(', _        ) => tokenizer.add_token(Token::ParenStart),
            (')', _        ) => tokenizer.add_token(Token::ParenEnd),
            ('{', _        ) => tokenizer.add_token(Token::BraceStart),
            ('}', _        ) => tokenizer.add_token(Token::BraceEnd),
            (',', _        ) => tokenizer.add_token(Token::Comma),
            ('.', _        ) => tokenizer.add_token(Token::Period),
            ('-', _        ) => tokenizer.add_token(Token::Minus),
            ('+', _        ) => tokenizer.add_token(Token::Plus),
            ('*', _        ) => tokenizer.add_token(Token::Asterisk),
            (';', _        ) => tokenizer.add_token(Token::Semicolon),
            ('/', Some('/')) => tokenizer.advance_until('\n'),
            ('/', _        ) => tokenizer.add_token(Token::Slash),
            ('!', Some('=')) => tokenizer.add_double_token(Token::BangEquals),
            ('!', _        ) => tokenizer.add_token(Token::Bang),
            ('=', Some('=')) => tokenizer.add_double_token(Token::DoubleEquals),
            ('=', _        ) => tokenizer.add_token(Token::Equals),
            ('>', Some('=')) => tokenizer.add_double_token(Token::GreaterEquals),
            ('>', _        ) => tokenizer.add_token(Token::Greater),
            ('<', Some('=')) => tokenizer.add_double_token(Token::LessEquals),
            ('<', _        ) => tokenizer.add_token(Token::Less),
            ('"', _) => {
                let start = tokenizer.current_char_index + 1;
                tokenizer.advance_until('"');
                let end = tokenizer.current_char_index;
                let string = String::from_iter(&tokenizer.source[start..end]);

                if tokenizer.is_at_end() {
                    tokenizer.push_error(TokenErrorType::UnterminatedString(string));
                    continue;
                }
                tokenizer.add_token(Token::StringLiteral(string));
            },
            ('0'..='9', _) => {
                let start = tokenizer.current_char_index;

                let mut does_end_with_period = false;
                while !tokenizer.is_at_end() {
                    let (mut cc, mut nc) = (tokenizer.get_current_char(), tokenizer.get_next_char());
                    if !(cc.is_ascii_digit() || cc == '.') {
                        break;
                    }
                    if cc == '.' && (nc.is_some() && !nc.unwrap().is_ascii_digit()) {
                        does_end_with_period = true;
                        break;
                    }
                    tokenizer.advance();
                }

                let end = tokenizer.current_char_index;
                let num_string = String::from_iter(&tokenizer.source[start..end]);
                let num: f64 = num_string.parse().unwrap();
                tokenizer.add_token(Token::NumberLiteral(num));

                if does_end_with_period {
                    tokenizer.add_token(Token::Period);
                }
            },
            ('\n', _) => {
                tokenizer.current_line_number += 1;
                tokenizer.advance();
            },
            (' ' | '\r' | '\t', _) => tokenizer.advance(),
            (c, _) => {
                if is_alpha(c) {
                    let start = tokenizer.current_char_index;
                    while !tokenizer.is_at_end() && is_alpha_numeric(tokenizer.get_current_char()) {
                        tokenizer.advance();
                    }
                    let end = tokenizer.current_char_index;
                    let identifier = String::from_iter(&tokenizer.source[start..end]);
                    tokenizer.match_identifier(&identifier);
                    continue;
                } 

                tokenizer.push_error(TokenErrorType::UnexpectedCharacter(c));
                tokenizer.advance();
            },
        }
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
    fn push_error(&mut self, token_error: TokenErrorType) {
        let error = TokenError {
            error: token_error,
            line_number: self.current_line_number,
        };
        self.errors.get_or_insert_with(Vec::new).push(error);
    }

    fn add_token(&mut self, token: Token) {
        let meta = TokenMeta {
            token,
            line_number: self.current_line_number,
        };
        self.tokens.push(meta);
        self.advance();
    }

    fn add_double_token(&mut self, token: Token) {
        let meta = TokenMeta {
            token,
            line_number: self.current_line_number,
        };
        self.tokens.push(meta);
        self.advance_n(2);
    }

    fn is_at_end(&self) -> bool {
        self.current_char_index >= self.source.len()
    }

    fn does_input_end_at_next(&self) -> bool {
        self.current_char_index + 1 >= self.source.len()
    }

    fn does_input_end_at(&self, index: usize) -> bool {
        index >= self.source.len()
    }

    fn advance(&mut self) {
        self.current_char_index += 1;
    }

    fn advance_n(&mut self, step: usize) {
        self.current_char_index += step;
    }

    fn advance_until(&mut self, target: char) {
        self.advance();
        while !self.is_at_end() && self.get_current_char() != target {
            if self.get_current_char() == '\n' {
                self.current_line_number += 1;
            }
            self.advance();
        }
    }

    fn get_current_char(&self) -> char {
        self.source[self.current_char_index]
    }

    fn get_next_char(&self) -> Option<char> {
        if self.does_input_end_at_next() {
            return None;
        }
        Some(self.source[self.current_char_index + 1])
    }

    fn match_identifier(&mut self, identifier: &str) {
        match identifier {
            "and"    => self.add_token(Token::Keyword(Keyword::And)),
            "class"  => self.add_token(Token::Keyword(Keyword::Class)),
            "else"   => self.add_token(Token::Keyword(Keyword::Else)),
            "false"  => self.add_token(Token::Keyword(Keyword::False)),
            "for"    => self.add_token(Token::Keyword(Keyword::For)),
            "fun"    => self.add_token(Token::Keyword(Keyword::Fun)),
            "if"     => self.add_token(Token::Keyword(Keyword::If)),
            "nil"    => self.add_token(Token::Keyword(Keyword::Nil)),
            "or"     => self.add_token(Token::Keyword(Keyword::Or)),
            "print"  => self.add_token(Token::Keyword(Keyword::Print)),
            "return" => self.add_token(Token::Keyword(Keyword::Return)),
            "super"  => self.add_token(Token::Keyword(Keyword::Super)),
            "this"   => self.add_token(Token::Keyword(Keyword::This)),
            "true"   => self.add_token(Token::Keyword(Keyword::True)),
            "var"    => self.add_token(Token::Keyword(Keyword::Var)),
            "while"  => self.add_token(Token::Keyword(Keyword::While)),
            _        => self.add_token(Token::Identifier(String::from(identifier))),
        }
    }

}

fn is_alpha(c: char) -> bool { c == '_' || ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) }

fn is_alpha_numeric(c: char) -> bool { is_alpha(c) || c.is_ascii_digit() }
