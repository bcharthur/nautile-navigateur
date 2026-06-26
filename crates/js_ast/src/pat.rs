use crate::expr::{Expr, Ident};
use crate::span::Span;

#[derive(Debug, Clone)]
pub enum Pat {
    Ident(Ident),
    Array(ArrayPat),
    Object(ObjectPat),
    Rest(RestPat),
    Assign(AssignPat),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct ArrayPat { pub elements: Vec<Option<Pat>>, pub span: Span }
#[derive(Debug, Clone)]
pub struct ObjectPat { pub props: Vec<ObjectPatProp>, pub span: Span }
#[derive(Debug, Clone)]
pub enum ObjectPatProp {
    KeyValue { key: crate::expr::PropKey, value: Pat },
    Assign { key: Ident, default: Option<Expr> },
    Rest(RestPat),
}
#[derive(Debug, Clone)]
pub struct RestPat { pub arg: Box<Pat>, pub span: Span }
#[derive(Debug, Clone)]
pub struct AssignPat { pub left: Box<Pat>, pub right: Expr, pub span: Span }
