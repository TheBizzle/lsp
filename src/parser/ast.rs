use crate::lexer::token::Token;
use ordered_float::NotNan;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Symbol {
  pub name: String,
  pub token: Token,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
  Call { call: FuncCall, token: Token },
  Function { value: FuncLiteral, token: Token },
  Grouping { value: Box<Self>, token: Token },
  List { values: Vec<Self>, token: Token },
  LValue { name: Symbol, token: Token },
  Negated { value: Box<Self>, token: Token },
  Number { value: NotNan<f64>, token: Token },
  Op { left: Box<Self>, operator: Operator, right: Box<Self>, token: Token },
  String { value: String, token: Token },
}

#[derive(Debug, PartialEq)]
pub struct Arg {
  pub name: Symbol,
  pub value: Expr,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
  Plus,
  Minus,
  Times,
  Divide,
  LessThan,
  LessOrEquals,
  GreaterThan,
  GreaterOrEquals,
}

#[derive(Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
  FunctionCall(Box<FuncCall>),
  Include(String),
  VariableDecl(VarDecl),
}

#[derive(Debug, PartialEq)]
pub struct FuncCall {
  pub func: Symbol,
  pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq)]
pub struct FuncLiteral {
  pub name: Symbol,
  pub formals: Vec<Formal>,
  pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Formal {
  pub name: Symbol,
  pub default: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct VarDecl {
  pub name: Symbol,
  pub init: Expr,
}
