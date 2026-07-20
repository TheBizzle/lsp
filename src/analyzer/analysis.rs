use std::collections::{HashMap, HashSet};

use crate::errors::AnalyzerError;

use crate::lexer::token::Token;

use crate::analyzer::address::{NamedVarAddress, ScopeAddress};
use crate::analyzer::builtins::{INITIAL_SCOPE_ADDRESS, initial_state};
use crate::analyzer::organic_type::OrganicType;
use crate::analyzer::scope::{Env, Scope};

pub struct Analysis<'a> {
  pub definitions: HashMap<NamedVarAddress, Token>,
  pub errors: Vec<AnalyzerError<'a>>,
  pub usages: HashMap<NamedVarAddress, HashSet<Token>>,
}

#[allow(clippy::derivable_impls)] // TODO
impl Default for Analysis<'_> {
  fn default() -> Self {
    Self { definitions: HashMap::new(), errors: Vec::new(), usages: HashMap::new() }
  }
}

pub struct AnalysisState<'a> {
  pub analysis: Analysis<'a>,
  pub last_scope_addr: &'a ScopeAddress,
  pub scopes: Vec<Scope<'a>>,
  pub vars: HashMap<NamedVarAddress, OrganicType<'a>>,
}

impl Default for AnalysisState<'_> {
  fn default() -> Self {
    let (bindings, vars) = initial_state();
    Self {
      analysis: Analysis::default(),
      last_scope_addr: INITIAL_SCOPE_ADDRESS,
      scopes: vec![Scope { env: Env { bindings }, address: INITIAL_SCOPE_ADDRESS }],
      vars,
    }
  }
}
