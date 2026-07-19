use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use tower_lsp_server::Client;
use tower_lsp_server::ls_types::Diagnostic;

use crate::lexer::doc_loc::DocLoc;

type Diagnostics = Arc<Mutex<Vec<Diagnostic>>>;
type Documents = Arc<RwLock<HashMap<DocLoc, String>>>;

#[derive(Debug)]
pub struct LspBackend {
  pub client: Client,
  pub diagnostics: Diagnostics,
  pub documents: Documents,
}

impl LspBackend {
  #[must_use]
  pub fn new(client: Client) -> Self {
    let diagnostics = Arc::new(Mutex::new(Vec::new()));
    let documents = Arc::new(RwLock::new(HashMap::new()));
    Self { client, diagnostics, documents }
  }
}
