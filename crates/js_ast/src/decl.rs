use crate::expr::Expr;
use crate::pat::Pat;
use crate::span::Span;

#[derive(Debug, Clone)]
pub enum Decl {
    Var(VarDecl),
    Fn(FnDecl),
    Class(ClassDecl),
    Import(ImportDecl),
    Export(ExportDecl),
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub kind: VarKind,
    pub decls: Vec<VarDeclarator>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarKind { Var, Let, Const }

#[derive(Debug, Clone)]
pub struct VarDeclarator { pub id: Pat, pub init: Option<Expr>, pub span: Span }

#[derive(Debug, Clone)]
pub struct FnDecl { pub id: crate::expr::Ident, pub params: Vec<Pat>, pub body: crate::stmt::BlockStmt, pub is_async: bool, pub is_generator: bool, pub span: Span }

#[derive(Debug, Clone)]
pub struct ClassDecl { pub id: crate::expr::Ident, pub super_class: Option<Expr>, pub body: Vec<crate::expr::ClassMember>, pub span: Span }

#[derive(Debug, Clone)]
pub struct ImportDecl { pub specifiers: Vec<ImportSpecifier>, pub source: String, pub span: Span }

#[derive(Debug, Clone)]
pub enum ImportSpecifier {
    Default(crate::expr::Ident),
    Namespace(crate::expr::Ident),
    Named { imported: Option<String>, local: crate::expr::Ident },
}

#[derive(Debug, Clone)]
pub struct ExportDecl { pub kind: ExportKind, pub span: Span }

#[derive(Debug, Clone)]
pub enum ExportKind {
    Default(Expr),
    Named(Vec<ExportSpecifier>),
    All { source: Option<String> },
    Decl(Box<Decl>),
}

#[derive(Debug, Clone)]
pub struct ExportSpecifier { pub local: crate::expr::Ident, pub exported: Option<String> }
