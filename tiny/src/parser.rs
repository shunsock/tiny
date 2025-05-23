use crate::value_object::ast::{BinaryOperation, Expr, Stmt};
use crate::value_object::token::{Token, token_to_string};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken {
        expected: Option<Token>,
        actual: Token,
    },
}

pub fn parse_error_to_message(e: ParseError) -> String {
    match e {
        ParseError::UnexpectedEOF => "unexpected end of input".to_string(),
        ParseError::UnexpectedToken { expected, actual } => {
            if expected.is_none() {
                format!("[Unexpected Token] actual: {}", token_to_string(actual))
            } else {
                format!(
                    "[Unexpected Token] expected: {}, actual: {}",
                    token_to_string(expected.unwrap()),
                    token_to_string(actual)
                )
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        match self.next() {
            Some(actual) if actual == expected => Ok(()),
            Some(actual) => Err(ParseError::UnexpectedToken {
                expected: Some(expected.clone()),
                actual: actual.clone(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expr()?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_add_expr()?;

        if let Some(Token::KeywordQuestion) = self.peek() {
            Ok(self.parse_if_expr(expr))?
        } else {
            Ok(expr)
        }
    }
    fn parse_if_expr(&mut self, cond: Expr) -> Result<Expr, ParseError> {
        self.expect(&Token::KeywordQuestion)?; // consume '?'
        let thn = self.parse_expr()?;
        self.expect(&Token::KeywordColon)?; // consume ':'
        let els = self.parse_expr()?;

        Ok(Expr::If {
            cond: Box::new(cond),
            thn: Box::new(thn),
            els: Box::new(els),
        })
    }

    fn parse_add_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary_expr()?;
        while matches!(self.peek(), Some(Token::KeywordPlus)) {
            self.next(); // consume '+'
            let right = self.parse_primary_expr()?;
            left = Expr::BinOp(Box::new(BinaryOperation::Add {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        Ok(left)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
        match self.next() {
            Some(Token::LiteralInt(n)) => Ok(Expr::Int(*n)),
            Some(Token::LiteralBool(b)) => Ok(Expr::Bool(*b)),
            Some(Token::LiteralFloat(f)) => Ok(Expr::Float(*f)),
            Some(actual) => Err(ParseError::UnexpectedToken {
                expected: None,
                actual: actual.clone(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}
