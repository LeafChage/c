use super::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Expected(Vec<u8>),
}
pub type Result<T> = std::result::Result<T, Error>;

impl From<Vec<std::ops::RangeInclusive<u8>>> for Error {
    fn from(rs: Vec<std::ops::RangeInclusive<u8>>) -> Self {
        let mut result = vec![];
        for r in rs {
            for v in r.into_iter() {
                result.push(u8::from(v));
            }
        }
        Self::Expected(result)
    }
}

impl From<Vec<u8>> for Error {
    fn from(rs: Vec<u8>) -> Self {
        Self::Expected(rs)
    }
}

/**
 * ignore token
 */
fn skip_spaces(src: &[u8]) -> &[u8] {
    match src {
        [h, tail @ ..] if (*h as char).is_whitespace() => skip_spaces(tail),
        _ => src,
    }
}

#[test]
fn it_skip_space_with() {
    assert_eq!(skip_spaces("   Ab".as_bytes()), "Ab".as_bytes());
    assert_eq!(skip_spaces("Ab".as_bytes()), "Ab".as_bytes());
}

fn skip_until_newline(src: &[u8]) -> Result<&[u8]> {
    match src {
        [b'\n', tail @ ..] => Ok(tail),
        [b'\r', tail @ ..] => Ok(tail),
        [_, tail @ ..] => skip_until_newline(tail),
        _ => Err(Error::from(vec![b'\n', b'\r'])),
    }
}
fn skip_comment_oneline(src: &[u8]) -> Result<&[u8]> {
    match src {
        [b'/', b'/', tail @ ..] => skip_until_newline(tail),
        _ => Ok(src),
    }
}

#[test]
fn it_skip_comment_oneline() {
    assert_eq!(
        skip_comment_oneline(
            r#"// hello world
hello"#
                .as_bytes()
        ),
        Ok("hello".as_bytes())
    );
}

fn skip_until_end_of_block_comment(src: &[u8]) -> Result<&[u8]> {
    match src {
        [b'*', b'/', tail @ ..] => Ok(tail),
        [_, tail @ ..] => skip_until_end_of_block_comment(tail),
        _ => Err(Error::from(vec![b'*', b'/'])),
    }
}
fn skip_comment_block(src: &[u8]) -> Result<&[u8]> {
    match src {
        [b'/', b'*', tail @ ..] => skip_until_end_of_block_comment(tail),
        _ => Ok(src),
    }
}

#[test]
fn it_skip_comment_block() {
    assert_eq!(
        skip_comment_block(
            r#"/* hello world
hello */hello"#
                .as_bytes()
        ),
        Ok("hello".as_bytes())
    );
}

fn ignore_space_and_comment(src: &[u8]) -> Result<&[u8]> {
    let src = skip_spaces(src);
    let src = skip_comment_oneline(src)?;
    let src = skip_comment_block(src)?;
    Ok(skip_spaces(src))
}

/**
 * number
 */
fn number_inner(src: &[u8]) -> (String, &[u8]) {
    if let [head, tail @ ..] = src {
        match head {
            b'0'..=b'9' => {
                let (head2, tail) = number_inner(tail);
                (format!("{}{}", *head as char, head2), tail)
            }
            _ => (String::new(), src),
        }
    } else {
        (String::new(), src)
    }
}

fn number(src: &[u8]) -> Result<(isize, &[u8])> {
    let (n, src) = number_inner(src);
    if n == String::new() {
        Err(Error::from(vec![b'0'..=b'9']))
    } else {
        Ok((isize::from_str_radix(n.as_ref(), 10).unwrap(), src))
    }
}

#[test]
fn it_number() {
    assert_eq!(number("100".as_bytes()), Ok((100, "".as_bytes())));
}

/**
 * identity
 */
fn identity_inner(src: &[u8]) -> (String, &[u8]) {
    if let [head, tail @ ..] = src {
        match head {
            b'a'..=b'z' | b'_' | b'A'..=b'Z' => {
                let (head2, tail) = identity_inner(tail);
                (format!("{}{}", *head as char, head2), tail)
            }
            _ => (String::new(), src),
        }
    } else {
        (String::new(), src)
    }
}
fn identity(src: &[u8]) -> Result<(String, &[u8])> {
    if let [head, tail @ ..] = src {
        match head {
            b'a'..=b'z' | b'A'..=b'Z' => {
                let (head2, tail2) = identity_inner(tail);
                Ok((format!("{}{}", *head as char, head2), tail2))
            }
            _ => Err(Error::from(vec![b'a'..=b'z', b'A'..=b'Z'])),
        }
    } else {
        Err(Error::from(vec![b'a'..=b'z', b'A'..=b'Z']))
    }
}

#[test]
fn it_identity() {
    assert_eq!(
        identity("to ken".as_bytes()),
        Ok(("to".to_owned(), " ken".as_bytes()))
    );
    assert_eq!(
        identity("token".as_bytes()),
        Ok(("token".to_owned(), "".as_bytes()))
    );
}

/**
 * token
 */
fn token(src: &[u8]) -> Result<(Token, &[u8])> {
    match src {
        [b'i', b'f', src @ ..] => Ok((Token::If, src)),
        [b'e', b'l', b's', b'e', src @ ..] => Ok((Token::Else, src)),
        [b'f', b'o', b'r', src @ ..] => Ok((Token::For, src)),
        [b'w', b'h', b'i', b'l', b'e', src @ ..] => Ok((Token::While, src)),
        [b'r', b'e', b't', b'u', b'r', b'n', src @ ..] => Ok((Token::Return, src)),
        [b'&', b'&', src @ ..] => Ok((Token::And, src)),
        [b'|', b'|', src @ ..] => Ok((Token::Or, src)),
        [b'=', b'=', src @ ..] => Ok((Token::Equal, src)),
        [b'!', b'=', src @ ..] => Ok((Token::NotEqual, src)),
        [b'<', b'=', src @ ..] => Ok((Token::LessEqual, src)),
        [b'>', b'=', src @ ..] => Ok((Token::MoreEqual, src)),
        [b'+', src @ ..] => Ok((Token::Plus, src)),
        [b'-', src @ ..] => Ok((Token::Minus, src)),
        [b'*', src @ ..] => Ok((Token::Multiple, src)),
        [b'/', src @ ..] => Ok((Token::Devide, src)),
        [b'!', src @ ..] => Ok((Token::Not, src)),
        [b'=', src @ ..] => Ok((Token::Assign, src)),
        [b'<', src @ ..] => Ok((Token::Less, src)),
        [b'>', src @ ..] => Ok((Token::More, src)),
        [b';', src @ ..] => Ok((Token::EndExpr, src)),
        [b'(', src @ ..] => Ok((Token::LeftParen, src)),
        [b')', src @ ..] => Ok((Token::RightParen, src)),
        [b'{', src @ ..] => Ok((Token::LeftBlock, src)),
        [b'}', src @ ..] => Ok((Token::RightBlock, src)),
        [b'0'..=b'9', ..] => number(src)
            .map(|(n, src)| Ok((Token::number(n), src)))
            .unwrap(),
        [b'a'..=b'z' | b'A'..=b'Z', ..] => identity(src)
            .map(|(s, src)| Ok((Token::identity(s), src)))
            .unwrap(),
        _ => Err(Error::from(vec![
            b'i', b'e', b'f', b'w', b'&', b'|', b'=', b'!', b'<', b'>', b'+', b'-', b'*', b'/',
            b'!', b'=', b'<', b'>', b';', b'(', b')', b'{', b'}',
        ])),
    }
}

pub fn tokens(src: &[u8]) -> Result<(Vec<Token>, &[u8])> {
    let mut result = vec![];
    let mut src = src;
    loop {
        src = ignore_space_and_comment(src)?;
        if src.len() <= 0 {
            break;
        }

        match token(src) {
            Ok((token, output)) => {
                result.push(token);
                src = output;
            }
            Err(e) => return Err(e),
        }
    }
    Ok((result, src))
}

#[test]
fn it_tokens() {
    assert_eq!(
        tokens(
            "
            a = 3;
            b = 5 * 6 - 8;
            // b = 5 * 6 - 8;
            a + b / 2;
            "
            .as_bytes()
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
            "".as_bytes()
        ))
    );
}
