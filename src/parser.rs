use crate::tokenizer::*;

use std::fmt;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unary(Box<Expression>, UnaryOperator),
    Binary(Box<Expression>, Box<Expression>, BinaryOperator),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negative,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub enum ParseErrorType {
    UnterminatedParen,
    InvalidToken(Token),
    UnexpectedEOF,
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorType::UnterminatedParen
                => write!(f, "Unterminated parentheses"),
            ParseErrorType::InvalidToken(t)
                => write!(f, "Invalid token: \"{:?}\"", t),
            ParseErrorType::UnexpectedEOF
                => write!(f, "Unexpected end of file"),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub line_number: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error on line {}:\n\t{}", self.line_number, self.error)
    }
}

pub fn parse(tokens: Vec<TokenMeta>) -> Result<Expression, ParseError> {
    let mut parser = Parser { tokens, pos: 0 };
    return parser.parse_equality();
}

struct Parser {
    tokens: Vec<TokenMeta>,
    pos: usize,
}

impl Parser {
    fn peek_token(&self) -> Option<&Token> { 
        if self.pos >= self.tokens.len() {
            return None;
        }

        let token = &self.tokens[self.pos].token;
        Some(token)
    }

    fn advance_token(&mut self) {
        self.pos += 1;
    }

    fn get_current_line_number(&self) -> usize { 
        self.tokens[self.pos].line_number
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.parse_comparison()?;

        loop {
            match self.peek_token() {
                Some(Token::DoubleEquals) => {
                    self.advance_token();
                    let operator = BinaryOperator::Equals;
                    let rhs: Expression = self.parse_comparison()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                Some(Token::BangEquals) => {
                    self.advance_token();
                    let operator = BinaryOperator::NotEquals;
                    let rhs: Expression = self.parse_comparison()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.parse_term()?;

        loop {
            match self.peek_token() {
                Some(Token::Greater) => {
                    self.advance_token();
                    let operator = BinaryOperator::GreaterThan;
                    let rhs: Expression = self.parse_term()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                Some(Token::GreaterEquals) => {
                    self.advance_token();
                    let operator = BinaryOperator::GreaterThanOrEquals;
                    let rhs: Expression = self.parse_term()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                Some(Token::Less) => {
                    self.advance_token();
                    let operator = BinaryOperator::LessThan;
                    let rhs: Expression = self.parse_term()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                Some(Token::LessEquals) => {
                    let operator = BinaryOperator::LessThanOrEquals;
                    let rhs: Expression = self.parse_term()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.parse_factor()?;

        loop {
            match self.peek_token() {
                Some(Token::Plus) => {
                    self.advance_token();
                    let operator = BinaryOperator::Plus;
                    let rhs: Expression = self.parse_factor()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                Some(Token::Minus) => {
                    self.advance_token();
                    let operator = BinaryOperator::Minus;
                    let rhs: Expression = self.parse_factor()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.parse_unary()?;

        loop {
            match self.peek_token() {
                Some(Token::Asterisk) => {
                    self.advance_token();
                    let operator = BinaryOperator::Times;
                    let rhs: Expression = self.parse_unary()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                Some(Token::Slash) => {
                    self.advance_token();
                    let operator = BinaryOperator::Divide;
                    let rhs: Expression = self.parse_unary()?;
                    expr = Expression::Binary(Box::new(expr), Box::new(rhs), operator);
                },
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        loop {
            match self.peek_token() {
                Some(Token::Minus) => {
                    self.advance_token();
                    let operator = UnaryOperator::Negative;
                    let rhs: Expression = self.parse_unary()?;
                    return Ok(Expression::Unary(Box::new(rhs), operator));
                },
                Some(Token::Bang) => {
                    self.advance_token();
                    let operator = UnaryOperator::Not;
                    let rhs: Expression = self.parse_unary()?;
                    return Ok(Expression::Unary(Box::new(rhs), operator));
                },
                _ => break,
            }
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek_token() {
            Some(Token::NumberLiteral(n)) => {
                let n = *n;
                self.advance_token();
                Ok(Expression::Literal(Literal::Number(n)))
            },
            Some(Token::StringLiteral(s)) => {
                let s = (*s).clone();
                self.advance_token();
                Ok(Expression::Literal(Literal::String(s)))
            },
            Some(Token::Keyword(Keyword::True)) => {
                self.advance_token();
                Ok(Expression::Literal(Literal::Boolean(true)))
            },
            Some(Token::Keyword(Keyword::False)) => {
                self.advance_token();
                Ok(Expression::Literal(Literal::Boolean(false)))
            },
            Some(Token::Keyword(Keyword::Nil)) => {
                self.advance_token();
                Ok(Expression::Literal(Literal::Nil))
            },
            Some(Token::ParenStart) => {
                self.advance_token();
                let expr: Expression = self.parse_equality()?;
                let next_token = self.peek_token();
                if next_token.is_none() || *next_token.unwrap() != Token::ParenEnd {
                    return Err(ParseError {
                        error: ParseErrorType::UnterminatedParen,
                        line_number: self.get_current_line_number(),
                    })
                }
                self.advance_token();
                Ok(expr)
            },
            Some(t) => Err(ParseError {
                error: ParseErrorType::InvalidToken((*t).clone()),
                line_number: self.get_current_line_number(),
            }),
            _ => Err(ParseError {
                error: ParseErrorType::UnexpectedEOF,
                line_number: self.get_current_line_number(),
            }),
        }
    }
}
