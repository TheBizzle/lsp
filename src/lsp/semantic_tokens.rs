use tower_lsp_server::ls_types::{SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensResult};

use crate::lexer::doc_loc::DocLoc;
use crate::lexer::lex;
use crate::lexer::source_loc::SourceLoc;
use crate::lexer::token::Token;
use crate::lexer::token::TokenType::{
  Colon, Comma, Divide, Equals, GreaterThan, GreaterThanEquals, Identifier, Include, LeftBrace, LeftBracket,
  LeftParen, LessThan, LessThanEquals, Minus, Multiply, Newline, Number, Plus, RightBrace, RightBracket,
  RightParen, String, Whitespace,
};

const LANGUAGE_CONSTANT: SemanticTokenType = SemanticTokenType::new("language_constant");

pub const TOKEN_TYPES: &[SemanticTokenType] = &[
  SemanticTokenType::FUNCTION,
  LANGUAGE_CONSTANT,
  SemanticTokenType::NUMBER,
  SemanticTokenType::OPERATOR,
  SemanticTokenType::PARAMETER,
  SemanticTokenType::STRING,
  SemanticTokenType::VARIABLE,
];

struct Semantic {
  line: u32,
  start: u32,
  length: u32,
  token_type: SemanticTokenType,
  modifiers: Vec<Modifier>,
}

enum Modifier {
  _Mod1,
  _Mod2,
  _Mod3,
}

pub async fn calc_semantic_tokens(doc_loc: &DocLoc, text: &str) -> Option<SemanticTokensResult> {
  let (tokens, _) = lex(doc_loc, text);

  let mut last_loc = SourceLoc { doc_loc: doc_loc.clone(), pos: 0, line: 1, column: 1, length: 0 };

  let mut semantics = Vec::new();
  for token in tokens {
    if let Some(converted) = convert_token(&token, &last_loc) {
      semantics.push(converted);
      last_loc = token.source_loc;
    }
  }

  let data = semantics.iter().map(as_lsp_token).collect();
  Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data }))
}

fn convert_token(token: &Token, last_loc: &SourceLoc) -> Option<Semantic> {
  #[allow(clippy::match_same_arms)]
  let opt = match token.token_type {
    Comma => None,
    Colon => None,
    Divide => Some(SemanticTokenType::OPERATOR),
    Equals => None,
    GreaterThan => Some(SemanticTokenType::OPERATOR),
    GreaterThanEquals => Some(SemanticTokenType::OPERATOR),
    Identifier(_) => Some(SemanticTokenType::VARIABLE),
    Include => Some(SemanticTokenType::KEYWORD),
    LeftBrace => None,
    LeftBracket => None,
    LeftParen => None,
    LessThan => Some(SemanticTokenType::OPERATOR),
    LessThanEquals => Some(SemanticTokenType::OPERATOR),
    Newline => panic!("Lexer leaking newlines should not be possible"),
    Number(_) => Some(SemanticTokenType::NUMBER),
    Minus => Some(SemanticTokenType::OPERATOR),
    Multiply => Some(SemanticTokenType::OPERATOR),
    Plus => Some(SemanticTokenType::OPERATOR),
    RightBrace => None,
    RightBracket => None,
    RightParen => None,
    String(_) => Some(SemanticTokenType::STRING),
    Whitespace => panic!("Lexer leaking whitespace should not be possible"),
  };

  opt.map(|token_type| {
    let line = token.source_loc.line - last_loc.line;
    let start = if line == 0 {
      token.source_loc.column - last_loc.column
    } else {
      token.source_loc.column - 1
    };
    let length = token.source_loc.length;

    Semantic { line, start, length, token_type, modifiers: vec![] }
  })
}

fn as_lsp_token(token: &Semantic) -> SemanticToken {
  let token_type = as_type_int(&token.token_type);
  let modifiers = token.modifiers.iter().map(as_modifier_int).sum();
  SemanticToken {
    delta_line: token.line,
    delta_start: token.start,
    length: token.length,
    token_type,
    token_modifiers_bitset: modifiers,
  }
}

fn as_type_int(token_type: &SemanticTokenType) -> u32 {
  // The token types are not structural/matchable. --Jason B. (7/18/26)
  if token_type == &SemanticTokenType::FUNCTION {
    0
  } else if token_type == &LANGUAGE_CONSTANT {
    1
  } else if token_type == &SemanticTokenType::NUMBER {
    2
  } else if token_type == &SemanticTokenType::OPERATOR {
    3
  } else if token_type == &SemanticTokenType::PARAMETER {
    4
  } else if token_type == &SemanticTokenType::STRING {
    5
  } else if token_type == &SemanticTokenType::VARIABLE {
    6
  } else {
    eprintln!("Warning!  Unknown token type: {token_type:?}");
    100
  }
}

const fn as_modifier_int(modifier: &Modifier) -> u32 {
  match modifier {
    Modifier::_Mod1 => 1,
    Modifier::_Mod2 => 2,
    Modifier::_Mod3 => 4,
  }
}
