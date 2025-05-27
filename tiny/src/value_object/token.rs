#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeywordColon,
    KeywordPlus,
    KeywordQuestion,
    ParenLeft,
    ParenRight,
    LiteralInt(i32),
    LiteralFloat(f32),
    LiteralBool(bool),
}

pub fn token_to_string(t: Token) -> String {
    match t {
        Token::KeywordColon => String::from(":"),
        Token::KeywordPlus => String::from("+"),
        Token::KeywordQuestion => String::from("?"),
        Token::ParenLeft => String::from("("),
        Token::ParenRight => String::from(")"),
        Token::LiteralInt(i) => format!("{}", i),
        Token::LiteralFloat(f) => format!("{}", f),
        Token::LiteralBool(b) => format!("{}", b),
    }
}
