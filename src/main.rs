use tokio::io::{stdin, stdout};

use tower_lsp_server::{LspService, Server};

use organic_lsp::lsp::lsp_backend::LspBackend;

#[tokio::main]
async fn main() {
  let (service, socket) = LspService::new(LspBackend::new);
  Server::new(stdin(), stdout(), socket).serve(service).await;
}
