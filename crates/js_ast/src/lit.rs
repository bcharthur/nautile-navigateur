use crate::span::Span;

#[derive(Debug, Clone)]
pub enum Lit {
    Null(Span),
    Bool(bool, Span),
    Number(f64, Span),
    Str(String, Span),
    BigInt(String, Span),
    Regex(String, String, Span),
    Template(TemplateLit),
}

#[derive(Debug, Clone)]
pub struct TemplateLit {
    pub quasis: Vec<TemplateElement>,
    pub exprs: Vec<crate::expr::Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TemplateElement {
    pub raw: String,
    pub cooked: Option<String>,
    pub tail: bool,
    pub span: Span,
}
