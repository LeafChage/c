use super::super::tokenizer::Token;
use super::Node;

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Expected(Vec<Token>),
}
pub type Result<T> = std::result::Result<T, Error>;

pub struct CParser {
    id_jar: HashMap<String, usize>,
}

impl CParser {
    pub fn new() -> Self {
        CParser {
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

    fn stmt<'a>(&mut self, tokens: &'a [Token]) -> Result<(Node, &'a [Token])> {
        match tokens {
            [Token::Return, tokens @ ..] => {
                let (node, tokens) = self.stmt(tokens)?;
                Ok((Node::rtn(node), tokens))
            }
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
    fn it_program() {
        let mut cparser = CParser::new();
        assert_eq!(
            cparser.program(
                &tokenize(
                    "
                a = 3;
                b = 5 * 6 - 8;
                return a + b / 2;
            "
                )[..]
            ),
            Ok((
                vec![
                    Node::assign(Node::local_variable("a", 0), Node::number(3)),
                    Node::assign(
                        Node::local_variable("b", 8),
                        Node::minus(
                            Node::multiple(Node::number(5), Node::number(6)),
                            Node::number(8),
                        )
                    ),
                    Node::rtn(Node::plus(
                        Node::local_variable("a", 0),
                        Node::devide(Node::local_variable("b", 8), Node::number(2))
                    )),
                ],
                &[] as &[Token]
            ))
        );
    }
}
