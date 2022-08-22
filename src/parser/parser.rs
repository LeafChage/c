use super::super::tokenizer::Token;
use super::{Node, Sign};
use combine::{between, choice, many, optional, parser, token, value, EasyParser, Parser, Stream};

parser! {
    fn identity[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token> ]
        {
            token(Token::identity(""))
                .map(|t| if let Token::Identity(t) = t {
                    Node::identity(t)
                }else{
                    unreachable!()
                })
        }
}
parser! {
    fn number[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token> ]
        {
            token(Token::number(0))
                .map(|n| if let Token::Number(n) = n {
                    Node::Number(n)
                }else{
                    unreachable!()
                })
        }
}

parser! {
    pub fn primary[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            let paren_expr = between(
                token(Token::LeftParen),
                token(Token::RightParen),
                expr());

            number()
                .or(identity())
                .or(paren_expr)
        }
}

parser! {
    pub fn unary[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token> ]
        {
            let plus_or_minus = choice((
                    token(Token::Plus),
                    token(Token::Minus)
                    ));

            optional(plus_or_minus)
                .and(primary())
                .map(|(t, n)|
                     match t {
                         Some(Token::Plus) => Node::unary(Sign::Plus, n),
                         Some(Token::Minus) => Node::unary(Sign::Minus, n),
                         Some(_) => unreachable!(),
                         None => n,
                     }
                    )
        }
}

parser! {
    fn _multiple[Input](left: Node)(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            let mul = token(Token::Multiple).with(unary())
                .map(|right| Node::multiple(left.clone(), right));
            let dev = token(Token::Devide).with(unary())
                .map(|right| Node::devide(left.clone(), right));

            choice((mul,dev))
                .then(|left| _multiple(left))
                .or(value(left.clone()))
        }
}
parser! {
    pub fn multiple[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            unary().then(|left| _multiple(left))
        }
}

parser! {
    pub fn _add[Input](left: Node)(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            let plus = token(Token::Plus).with(multiple())
                .map(|right| Node::plus(left.clone(), right));
            let minus = token(Token::Minus).with(multiple())
                .map(|right| Node::minus(left.clone(), right));

            choice((plus, minus))
                .then(|left| _add(left))
                .or(value(left.clone()))
        }
}
parser! {
    pub fn add[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            multiple().then(|left| _add(left))
        }
}

parser! {
    pub fn _relational[Input](left: Node)(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            let greater = token(Token::More)
                .with(add()).map(|right| Node::less(right, left.clone()));
            let less = token(Token::Less)
                .with(add()).map(|right| Node::less(left.clone(), right));
            let greater_than = token(Token::MoreEqual)
                .with(add()) .map(|right| Node::less_equal(right, left.clone()));
            let less_than = token(Token::LessEqual)
                .with(add()) .map(|right| Node::less_equal(left.clone(), right));

            choice((greater, less, greater_than, less_than))
                .then(|left| _relational(left))
                .or(value(left.clone()))
        }
}
parser! {
    pub fn relational[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            add().then(|left| _relational(left))
        }
}

parser! {
    pub fn _equality[Input](left: Node)(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            let equal = token(Token::Equal)
                .with(relational()) .map(|right| Node::equal(left.clone(), right));
            let unequal = token(Token::NotEqual)
                .with(relational()) .map(|right| Node::unequal(left.clone(), right));

            choice((equal, unequal))
                .then(|left| _equality(left))
                .or(value(left.clone()))
        }
}
parser! {
    pub fn equality[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            relational().then(|left| _equality(left))
        }
}

parser! {
    pub fn assign[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            equality().and(optional(token(Token::Assign).with(assign())))
                .map(|(left, right)| Node::assign(left, right))
        }
}

parser! {
    pub fn expr[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            equality()
        }
}

parser! {
    pub fn stmt[Input]()(Input) -> Node
        where [ Input: Stream<Token = Token>]
        {
            expr().skip(token(Token::EndExpr))
        }
}

parser! {
    pub fn program[Input]()(Input) -> Vec<Node>
        where [ Input: Stream<Token = Token>]
        {
            many::<Vec<Node>, _, _>(stmt())
        }
}

#[cfg(test)]
mod tests {
    use super::super::super::tokenizer::tokenize;
    use super::*;
    use combine::EasyParser;

    #[test]
    fn it_multiple() {
        assert_eq!(
            multiple().easy_parse(&tokenize("3 * 5")[..]).unwrap().0,
            Node::multiple(Node::number(3), Node::number(5))
        );
        assert_eq!(
            multiple().easy_parse(&tokenize("3 / 5")[..]).unwrap().0,
            Node::devide(Node::number(3), Node::number(5))
        );
    }
    #[test]
    fn it_add() {
        assert_eq!(
            add().easy_parse(&tokenize("3 * 5")[..]).unwrap().0,
            Node::multiple(Node::number(3), Node::number(5))
        );
        assert_eq!(
            add().easy_parse(&tokenize("3 * 5 - 2")[..]).unwrap().0,
            Node::minus(
                Node::multiple(Node::number(3), Node::number(5)),
                Node::number(2)
            )
        );
        assert_eq!(
            add().easy_parse(&tokenize("3 - 5 * 2")[..]).unwrap().0,
            Node::minus(
                Node::number(3),
                Node::multiple(Node::number(5), Node::number(2)),
            )
        );
    }

    #[test]
    fn it_expr() {
        assert_eq!(
            expr().easy_parse(&tokenize("1*2+(3+4)")[..]).unwrap().0,
            Node::plus(
                Node::multiple(Node::number(1), Node::number(2),),
                Node::plus(Node::number(3), Node::number(4))
            )
        );
        assert_eq!(
            expr().easy_parse(&tokenize("5+6*7")[..]).unwrap().0,
            Node::plus(
                Node::number(5),
                Node::multiple(Node::number(6), Node::number(7))
            )
        );
        assert_eq!(
            expr().easy_parse(&tokenize("(3+5)/2")[..]).unwrap().0,
            Node::devide(
                Node::plus(Node::number(3), Node::number(5)),
                Node::number(2)
            )
        );
        assert_eq!(
            expr().easy_parse(&tokenize("(3+5)/2")[..]).unwrap().0,
            Node::devide(
                Node::plus(Node::number(3), Node::number(5)),
                Node::number(2)
            )
        );
        assert_eq!(
            expr().easy_parse(&tokenize("-3*+5")[..]).unwrap().0,
            Node::multiple(
                Node::unary(Sign::Minus, Node::number(3)),
                Node::unary(Sign::Plus, Node::number(5)),
            )
        );
        assert_eq!(
            expr().easy_parse(&tokenize("1 == 2")[..]).unwrap().0,
            Node::equal(Node::number(1), Node::number(2))
        );
    }
}
