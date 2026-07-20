pub mod address;
pub mod analysis;
pub mod builtins;
pub mod common;
pub mod expr_analyzer;
pub mod function;
pub mod organic_type;
pub mod scope;
pub mod statement_analyzer;

use crate::parser::ast::Statement;

use analysis::{Analysis, AnalysisState};

#[must_use]
pub fn analyze<'a>(statements: Vec<Statement>) -> Analysis<'a> {
  let mut state = AnalysisState::default();
  statement_analyzer::run(&mut state, statements);
  state.analysis
}
