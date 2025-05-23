#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeywordColon,
    KeywordPlus,
    KeywordQuestion,
    LiteralInt(i32),
    LiteralBool(bool),
}

pub fn token_to_string(t: Token) -> String {
    match t {
        Token::KeywordColon => String::from(":"),
        Token::KeywordPlus => String::from("+"),
        Token::KeywordQuestion => String::from("?"),
        Token::LiteralInt(i) => format!("{}", i),
        Token::LiteralBool(b) => format!("{}", b),
    }
}
