use crate::lexer::token::Token;

use crate::errors::AnalyzerError;
use crate::errors::AnalyzerErrorType;

use crate::analyzer::address::NamedVarAddress;
use crate::analyzer::analysis::AnalysisState;
use crate::analyzer::organic_type::OrganicType as OT;

pub(super) fn resolve_addr(state: &AnalysisState, name: &str) -> Option<NamedVarAddress> {
  for scope in &state.scopes {
    if let Some(addr) = scope.env.bindings.get(name) {
      return Some(addr.clone());
    }
  }
  None
}

pub(super) fn resolve_type<'a>(state: &AnalysisState<'a>, addr: &NamedVarAddress) -> OT<'a> {
  state
    .vars
    .get(addr)
    .map_or_else(|| panic!("Invalid LSP state!  Known `{addr:?}` lacked a binding!"), Clone::clone)
}

pub(super) fn push_error<'a>(state: &mut AnalysisState<'a>, token: Token, typ: AnalyzerErrorType<'a>) {
  state.analysis.errors.push(AnalyzerError { typ, offender: token });
}
