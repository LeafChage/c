use super::Token;
use combine::{
    choice, many, many1, parser,
    parser::char::{alpha_num, digit, letter, space, string},
    skip_many, token, EasyParser, Stream,
};

parser! {
    fn skip_spaces[Input]()(Input) -> ()
        where [
            Input: Stream<Token = char>,
        ]
        { skip_many(space()) }
}
#[test]
fn it_skip_space_with() {
    assert_eq!(skip_spaces().easy_parse("   Ab"), Ok(((), "Ab")));
    assert_eq!(skip_spaces().easy_parse("Ab"), Ok(((), "Ab")));
}

parser! {
    fn identity[Input]()(Input) -> Token
        where [ Input: Stream<Token = char>]
        {
            skip_spaces().with(
                letter()
                .and(many::<Vec<_>, _, _>(choice((alpha_num(), token('_')))))
                .map(|(h, tail)| {
                    let tail = tail.into_iter().collect::<String>();
                    Token::identity(format!("{}{}", h, tail))
            }))
        }
}
#[test]
fn it_identity() {
    assert_eq!(
        identity().easy_parse("token").unwrap().0,
        Token::identity("token")
    );
    assert_eq!(
        identity().easy_parse(" token").unwrap().0,
        Token::identity("token")
    );
}

parser! {
    fn number[Input]()(Input) -> Token
        where [ Input: Stream<Token = char>]
        {
            skip_spaces().with(
                many1::<Vec<_>, _, _>(digit())
                .map(|nums| {
                let n = isize::from_str_radix(
                    nums.into_iter()
                    .collect::<String>()
                    .as_ref(),
                    10
                    ).unwrap();
                Token::Number(n as isize)
            }))
        }
}
#[test]
fn it_number() {
    assert_eq!(number().easy_parse("100").unwrap().0, Token::number(100));
    assert_eq!(number().easy_parse(" 100").unwrap().0, Token::number(100));
}

parser! {
    pub fn tokens[Input]()(Input) -> Vec<Token>
        where[Input: Stream<Token=char>]
        {
            many::<Vec<Token>, _, _>(skip_spaces().with(
                    choice((
                            string("&&").map(|_| Token::And),
                            string("||").map(|_| Token::Or) ,
                            string("==").map(|_| Token::Equal),
                            string("!=").map(|_| Token::NotEqual),
                            string("<=").map(|_| Token::LessEqual),
                            string(">=").map(|_| Token::MoreEqual) ,
                            token('+').map(|_| Token::Plus) ,
                            token('-').map(|_| Token::Minus) ,
                            token('*').map(|_| Token::Multiple) ,
                            token('/').map(|_| Token::Devide) ,
                            token('!').map(|_| Token::Not) ,
                            token('=').map(|_| Token::Assign) ,
                            token('<').map(|_| Token::Less) ,
                            token('>').map(|_| Token::More) ,
                            token(';').map(|_| Token::EndExpr) ,
                            token('(').map(|_| Token::LeftParen) ,
                            token(')').map(|_| Token::RightParen) ,
                            number(),
                            identity()
                           )))
                           )
        }
}

#[test]
fn it_tokens() {
    assert_eq!(
        tokens().easy_parse("+--").unwrap().0,
        vec![Token::Plus, Token::Minus, Token::Minus,]
    );
}
