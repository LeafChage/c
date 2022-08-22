#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

/// program    = stmt*
/// stmt       = expr ";"
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

    Identity(String),

    /// + right / - right
    Unary(Sign, Box<Node>),

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

    Assign(Box<Node>, Option<Box<Node>>),
}

impl Node {
    pub fn number(n: isize) -> Self {
        Node::Number(n)
    }
    pub fn identity<S>(n: S) -> Self
    where
        S: Into<String>,
    {
        Node::Identity(n.into())
    }
    pub fn unary(left: Sign, right: Self) -> Self {
        Node::Unary(left, Box::new(right))
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
    pub fn assign(left: Self, right: Option<Self>) -> Self {
        Node::Assign(Box::new(left), right.map(|r| Box::new(r)))
    }
}
