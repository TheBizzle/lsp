use lalrpop_util::ParseError as LALR;

pub mod ast;
pub mod lalrpop;

lalrpop_util::lalrpop_mod!(
  #[ allow( clippy::cast_sign_loss
          , clippy::cloned_instead_of_copied
          , clippy::default_trait_access
          , clippy::implicit_clone
          , clippy::match_same_arms
          , clippy::missing_const_for_fn
          , clippy::missing_errors_doc
          , clippy::must_use_candidate
          , clippy::option_if_let_else
          , clippy::redundant_pub_crate
          , clippy::too_many_lines
          , clippy::trivially_copy_pass_by_ref
          , clippy::unnecessary_wraps
          , clippy::unused_self
          , clippy::use_self
          )
  ]
  pub grammar, "/parser/grammar.rs"
);

use crate::errors::LspError;
use crate::errors::ParserError::{self, ExtraToken, FictionalToken, UnexpectedEOF, WrongToken};
use crate::lexer::doc_loc::DocLoc;
use crate::lexer::lex;
use crate::lexer::token::Token;
use crate::parser::ast::Statement;
use crate::parser::grammar::StatementParser;
use LspError::{LspLexerError, LspParserError};

pub fn analyze(doc_loc: &DocLoc, doc_text: &str) -> (Vec<Statement>, Vec<LspError>) {
  let (tokens, lerrors) = lex(doc_loc, doc_text);
  let lsp_lerrors = lerrors.into_iter().map(LspLexerError).collect();

  match parse(tokens) {
    Ok(statement) => (vec![statement], lsp_lerrors),
    Err(error) => {
      let mut lsp_all_errors = vec![LspParserError(error)];
      lsp_all_errors.extend(lsp_lerrors);
      (Vec::new(), lsp_all_errors)
    },
  }
}

fn parse(tokens: Vec<Token>) -> Result<Statement, ParserError> {
  let parser = StatementParser::new();
  let triples: Vec<_> = tokens
    .into_iter()
    .map(|t| {
      let (start, end) = t.source_loc.as_minis();
      (start, t, end)
    })
    .collect();

  parser.parse(triples).map_err(|err| match err {
    LALR::InvalidToken { location } => FictionalToken { location },
    LALR::UnrecognizedToken { token: (_start, token, _end), expected } => WrongToken { token, expected },
    LALR::UnrecognizedEof { location, expected } => UnexpectedEOF { location, expected },
    LALR::User { error } => error,
    LALR::ExtraToken { token: (_start, token, _end) } => ExtraToken { token },
  })
}
