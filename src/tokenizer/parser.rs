use super::Token;
use combine::{
    attempt, choice, many, many1, optional, parser,
    parser::{
        char::{alpha_num, digit, letter, newline, space, string},
        repeat::skip_until,
    },
    skip_many, token, value, EasyParser, Parser, Stream,
};

parser! {
    fn skip_comment_oneline[Input]()(Input) -> ()
        where [ Input: Stream<Token = char>, ]
        {
            attempt(string("//"))
                .with(skip_until(newline()))
                .with(newline())
                .with(value(()))
        }
}
#[test]
fn it_skip_comment_oneline() {
    assert_eq!(
        skip_comment_oneline().easy_parse(
            r#"// hello world
hello"#
        ),
        Ok(((), "hello"))
    );
}
parser! {
    fn skip_comment_block[Input]()(Input) -> ()
        where [ Input: Stream<Token = char>, ]
        {
            attempt(string("/*"))
                .with(skip_until(string("*/")))
                .with(string("*/"))
                .with(value(()))
        }
}
#[test]
fn it_skip_comment_block() {
    assert_eq!(
        skip_comment_block().easy_parse(
            r#"/* hello world
hello */hello"#
        ),
        Ok(((), "hello"))
    );
}

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
    fn ignore_space_and_comment[Input]()(Input) -> ()
        where [
            Input: Stream<Token = char>,
        ]
        {
            skip_spaces()
                .with(optional(skip_comment_oneline()))
                .with(optional(skip_comment_block()))
                .with(skip_spaces())
                .with(value(()))
        }
}

parser! {
    fn identity[Input]()(Input) -> String
        where [ Input: Stream<Token = char>]
        {
            letter()
                .and(
                    many1::<Vec<_>, _, _>(choice((alpha_num(), token('_'))))
                    .or(value(vec![]))
                    .map(|chars| chars.into_iter().collect::<String>())
                    )
                .map(|(h, tail)| format!("{}{}", h, tail))
        }
}
#[test]
fn it_identity() {
    assert_eq!(identity().easy_parse("to ken").unwrap().0, "to");
    assert_eq!(identity().easy_parse("token").unwrap().0, "token");
}

parser! {
    fn number[Input]()(Input) -> isize
        where [ Input: Stream<Token = char>]
        {
            many1::<Vec<_>, _, _>(digit())
                .map(|nums| {
                    isize::from_str_radix(
                        nums.into_iter()
                        .collect::<String>()
                        .as_ref(),
                        10
                        ).unwrap()
                })
        }
}
#[test]
fn it_number() {
    assert_eq!(number().easy_parse("100").unwrap().0, 100);
}

parser! {
    pub fn tokens[Input]()(Input) -> Vec<Token>
        where[Input: Stream<Token=char>]
        {
            ignore_space_and_comment().with(
                many::<Vec<Token>, _, _>(
                    choice((
                            attempt(string("return")).map(|_| Token::Return),
                            attempt(string("if")).map(|_| Token::If),
                            attempt(string("else")).map(|_| Token::Else),
                            attempt(string("for")).map(|_| Token::For),
                            attempt(string("while")).map(|_| Token::While),
                            attempt(string("&&")).map(|_| Token::And),
                            attempt(string("||")).map(|_| Token::Or) ,
                            attempt(string("==")).map(|_| Token::Equal),
                            attempt(string("!=")).map(|_| Token::NotEqual),
                            attempt(string("<=")).map(|_| Token::LessEqual),
                            attempt(string(">=")).map(|_| Token::MoreEqual),
                            token('+').map(|_| Token::Plus),
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
                            number().map(|n| Token::number(n)),
                            identity().map(|s| Token::identity(s)),
                            )).skip(ignore_space_and_comment())))
        }
}

#[test]
fn it_tokens() {
    assert_eq!(
        tokens().easy_parse(
            "
            a = 3;
            b = 5 * 6 - 8;
            // b = 5 * 6 - 8;
            a + b / 2;
            "
        ),
        Ok((
            vec![
                Token::identity("a"),
                Token::Assign,
                Token::Number(3),
                Token::EndExpr,
                Token::identity("b"),
                Token::Assign,
                Token::Number(5),
                Token::Multiple,
                Token::Number(6),
                Token::Minus,
                Token::Number(8),
                Token::EndExpr,
                Token::identity("a"),
                Token::Plus,
                Token::identity("b"),
                Token::Devide,
                Token::Number(2),
                Token::EndExpr,
            ],
            ""
        ))
    );
}
