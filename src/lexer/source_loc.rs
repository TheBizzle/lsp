use crate::lexer::doc_loc::DocLoc;

#[derive(Clone, Copy, Debug, Default)]
pub struct MiniLoc {
  pub line: u32,
  pub column: u32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceLoc {
  pub doc_loc: DocLoc,
  pub pos: u32,
  pub line: u32,
  pub column: u32,
  pub length: u32,
}
