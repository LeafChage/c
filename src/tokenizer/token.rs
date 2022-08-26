#[derive(Clone, Eq, Debug)]
pub enum Token {
    /// +
    Plus,

    /// -
    Minus,

    /// *
    Multiple,

    /// /
    Devide,

    /// !
    Not,

    /// &&
    And,

    /// ||
    Or,

    /// =
    Assign,

    /// ==
    Equal,

    /// !=
    NotEqual,

    /// <
    Less,

    /// <=
    LessEqual,

    /// >
    More,

    /// >=
    MoreEqual,

    /// ;
    EndExpr,

    /// (
    LeftParen,

    /// )
    RightParen,

    /// return
    Return,

    Number(isize),

    Identity(String),
}

impl Token {
    pub fn identity<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Token::Identity(s.into())
    }

    pub fn number(n: isize) -> Self {
        Token::Number(n)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Plus, Token::Plus)
            | (Token::Minus, Token::Minus)
            | (Token::Multiple, Token::Multiple)
            | (Token::Devide, Token::Devide)
            | (Token::Not, Token::Not)
            | (Token::And, Token::And)
            | (Token::Or, Token::Or)
            | (Token::Assign, Token::Assign)
            | (Token::Equal, Token::Equal)
            | (Token::NotEqual, Token::NotEqual)
            | (Token::Less, Token::Less)
            | (Token::LessEqual, Token::LessEqual)
            | (Token::More, Token::More)
            | (Token::MoreEqual, Token::MoreEqual)
            | (Token::EndExpr, Token::EndExpr)
            | (Token::LeftParen, Token::LeftParen)
            | (Token::RightParen, Token::RightParen)
            | (Token::Return, Token::Return)
            | (Token::Number(_), Token::Number(_))
            | (Token::Identity(_), Token::Identity(_)) => true,
            _ => false,
        }
    }
}
