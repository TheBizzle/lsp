use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::lexer::doc_loc::DocLoc;
use crate::lexer::source_loc::{MiniLoc, SourceLoc};
use crate::lexer::token::Token;
use crate::token::Token;

#[derive(Debug)]
pub enum LspError {
  LspLexerError(LexerError),
  LspParserError(ParserError),
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

use LexerError::{FileTooBig, UnknownToken};
use LspError::{LspLexerError, LspParserError};
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
