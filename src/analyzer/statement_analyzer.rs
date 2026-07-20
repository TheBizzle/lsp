use std::collections::HashSet;

use crate::parser::ast::{Statement, VarDecl};

use crate::errors::AnalyzerErrorType::DuplicateVar;

use crate::analyzer::address::NamedVarAddress;
use crate::analyzer::analysis::AnalysisState;
use crate::analyzer::common::push_error;
use crate::analyzer::expr_analyzer::{crawl_expr, crawl_function_call};

pub fn run(state: &mut AnalysisState, statements: Vec<Statement>) {
  for statement in statements {
    crawl_statement(state, statement);
  }
}

fn crawl_statement(state: &mut AnalysisState, statement: Statement) {
  match statement {
    Statement::FunctionCall(fn_call) => {
      crawl_function_call(state, *fn_call);
    },
    Statement::Include(_path) => todo!("Import foreign terms into namespace"),
    Statement::VariableDecl(var_decl) => crawl_var_decl(state, var_decl),
  }
}

fn crawl_var_decl(state: &mut AnalysisState, var_decl: VarDecl) {
  let my_addr =
    NamedVarAddress { name: var_decl.name.name.clone(), scope_addr: state.last_scope_addr.clone() };

  if state.scopes[0].env.bindings.contains_key(var_decl.name.name.as_str()) {
    push_error(state, var_decl.name.token, DuplicateVar);
  } else {
    state.analysis.definitions.insert(my_addr.clone(), var_decl.name.token);
    state.analysis.usages.insert(my_addr.clone(), HashSet::default());
  }

  if let Some(typ) = crawl_expr(state, var_decl.init) {
    state.vars.insert(my_addr, typ);
  }
}
