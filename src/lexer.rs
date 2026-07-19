pub mod doc_loc;
pub mod source_loc;
pub mod token;

use std::ops::Range;

use logos::Logos;

use crate::errors::LexerError;
use crate::lexer::doc_loc::DocLoc;
use crate::lexer::source_loc::SourceLoc;
use crate::lexer::token::{Token, TokenType};

#[must_use]
pub fn lex(doc_loc: &DocLoc, doc_text: &str) -> (Vec<Token>, Vec<LexerError>) {
  let mut line_num: u32 = 1;
  let mut last_line_offset: u32 = 0;
  let mut errors: Vec<LexerError> = Vec::new();
  let mut token_sequence = Vec::new();

  for (result, span) in TokenType::lexer(doc_text).spanned() {
    match as_u32s(span.clone()) {
      Err(size) => {
        errors.push(LexerError::FileTooBig { size, line_num });
        return (token_sequence, errors);
      },
      Ok((start, length, end)) => {
        let column = 1 + start - last_line_offset;
        let source_loc = SourceLoc { doc_loc: doc_loc.clone(), pos: start, line: line_num, column, length };
        if let Ok(token_type) = result {
          if token_type == TokenType::Newline {
            line_num += 1;
            last_line_offset = end;
          } else {
            token_sequence.push(Token { token_type, source_loc });
          }
        } else {
          let error = LexerError::UnknownToken { culprit: doc_text[span].to_string(), source_loc };
          errors.push(error);
        }
      },
    }
  }

  (token_sequence, errors)
}

fn as_u32s(span: Range<usize>) -> Result<(u32, u32, u32), usize> {
  let start = u32::try_from(span.start).map_err(|_| span.start)?;
  let len = u32::try_from(span.len()).map_err(|_| span.len())?;
  let end = u32::try_from(span.end).map_err(|_| span.end)?;
  Ok((start, len, end))
}
