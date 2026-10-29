use crate::token::Span;
use crate::types::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Integer(String),
    Float(String),
    Bool(bool),
    Negate(Box<Expr>),
    Call {
        name: String,
        arguments: Vec<Expr>,
    },
    Array(Vec<Expr>),
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl BinaryOperator {
    pub fn is_comparison(self) -> bool {
        matches!(
            self,
            Self::Equal
                | Self::NotEqual
                | Self::Less
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtKind {
    /// Declares a variable using `assign name value`.
    Set {
        name: String,
        value: Expr,
    },
    Show(Expr),
    Assign {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    Return(Expr),
}
