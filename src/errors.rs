use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::analyzer::organic_type::OrganicType;
use crate::lexer::doc_loc::DocLoc;
use crate::lexer::source_loc::{MiniLoc, SourceLoc};
use crate::lexer::token::Token;
use crate::parser::ast::Symbol;

#[derive(Debug)]
pub enum LspError<'a> {
  LspLexerError(LexerError),
  LspParserError(ParserError),
  LspAnalyzerError(AnalyzerError<'a>),
}

#[derive(Debug)]
pub enum LexerError {
  FileTooBig { size: usize, line_num: u32 },
  UnknownToken { culprit: String, source_loc: SourceLoc },
}

#[derive(Debug)]
pub enum ParserError {
  ExtraToken { token: Token },
  FictionalToken { location: MiniLoc },
  UnexpectedEOF { location: MiniLoc, expected: Vec<String> },
  WrongToken { token: Token, expected: Vec<String> },
}

#[derive(Debug)]
pub struct AnalyzerError<'a> {
  pub typ: AnalyzerErrorType<'a>,
  pub offender: Token,
}

#[derive(Debug)]
pub enum AnalyzerErrorType<'a> {
  ArityMismatch { fn_name: Symbol, expected: u32, got: u32 },
  BadInternalState,
  DuplicateVar,
  NoSuchFn,
  NoSuchVariable,
  TypeMismatch { expected: OrganicType<'a>, got: OrganicType<'a> },
  VarCannotInitInTermsOfSelf,
}

use AnalyzerErrorType::{
  ArityMismatch, BadInternalState, DuplicateVar, NoSuchFn, NoSuchVariable, TypeMismatch,
  VarCannotInitInTermsOfSelf,
};
use LexerError::{FileTooBig, UnknownToken};
use LspError::{LspAnalyzerError, LspLexerError, LspParserError};
use ParserError::{ExtraToken, FictionalToken, UnexpectedEOF, WrongToken};

#[must_use]
pub fn as_diagnostic(error: LspError) -> Diagnostic {
  let (range, message) = match error {
    LspLexerError(FileTooBig { size, line_num }) => {
      let range = Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: line_num - 1, character: 0 },
      };
      let msg = format!(
        "This file is too large!  organic-lsp can only handle values up to {} characters in size, but this file has at least {size} characters",
        u32::MAX
      );
      (range, msg)
    },
    LspLexerError(UnknownToken { culprit, source_loc }) => {
      (as_range(&source_loc), format!("Unknown token: {culprit}"))
    },

    LspParserError(ExtraToken { token }) => {
      (as_range(&token.source_loc), format!("Token found after EOF: {:?}", token.token_type))
    },
    LspParserError(FictionalToken { location }) => {
      (as_range_mini(location), format!("Unparseable token type at location: {location:?}"))
    },
    LspParserError(UnexpectedEOF { location, expected }) => {
      (as_range_mini(location), format!("Unexpected EOF at location {location:?}\nExpected: ${expected:?}"))
    },
    LspParserError(WrongToken { token, expected }) => {
      let msg = format!("Wrong token for this context: {token:?}\nExpected: ${expected:?}");
      (as_range(&token.source_loc), msg)
    },

    LspAnalyzerError(AnalyzerError { typ: ArityMismatch { fn_name, expected, got }, offender }) => {
      let msg = format!("Function `{}` takes {expected} arguments, but got {got}.", fn_name.name);
      (as_range(&offender.source_loc), msg)
    },
    LspAnalyzerError(AnalyzerError { typ: BadInternalState, offender }) => {
      let msg = format!("Fatal internal error on `{:?}`", offender.token_type);
      (as_range(&offender.source_loc), msg)
    },
    LspAnalyzerError(AnalyzerError { typ: DuplicateVar, offender }) => {
      let msg = format!("Duplicate variable: {:?}", offender.token_type);
      (as_range(&offender.source_loc), msg)
    },
    LspAnalyzerError(AnalyzerError { typ: NoSuchFn, offender }) => {
      let msg = format!("No such function: {:?}", offender.token_type);
      (as_range(&offender.source_loc), msg)
    },
    LspAnalyzerError(AnalyzerError { typ: NoSuchVariable, offender }) => {
      let msg = format!("No such variable: {:?}", offender.token_type);
      (as_range(&offender.source_loc), msg)
    },
    LspAnalyzerError(AnalyzerError { typ: TypeMismatch { expected, got }, offender }) => {
      let msg = format!(
        "Could not match expected type `{expected:?}` with actual type `{got:?}`, regarding value `{:?}`.",
        offender.token_type
      );
      (as_range(&offender.source_loc), msg)
    },
    LspAnalyzerError(AnalyzerError { typ: VarCannotInitInTermsOfSelf, offender }) => {
      let msg = format!("`{:?}` cannot be defined in terms of itself", offender.token_type);
      (as_range(&offender.source_loc), msg)
    },
  };

  Diagnostic { range, severity: Some(DiagnosticSeverity::ERROR), message, ..Default::default() }
}

fn as_range_mini(mini: MiniLoc) -> Range {
  as_range(&SourceLoc { line: mini.line, column: mini.column, length: 1, pos: 0, doc_loc: DocLoc::new("") })
}

const fn as_range(source_loc: &SourceLoc) -> Range {
  let &SourceLoc { line, column, length, .. } = source_loc;
  Range {
    start: Position { line: line - 1, character: column - 1 },
    end: Position { line: line - 1, character: column - 1 + length },
  }
}
