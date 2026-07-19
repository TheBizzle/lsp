use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::lexer::source_loc::SourceLoc;
use crate::lexer::token::Token;

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
  SomeParserErrorIDK { culprit: Token },
}

use LexerError::{FileTooBig, UnknownToken};
use LspError::{LspLexerError, LspParserError};
use ParserError::SomeParserErrorIDK;

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
      (as_range(&source_loc), format!("Unknown token: {culprit}")) // TODO
    },
    LspParserError(SomeParserErrorIDK { culprit: Token { token_type: _, source_loc } }) => {
      (as_range(&source_loc), "Just some random parser error, IDK".to_string()) // TODO
    },
  };

  Diagnostic { range, severity: Some(DiagnosticSeverity::ERROR), message, ..Default::default() }
}

const fn as_range(source_loc: &SourceLoc) -> Range {
  let &SourceLoc { line, column, length, .. } = source_loc;
  Range {
    start: Position { line: line - 1, character: column - 1 },
    end: Position { line: line - 1, character: column - 1 + length },
  }
}
