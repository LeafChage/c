#[derive(Clone, PartialEq, Eq, Debug)]
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

    /// if
    If,

    /// else
    Else,

    /// while
    While,

    /// for
    For,

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
