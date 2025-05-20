use tailcall::tailcall;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeywordColon,
    KeywordPlus,
    KeywordQuestion,
    LiteralInt(i32),
}

pub fn token_to_string(t: Token) -> String {
    match t {
        Token::KeywordColon => String::from(":"),
        Token::KeywordPlus => String::from("+"),
        Token::KeywordQuestion => String::from("?"),
        Token::LiteralInt(i) => format!("{}", i),
    }
}

pub enum TokenizeError {
    ParseIntError,
    UnexpectedCharacter(char),
}

pub fn tokenize_error_to_message(e: TokenizeError) -> String {
    match e {
        TokenizeError::ParseIntError => "Failed to parse int".to_string(),
        TokenizeError::UnexpectedCharacter(c) => format!("Unexpected character: {}", c),
    }
}

pub(crate) struct Tokenizer;

impl Tokenizer {
    #[tailcall]
    pub fn tokenize(stream: &str) -> Result<Vec<Token>, TokenizeError> {
        Self::tokenize_recursive(stream.trim_start(), vec![])
    }

    #[tailcall]
    fn tokenize_recursive(stream: &str, mut tokens: Vec<Token>) -> Result<Vec<Token>, TokenizeError> {
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
            c if c.is_ascii_digit() => {
                match parse_int_token(stream, c) {
                    Ok((token, rest)) => {
                        tokens.push(token);
                        Self::tokenize_recursive(rest, tokens)
                    }
                    Err(e) => Err(e),
                }
            }
            c if c.is_whitespace() => {
                Self::tokenize_recursive(rest, tokens)
            }
            c => Err(TokenizeError::UnexpectedCharacter(c)),
        }
    }
}

fn parse_int_token(stream: &str, first: char) -> Result<(Token, &str), TokenizeError> {
    let mut num = first.to_string();
    let mut consumed = first.len_utf8();

    for (i, c) in stream[consumed..].char_indices() {
        if c.is_ascii_digit() {
            num.push(c);
            consumed += c.len_utf8();
        } else {
            break;
        }
    }

    match num.parse::<i32>() {
        Ok(n) => Ok((Token::LiteralInt(n), &stream[consumed..])),
        Err(_) => Err(TokenizeError::ParseIntError),
    }
}