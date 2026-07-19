use crate::lexer::token::Token;
use crate::lexer::token::TokenType::Identifier;
use crate::parser::ast::{Arg, Expr, Formal, Symbol};

#[derive(Debug)]
pub struct Param {
  pub symbol: Symbol,
  pub value_option: Option<Expr>,
}

pub fn as_args(params: Vec<Param>) -> Vec<Arg> {
  params.into_iter().map(as_arg).collect()
}

fn as_arg(param: Param) -> Arg {
  if let Some(value) = param.value_option {
    Arg { name: param.symbol, value }
  } else {
    panic!("Cannot convert `Param` {param:?} to argument, since it lacks an argument value!")
  }
}

pub fn as_formals(params: Vec<Param>) -> Vec<Formal> {
  params.into_iter().map(as_formal).collect()
}

fn as_formal(param: Param) -> Formal {
  Formal { name: param.symbol, default: param.value_option }
}

/// # Panics
/// When given a token that isn't an identifier
#[must_use]
pub fn as_symbol(ident: &Token, descriptor: &str) -> Symbol {
  match &ident.token_type {
    Identifier(name) => Symbol { name: name.clone(), token: ident.clone() },
    x => panic!("Impossible {descriptor}: {x:?}"),
  }
}

pub fn sequence<T>(head: T, tail: Vec<T>) -> Vec<T> {
  std::iter::once(head).chain(tail).collect()
}
