use std::borrow::Cow;
use std::sync::Arc;

use crate::parser::ast::{Arg, Expr, FuncCall, Symbol};

use crate::errors::AnalyzerErrorType::{NoSuchFn, TypeMismatch};

use crate::analyzer::analysis::AnalysisState;
use crate::analyzer::common::{push_error, resolve_addr, resolve_type};
use crate::analyzer::function::{Function, ParamInfo};
use crate::analyzer::organic_type::OrganicType as OT;

pub(super) fn crawl_expr<'a>(_state: &mut AnalysisState, _expr: Expr) -> Option<OT<'a>> {
  todo!("")
}

pub(super) fn crawl_function_call<'a>(state: &mut AnalysisState<'a>, fn_call: FuncCall) -> Option<OT<'a>> {
  let FuncCall { func: Symbol { name, token }, args } = fn_call;

  match resolve_addr(state, &name) {
    None => {
      push_error(state, token, NoSuchFn);
      None
    },

    Some(addr) => match resolve_type(state, &addr) {
      OT::Function(func) => {
        state.analysis.usages.entry(addr).or_default().insert(token);
        Some(func.return_type.clone())
      },

      got => {
        let expected_formals = args
          .into_iter()
          .map(|Arg { name, value }| {
            ParamInfo(Cow::Owned(name.name), crawl_expr(state, value).unwrap_or(OT::Unknown), false)
          })
          .collect();

        let func = Function { params: expected_formals, return_type: OT::Unknown };
        let expected = OT::Function(Arc::new(func));

        push_error(state, token, TypeMismatch { expected, got });
        None
      },
    },
  }
}

//  pub enum Expr {
//    Call { call: FuncCall, token: Token },
//    Function { value: FuncLiteral, token: Token },
//    Grouping { value: Box<Self>, token: Token },
//    List { values: Vec<Self>, token: Token },
//    LValue { name: Symbol, token: Token },
//    Negated { value: Box<Self>, token: Token },
//    Number { value: NotNan<f64>, token: Token },
//    Op { left: Box<Self>, operator: Operator, right: Box<Self>, token: Token },
//    String { value: String, token: Token },
//  }
//
//  pub enum Operator {
//    Plus,
//    Minus,
//    Times,
//    Divide,
//    LessThan,
//    LessOrEquals,
//    GreaterThan,
//    GreaterOrEquals,
//  }
//
//  pub struct FuncLiteral {
//    pub name: Symbol,
//    pub formals: Vec<Formal>,
//    pub body: Vec<Statement>,
//  }
//
//  #[derive(Debug, PartialEq)]
//  pub struct Formal {
//    pub name: Symbol,
//    pub default: Option<Expr>,
//  }
