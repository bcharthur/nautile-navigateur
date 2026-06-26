use crate::expr::Expr;
use crate::decl::Decl;
use crate::pat::Pat;
use crate::span::Span;

pub type Stmt = Box<StmtKind>;

#[derive(Debug, Clone)]
pub enum StmtKind {
    Block(BlockStmt),
    Empty(Span),
    Expr(ExprStmt),
    If(IfStmt),
    While(WhileStmt),
    DoWhile(DoWhileStmt),
    For(ForStmt),
    ForIn(ForInStmt),
    ForOf(ForOfStmt),
    Return(ReturnStmt),
    Throw(ThrowStmt),
    Try(TryStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Switch(SwitchStmt),
    Label(LabeledStmt),
    Decl(Decl),
    With(WithStmt),
    Debugger(Span),
}

#[derive(Debug, Clone)]
pub struct BlockStmt { pub body: Vec<Stmt>, pub span: Span }
#[derive(Debug, Clone)]
pub struct ExprStmt { pub expr: Expr, pub span: Span }
#[derive(Debug, Clone)]
pub struct IfStmt { pub test: Expr, pub consequent: Stmt, pub alternate: Option<Stmt>, pub span: Span }
#[derive(Debug, Clone)]
pub struct WhileStmt { pub test: Expr, pub body: Stmt, pub span: Span }
#[derive(Debug, Clone)]
pub struct DoWhileStmt { pub body: Stmt, pub test: Expr, pub span: Span }
#[derive(Debug, Clone)]
pub struct ForStmt { pub init: Option<ForInit>, pub test: Option<Expr>, pub update: Option<Expr>, pub body: Stmt, pub span: Span }
#[derive(Debug, Clone)]
pub enum ForInit { Decl(Decl), Expr(Expr) }
#[derive(Debug, Clone)]
pub struct ForInStmt { pub left: ForHead, pub right: Expr, pub body: Stmt, pub span: Span }
#[derive(Debug, Clone)]
pub struct ForOfStmt { pub is_await: bool, pub left: ForHead, pub right: Expr, pub body: Stmt, pub span: Span }
#[derive(Debug, Clone)]
pub enum ForHead { Decl(Decl), Pat(Pat) }
#[derive(Debug, Clone)]
pub struct ReturnStmt { pub arg: Option<Expr>, pub span: Span }
#[derive(Debug, Clone)]
pub struct ThrowStmt { pub arg: Expr, pub span: Span }
#[derive(Debug, Clone)]
pub struct TryStmt { pub block: BlockStmt, pub handler: Option<CatchClause>, pub finalizer: Option<BlockStmt>, pub span: Span }
#[derive(Debug, Clone)]
pub struct CatchClause { pub param: Option<Pat>, pub body: BlockStmt, pub span: Span }
#[derive(Debug, Clone)]
pub struct BreakStmt { pub label: Option<String>, pub span: Span }
#[derive(Debug, Clone)]
pub struct ContinueStmt { pub label: Option<String>, pub span: Span }
#[derive(Debug, Clone)]
pub struct SwitchStmt { pub discriminant: Expr, pub cases: Vec<SwitchCase>, pub span: Span }
#[derive(Debug, Clone)]
pub struct SwitchCase { pub test: Option<Expr>, pub consequent: Vec<Stmt>, pub span: Span }
#[derive(Debug, Clone)]
pub struct LabeledStmt { pub label: String, pub body: Stmt, pub span: Span }
#[derive(Debug, Clone)]
pub struct WithStmt { pub object: Expr, pub body: Stmt, pub span: Span }
