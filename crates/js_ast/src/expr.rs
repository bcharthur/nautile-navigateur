use crate::lit::Lit;
use crate::span::Span;

pub type Expr = Box<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Lit(Lit),
    Ident(Ident),
    This(Span),
    Array(ArrayExpr),
    Object(ObjectExpr),
    Function(FnExpr),
    Arrow(ArrowExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Logical(LogicalExpr),
    Assign(AssignExpr),
    Member(MemberExpr),
    Call(CallExpr),
    New(NewExpr),
    Sequence(SequenceExpr),
    Conditional(ConditionalExpr),
    Await(AwaitExpr),
    Yield(YieldExpr),
    Spread(SpreadExpr),
    TaggedTemplate(TaggedTemplateExpr),
    OptionalChain(OptionalChainExpr),
    Class(ClassExpr),
    MetaProp(MetaPropExpr),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Expr,
    pub right: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Rem, Exp,
    Eq, NotEq, StrictEq, StrictNotEq,
    Lt, Lte, Gt, Gte,
    BitAnd, BitOr, BitXor, Shl, Shr, UShr,
    In, Instanceof,
    NullishCoalesce,
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub op: AssignOp,
    pub left: AssignTarget,
    pub right: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssignOp {
    Assign,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, ExpAssign,
    BitAndAssign, BitOrAssign, BitXorAssign,
    ShlAssign, ShrAssign, UShrAssign,
    AndAssign, OrAssign, NullishAssign,
}

#[derive(Debug, Clone)]
pub enum AssignTarget {
    Ident(Ident),
    Member(MemberExpr),
    Pattern(crate::pat::Pat),
}

#[derive(Debug, Clone)]
pub struct MemberExpr {
    pub object: Expr,
    pub property: MemberProp,
    pub computed: bool,
    pub optional: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum MemberProp {
    Ident(Ident),
    Computed(Expr),
    PrivateName(String),
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Callee,
    pub args: Vec<Argument>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Callee {
    Expr(Expr),
    Super(Span),
    Import(Span),
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub spread: bool,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct NewExpr {
    pub callee: Expr,
    pub args: Vec<Argument>,
    pub span: Span,
}

#[derive(Debug, Clone)] pub struct ArrayExpr { pub elements: Vec<Option<Argument>>, pub span: Span }
#[derive(Debug, Clone)] pub struct ObjectExpr { pub props: Vec<ObjectProp>, pub span: Span }
#[derive(Debug, Clone)] pub struct FnExpr { pub id: Option<Ident>, pub params: Vec<crate::pat::Pat>, pub body: crate::stmt::BlockStmt, pub is_async: bool, pub is_generator: bool, pub span: Span }
#[derive(Debug, Clone)] pub struct ArrowExpr { pub params: Vec<crate::pat::Pat>, pub body: ArrowBody, pub is_async: bool, pub span: Span }
#[derive(Debug, Clone)] pub enum ArrowBody { Block(crate::stmt::BlockStmt), Expr(Expr) }
#[derive(Debug, Clone)] pub struct UnaryExpr { pub op: UnaryOp, pub arg: Expr, pub prefix: bool, pub span: Span }
#[derive(Debug, Clone, Copy)] pub enum UnaryOp { Not, BitNot, Pos, Neg, Typeof, Void, Delete, Throw }
#[derive(Debug, Clone)] pub struct LogicalExpr { pub op: LogicalOp, pub left: Expr, pub right: Expr, pub span: Span }
#[derive(Debug, Clone, Copy)] pub enum LogicalOp { And, Or, Nullish }
#[derive(Debug, Clone)] pub struct SequenceExpr { pub exprs: Vec<Expr>, pub span: Span }
#[derive(Debug, Clone)] pub struct ConditionalExpr { pub test: Expr, pub consequent: Expr, pub alternate: Expr, pub span: Span }
#[derive(Debug, Clone)] pub struct AwaitExpr { pub arg: Expr, pub span: Span }
#[derive(Debug, Clone)] pub struct YieldExpr { pub arg: Option<Expr>, pub delegate: bool, pub span: Span }
#[derive(Debug, Clone)] pub struct SpreadExpr { pub arg: Expr, pub span: Span }
#[derive(Debug, Clone)] pub struct TaggedTemplateExpr { pub tag: Expr, pub quasi: crate::lit::TemplateLit, pub span: Span }
#[derive(Debug, Clone)] pub struct OptionalChainExpr { pub base: Expr, pub span: Span }
#[derive(Debug, Clone)] pub struct ClassExpr { pub id: Option<Ident>, pub super_class: Option<Expr>, pub body: Vec<ClassMember>, pub span: Span }
#[derive(Debug, Clone)] pub struct MetaPropExpr { pub meta: String, pub property: String, pub span: Span }
#[derive(Debug, Clone)] pub enum ObjectProp { KeyValue(ObjectKeyValue), Shorthand(Ident), Method(ObjectMethod), Spread(SpreadExpr) }
#[derive(Debug, Clone)] pub struct ObjectKeyValue { pub key: PropKey, pub value: Expr }
#[derive(Debug, Clone)] pub enum PropKey { Ident(Ident), Str(String), Num(f64), Computed(Expr) }
#[derive(Debug, Clone)] pub struct ObjectMethod { pub key: PropKey, pub params: Vec<crate::pat::Pat>, pub body: crate::stmt::BlockStmt, pub kind: MethodKind, pub is_async: bool, pub is_generator: bool }
#[derive(Debug, Clone, Copy)] pub enum MethodKind { Method, Get, Set, Constructor }
#[derive(Debug, Clone)] pub enum ClassMember { Method(ClassMethod), Field(ClassField), StaticBlock(crate::stmt::BlockStmt) }
#[derive(Debug, Clone)] pub struct ClassMethod { pub key: PropKey, pub params: Vec<crate::pat::Pat>, pub body: crate::stmt::BlockStmt, pub kind: MethodKind, pub is_static: bool, pub is_async: bool, pub is_generator: bool, pub is_private: bool }
#[derive(Debug, Clone)] pub struct ClassField { pub key: PropKey, pub value: Option<Expr>, pub is_static: bool, pub is_private: bool }
