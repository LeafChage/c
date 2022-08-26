/// program    = stmt*
/// stmt       = expr ";"
///                 | "return" expr ";"
///                 | "if" "(" expr ")" stmt ("else" stmt)?
///                 | "while" "(" expr ")" stmt
///                 | "for" "(" expr? ";" expr? ";" expr? ")" stmt
/// expr       = assign
/// assign     = equality ("=" assign)?
/// equality   = relational ("==" relational | "!=" relational)*
/// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
/// add        = mul ("+" mul | "-" mul)*
/// mul        = unary ("*" unary | "/" unary)*
/// unary      = ("+" | "-")? primary
/// primary    = num | ident | "(" expr ")"
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Node {
    Number(isize),

    /// id, offset from RBP
    LocalVariable(String, usize),

    /// left == right
    Equal(Box<Node>, Box<Node>),

    /// left != right
    UnEqual(Box<Node>, Box<Node>),

    /// left < right
    Less(Box<Node>, Box<Node>),

    /// left <= right
    LessEqual(Box<Node>, Box<Node>),

    /// left + right
    Plus(Box<Node>, Box<Node>),

    /// left - right
    Minus(Box<Node>, Box<Node>),

    /// left * right
    Multiple(Box<Node>, Box<Node>),

    /// left / right
    Devide(Box<Node>, Box<Node>),

    Assign(Box<Node>, Box<Node>),

    Return(Box<Node>),

    /// ```
    /// if 1==1
    ///     print 3
    /// else
    ///     print 4
    /// ```
    /// If(1==1, print 3, print 4)
    If(Box<Node>, Box<Node>, Option<Box<Node>>),

    /// ```
    /// for (i=0; i<10; i++)
    ///     print 1
    /// ```
    /// For(i=0, i<10, i++, print 1)
    For(
        Option<Box<Node>>,
        Option<Box<Node>>,
        Option<Box<Node>>,
        Box<Node>,
    ),

    /// ```
    /// while (true)
    ///     print 1
    /// ```
    /// While(true, print 1)
    While(Box<Node>, Box<Node>),
}

impl Node {
    pub fn number(n: isize) -> Self {
        Node::Number(n)
    }
    pub fn local_variable<S>(n: S, offset: usize) -> Self
    where
        S: Into<String>,
    {
        Node::LocalVariable(n.into(), offset)
    }
    pub fn equal(left: Self, right: Self) -> Self {
        Node::Equal(Box::new(left), Box::new(right))
    }
    pub fn unequal(left: Self, right: Self) -> Self {
        Node::UnEqual(Box::new(left), Box::new(right))
    }
    pub fn less(left: Self, right: Self) -> Self {
        Node::Less(Box::new(left), Box::new(right))
    }
    pub fn less_equal(left: Self, right: Self) -> Self {
        Node::LessEqual(Box::new(left), Box::new(right))
    }
    pub fn plus(left: Self, right: Self) -> Self {
        Node::Plus(Box::new(left), Box::new(right))
    }
    pub fn minus(left: Self, right: Self) -> Self {
        Node::Minus(Box::new(left), Box::new(right))
    }
    pub fn multiple(left: Self, right: Self) -> Self {
        Node::Multiple(Box::new(left), Box::new(right))
    }
    pub fn devide(left: Self, right: Self) -> Self {
        Node::Devide(Box::new(left), Box::new(right))
    }
    pub fn assign(left: Self, right: Self) -> Self {
        Node::Assign(Box::new(left), Box::new(right))
    }
    pub fn return_n(node: Self) -> Self {
        Node::Return(Box::new(node))
    }

    pub fn if_n(condition: Self, true_action: Self, false_action: Option<Self>) -> Self {
        Node::If(
            Box::new(condition),
            Box::new(true_action),
            false_action.map(|fa| Box::new(fa)),
        )
    }

    pub fn for_n(
        condition1: Option<Self>,
        condition2: Option<Self>,
        condition3: Option<Self>,
        node: Self,
    ) -> Self {
        Node::For(
            condition1.map(|c| Box::new(c)),
            condition2.map(|c| Box::new(c)),
            condition3.map(|c| Box::new(c)),
            Box::new(node),
        )
    }

    pub fn while_n(condition: Self, node: Self) -> Self {
        Node::While(Box::new(condition), Box::new(node))
    }
}
