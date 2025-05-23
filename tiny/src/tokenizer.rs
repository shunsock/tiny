use crate::value_object::token::Token;
use tailcall::tailcall;

pub enum TokenizeError {
    ParseIntError,
    UnexpectedCharacter(char),
    UnexpectedKeyword(String),
}

pub fn tokenize_error_to_message(e: TokenizeError) -> String {
    match e {
        TokenizeError::ParseIntError => "Failed to parse int".to_string(),
        TokenizeError::UnexpectedCharacter(c) => format!("Unexpected character: {}", c),
        TokenizeError::UnexpectedKeyword(k) => format!("Unexpected keyword: {}", k),
    }
}

pub(crate) struct Tokenizer;

impl Tokenizer {
    #[tailcall]
    pub fn tokenize(stream: &str) -> Result<Vec<Token>, TokenizeError> {
        Self::tokenize_recursive(stream.trim_start(), vec![])
    }

    #[tailcall]
    fn tokenize_recursive(
        stream: &str,
        mut tokens: Vec<Token>,
    ) -> Result<Vec<Token>, TokenizeError> {
        if stream.is_empty() {
            return Ok(tokens);
        }

        let mut chars = stream.chars();
        let first = chars.next().unwrap();
        let rest = chars.as_str();

        match first {
            '+' => {
                tokens.push(Token::KeywordPlus);
                Self::tokenize_recursive(rest, tokens)
            }
            ':' => {
                tokens.push(Token::KeywordColon);
                Self::tokenize_recursive(rest, tokens)
            }
            '?' => {
                tokens.push(Token::KeywordQuestion);
                Self::tokenize_recursive(rest, tokens)
            }
            c if c.is_whitespace() => Self::tokenize_recursive(rest, tokens),
            c if c.is_ascii_digit() || c == '-' => match parse_int_token(stream, c) {
                Ok((token, rest)) => {
                    tokens.push(token);
                    Self::tokenize_recursive(rest, tokens)
                }
                Err(e) => Err(e),
            },
            c if c.is_ascii() => match parse_str_token(stream, c) {
                Ok((token, rest)) => {
                    tokens.push(token);
                    Self::tokenize_recursive(rest, tokens)
                }
                Err(e) => Err(e),
            },
            c => Err(TokenizeError::UnexpectedCharacter(c)),
        }
    }
}

fn parse_int_token(stream: &str, first: char) -> Result<(Token, &str), TokenizeError> {
    let mut numeric: String = first.to_string();
    let mut consumed: usize = first.len_utf8();
    let mut is_float: bool = false;

    for (_, c) in stream[consumed..].char_indices() {
        match c {
            '0'..='9' => {
                numeric.push(c);
                consumed += c.len_utf8();
            }
            '.' => {
                is_float = true;
                numeric.push(c);
                consumed += c.len_utf8();
            }
            _ => break,
        }
    }

    match is_float {
        true => {
            let n: f32 = numeric
                .parse::<f32>()
                .map_err(|_| TokenizeError::ParseIntError)?;
            Ok((Token::LiteralFloat(n), &stream[consumed..]))
        }
        false => {
            let n: i32 = numeric
                .parse::<i32>()
                .map_err(|_| TokenizeError::ParseIntError)?;
            Ok((Token::LiteralInt(n), &stream[consumed..]))
        }
    }
}

fn parse_str_token(stream: &str, first: char) -> Result<(Token, &str), TokenizeError> {
    let mut token_candidate: String = first.to_string();
    let mut consumed: usize = first.len_utf8();

    for (_, c) in stream[consumed..].char_indices() {
        if c.is_ascii_alphanumeric() {
            token_candidate.push(c);
            consumed += c.len_utf8();
        } else {
            break;
        }
    }

    match token_candidate.as_str() {
        "true" => Ok((Token::LiteralBool(true), &stream[consumed..])),
        "false" => Ok((Token::LiteralBool(false), &stream[consumed..])),
        _ => Err(TokenizeError::UnexpectedKeyword(token_candidate)),
    }
}
