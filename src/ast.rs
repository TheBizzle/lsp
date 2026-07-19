use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct Symbol {
  pub value: String,
  pub token: Token,
}

#[derive(Debug, PartialEq)]
pub enum LValue {
  Variable { symbol: Symbol },
}

#[derive(Debug, PartialEq)]
pub enum Expr {
  Array { values: Vec<Self>, token: Token },
  Call { call: FuncCall, token: Token },
  Grouping { value: Box<Self>, token: Token },
  LValue { lvalue: LValue, token: Token },
  Negated { value: Box<Self>, token: Token },
  Number { value: f64, token: Token },
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
  FunctionDecl(FuncDecl),
  VariableDecl(VarDecl),
}

#[derive(Debug, PartialEq)]
pub struct FuncCall {
  pub func: Symbol,
  pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq)]
pub struct FuncDecl {
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
