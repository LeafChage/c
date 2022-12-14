use super::super::tokenizer::Token;
use super::Node;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Expected(Vec<Token>),
}
pub type Result<T> = std::result::Result<T, Error>;

pub struct Parser {
    id_jar: HashMap<String, usize>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            id_jar: HashMap::new(),
        }
    }

    fn make_offset(&mut self, key: &str) -> usize {
        if let Some(offset) = self.id_jar.get(key) {
            *offset
        } else {
            let offset = self.id_jar.len() * 8;
            let _ = self.id_jar.insert(key.into(), offset);
            offset
        }
    }

    fn identity<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Identity(head), tail @ ..] => {
                let offset = self.make_offset(head);
                Ok((Node::local_variable(head, offset), tail))
            }
            _ => Err(Error::Expected(vec![Token::identity("")])),
        }
    }

    fn number<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Number(head), tokens @ ..] => Ok((Node::number(*head), tokens)),
            _ => Err(Error::Expected(vec![Token::number(0)])),
        }
    }

    fn in_paren<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::LeftParen, tokens @ ..] => {
                let (node, tokens) = self.expr(tokens)?;
                match tokens {
                    [Token::RightParen, tokens @ ..] => Ok((node, tokens)),
                    _ => Err(Error::Expected(vec![Token::RightParen])),
                }
            }
            _ => Err(Error::Expected(vec![Token::LeftParen])),
        }
    }

    fn primary<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        self.identity(tokens)
            .or(self.number(tokens))
            .or(self.in_paren(tokens))
    }

    fn unary<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Plus, Token::Number(n), tokens @ ..] => Ok((Node::number(*n), tokens)),
            [Token::Minus, Token::Number(n), tokens @ ..] => Ok((Node::number(*n * -1), tokens)),
            _ => self.primary(tokens),
        }
    }

    fn _multiple<'a>(&mut self, tokens: &'a [Token], left: Node) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Multiple, tokens @ ..] => {
                let (right, tokens) = self.unary(tokens)?;
                self._multiple(tokens, Node::multiple(left, right))
            }
            [Token::Devide, tokens @ ..] => {
                let (right, tokens) = self.unary(tokens)?;
                self._multiple(tokens, Node::devide(left, right))
            }
            _ => Ok((left, tokens)),
        }
    }
    fn multiple<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (left, tokens) = self.unary(tokens)?;
        self._multiple(tokens, left)
    }

    fn _add<'a>(&mut self, tokens: &'a [Token], left: Node) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Plus, tokens @ ..] => {
                let (right, tokens) = self.multiple(tokens)?;
                self._add(tokens, Node::plus(left, right))
            }
            [Token::Minus, tokens @ ..] => {
                let (right, tokens) = self.multiple(tokens)?;
                self._add(tokens, Node::minus(left, right))
            }
            _ => Ok((left, tokens)),
        }
    }

    fn add<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (left, tokens) = self.multiple(tokens)?;
        self._add(tokens, left)
    }

    fn _relational<'a>(&mut self, tokens: &'a [Token], left: Node) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::More, tokens @ ..] => {
                let (right, tokens) = self.add(tokens)?;
                self._relational(tokens, Node::less(right, left))
            }
            [Token::Less, tokens @ ..] => {
                let (right, tokens) = self.add(tokens)?;
                self._relational(tokens, Node::less(left, right))
            }
            [Token::MoreEqual, tokens @ ..] => {
                let (right, tokens) = self.add(tokens)?;
                self._relational(tokens, Node::less_equal(right, left))
            }
            [Token::LessEqual, tokens @ ..] => {
                let (right, tokens) = self.add(tokens)?;
                self._relational(tokens, Node::less_equal(left, right))
            }
            _ => Ok((left, tokens)),
        }
    }
    fn relational<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (left, tokens) = self.add(tokens)?;
        self._relational(tokens, left)
    }

    fn _equality<'a>(&mut self, tokens: &'a [Token], left: Node) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Equal, tokens @ ..] => {
                let (right, tokens) = self.relational(tokens)?;
                self._equality(tokens, Node::equal(right, left))
            }
            [Token::NotEqual, tokens @ ..] => {
                let (right, tokens) = self.relational(tokens)?;
                self._equality(tokens, Node::unequal(left, right))
            }
            _ => Ok((left, tokens)),
        }
    }
    fn equality<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (left, tokens) = self.relational(tokens)?;
        self._equality(tokens, left)
    }

    fn assign_right<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Assign, tokens @ ..] => self.assign(tokens),
            _ => Err(Error::Expected(vec![Token::Assign])),
        }
    }
    fn assign<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (left, tokens) = self.equality(tokens)?;
        let right = self.assign_right(tokens);
        match (left, right) {
            (Node::LocalVariable(id, offset), Ok((right, tokens))) => Ok((
                Node::assign(Node::local_variable(id, offset), right),
                tokens,
            )),
            (left, Err(..)) => Ok((left, tokens)),
            (_, _) => unimplemented!("expect left is local variable"),
        }
    }

    fn expr<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        self.assign(tokens)
    }

    fn return_n<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (node, tokens) = self.expr(tokens)?;
        match tokens {
            [Token::EndExpr, tail @ ..] => Ok((Node::return_n(node), tail)),
            _ => Err(Error::Expected(vec![Token::EndExpr])),
        }
    }

    fn if_else_body<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Else, tokens @ ..] => self.stmt(tokens),
            _ => Err(Error::Expected(vec![Token::Else])),
        }
    }
    fn if_n<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (condition, tokens) = self.in_paren(tokens)?;
        let (body, tokens) = self.stmt(tokens)?;

        if let Ok((else_body, tokens)) = self.if_else_body(tokens) {
            Ok((Node::if_n(condition, body, Some(else_body)), tokens))
        } else {
            Ok((Node::if_n(condition, body, None), tokens))
        }
    }

    fn for_condition<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::EndExpr, tokens @ ..] => Err(Error::Expected(vec![])),
            _ => {
                let (node, tokens) = self.expr(tokens)?;
                match tokens {
                    [Token::EndExpr, tokens @ ..] => Ok((node, tokens)),
                    _ => Err(Error::Expected(vec![Token::EndExpr])),
                }
            }
        }
    }
    fn for_condition_third<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::RightParen, _tokens @ ..] => Err(Error::Expected(vec![])),
            _ => {
                let (node, tokens) = self.expr(tokens)?;
                match tokens {
                    [Token::RightParen, tokens @ ..] => Ok((node, tokens)),
                    _ => Err(Error::Expected(vec![Token::RightParen])),
                }
            }
        }
    }
    fn for_n<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::LeftParen, tokens @ ..] => {
                let (condition1, tokens) = if let Ok((cnd, tokens)) = self.for_condition(tokens) {
                    (Some(cnd), tokens)
                } else {
                    (None, tokens)
                };
                let (condition2, tokens) = if let Ok((cnd, tokens)) = self.for_condition(tokens) {
                    (Some(cnd), tokens)
                } else {
                    (None, tokens)
                };
                let (condition3, tokens) =
                    if let Ok((cnd, tokens)) = self.for_condition_third(tokens) {
                        (Some(cnd), tokens)
                    } else {
                        (None, tokens)
                    };

                let (body, tokens) = self.stmt(tokens)?;
                Ok((
                    Node::for_n(condition1, condition2, condition3, body),
                    tokens,
                ))
            }
            _ => Err(Error::Expected(vec![Token::LeftParen])),
        }
    }

    fn while_n<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let (condition, tokens) = self.in_paren(tokens)?;
        let (body, tokens) = self.stmt(tokens)?;
        Ok((Node::while_n(condition, body), tokens))
    }

    fn block<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        let mut nodes = vec![];
        let mut tokens = tokens;
        loop {
            match tokens {
                [Token::RightBlock, tokens @ ..] => {
                    return Ok((Node::block(nodes), tokens));
                }
                [_, _tokens @ ..] => match self.stmt(tokens) {
                    Ok((node, _tokens)) => {
                        nodes.push(node);
                        tokens = _tokens
                    }
                    Err(e) => return Err(e),
                },
                _ => return Err(Error::Expected(vec![Token::RightBlock])),
            }
        }
    }

    fn stmt<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::LeftBlock, tokens @ ..] => self.block(tokens),
            [Token::Return, tokens @ ..] => self.return_n(tokens),
            [Token::If, tokens @ ..] => self.if_n(tokens),
            [Token::For, tokens @ ..] => self.for_n(tokens),
            [Token::While, tokens @ ..] => self.while_n(tokens),
            _ => {
                let (node, tokens) = self.expr(tokens)?;
                match tokens {
                    [Token::EndExpr, tail @ ..] => Ok((node, tail)),
                    _ => Err(Error::Expected(vec![Token::EndExpr])),
                }
            }
        }
    }

    pub fn program<'a>(&mut self, tokens: &'a [Token]) -> Result<(Vec<Node>, &'a [Token])> {
        let mut tokens = tokens;
        let mut stmts = vec![];
        while tokens.len() > 0 {
            match self.stmt(tokens) {
                Ok((node, _tokens)) => {
                    tokens = _tokens;
                    stmts.push(node);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok((stmts, tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tokenizer::tokenize;
    use super::*;
    #[test]
    fn it_block() {
        let mut parser = Parser::new();
        let tokens = tokenize(
            "
                n = 0;
                m = 0;
                for(i=0; i<1; i=i+1) {
                    n = n + i;
                    m = m + i;
                }
            ",
        );
        let n = Node::local_variable("n", 8 * 0);
        let m = Node::local_variable("m", 8 * 1);
        let i = Node::local_variable("i", 8 * 2);
        assert_eq!(
            parser.program(&tokens[..]),
            Ok((
                vec![
                    Node::assign(n.clone(), Node::number(0)),
                    Node::assign(m.clone(), Node::number(0)),
                    Node::for_n(
                        Some(Node::assign(i.clone(), Node::number(0))),
                        Some(Node::less(i.clone(), Node::number(1))),
                        Some(Node::assign(
                            i.clone(),
                            Node::plus(i.clone(), Node::number(1),)
                        )),
                        Node::block(vec![
                            Node::assign(n.clone(), Node::plus(n.clone(), i.clone())),
                            Node::assign(m.clone(), Node::plus(m.clone(), i.clone())),
                        ]),
                    ),
                ],
                &[] as &[Token]
            ))
        );
    }

    #[test]
    fn it_program() {
        let mut parser = Parser::new();
        let tokens = tokenize(
            "
                a = 3;
                b = 5 * 6 - 8;
                return a + b / 2;

                n = 0;
                m = 0;
                for(i=0; i<1; i=i+1) {
                    n = n + i;
                    m = m + i;
                }
                if(1==1) a + b;
                while(1==1) a + b;
            ",
        );
        let a = Node::local_variable("a", 8 * 0);
        let b = Node::local_variable("b", 8 * 1);
        let n = Node::local_variable("n", 8 * 2);
        let m = Node::local_variable("m", 8 * 3);
        let i = Node::local_variable("i", 8 * 4);
        assert_eq!(
            parser.program(&tokens[..]),
            Ok((
                vec![
                    Node::assign(a.clone(), Node::number(3)),
                    Node::assign(
                        b.clone(),
                        Node::minus(
                            Node::multiple(Node::number(5), Node::number(6)),
                            Node::number(8),
                        )
                    ),
                    Node::return_n(Node::plus(
                        a.clone(),
                        Node::devide(b.clone(), Node::number(2))
                    )),
                    Node::assign(n.clone(), Node::number(0)),
                    Node::assign(m.clone(), Node::number(0)),
                    Node::for_n(
                        Some(Node::assign(i.clone(), Node::number(0))),
                        Some(Node::less(i.clone(), Node::number(1))),
                        Some(Node::assign(
                            i.clone(),
                            Node::plus(i.clone(), Node::number(1),)
                        )),
                        Node::block(vec![
                            Node::assign(n.clone(), Node::plus(n.clone(), i.clone())),
                            Node::assign(m.clone(), Node::plus(m.clone(), i.clone())),
                        ]),
                    ),
                    Node::if_n(
                        Node::equal(Node::number(1), Node::number(1)),
                        Node::plus(a.clone(), b.clone()),
                        None
                    ),
                    Node::while_n(
                        Node::equal(Node::number(1), Node::number(1)),
                        Node::plus(a.clone(), b.clone()),
                    ),
                ],
                &[] as &[Token]
            ))
        );
    }
}
