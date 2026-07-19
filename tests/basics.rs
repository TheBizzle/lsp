#[cfg(test)]
mod tests {

  use std::path::PathBuf;

  use tokio::fs::read_to_string;

  use tower_lsp_server::ls_types::{
    Diagnostic, DiagnosticSeverity, DidOpenTextDocumentParams, Position, Range, TextDocumentItem,
  };
  use tower_lsp_server::{LanguageServer, LspService};

  use organic_lsp::lsp::lsp_backend::LspBackend;

  #[tokio::test]
  async fn can_open_cascade() {
    test_no_problem("./Organic/examples/cascade").await;
  }

  #[tokio::test]
  async fn can_open_chord_swell() {
    test_no_problem("./Organic/examples/chord_swell").await;
  }

  #[tokio::test]
  async fn can_open_groovy_bass() {
    test_no_problem("./Organic/examples/groovy_bass").await;
  }

  #[tokio::test]
  async fn can_open_harmonics() {
    test_no_problem("./Organic/examples/harmonics").await;
  }

  #[tokio::test]
  async fn can_open_siren() {
    test_no_problem("./Organic/examples/siren").await;
  }

  #[tokio::test]
  async fn can_open_spread_phase() {
    test_no_problem("./Organic/examples/spread_phase").await;
  }

  #[tokio::test]
  #[should_panic(expected = "No such file or directory")]
  async fn fails_to_open_nonexistent() {
    test_errors("./Organic/examples/doopy", Vec::new()).await;
  }

  #[tokio::test]
  async fn opens_and_errors_on_invalid() {
    let severity = Some(DiagnosticSeverity::ERROR);
    let message = "Unknown token: '".to_string();

    let start1 = Position { line: 0, character: 13 };
    let end1 = Position { line: 0, character: 14 };
    let range1 = Range { start: start1, end: end1 };
    let diagnostic1 = Diagnostic { range: range1, severity, message: message.clone(), ..Default::default() };

    let start2 = Position { line: 0, character: 24 };
    let end2 = Position { line: 0, character: 25 };
    let range2 = Range { start: start2, end: end2 };
    let diagnostic2 = Diagnostic { range: range2, severity, message, ..Default::default() };

    test_errors("./tests/invalid", vec![diagnostic1, diagnostic2]).await;
  }

  async fn test_no_problem(path: &str) {
    let diagnostics = open(path).await;
    assert!(diagnostics.is_empty(), "Expected {path} to have no errors, got: {diagnostics:?}");
  }

  async fn test_errors(path: &str, expected: Vec<Diagnostic>) {
    let actual = open(path).await;
    assert_eq!(actual, expected);
  }

  async fn open(path: &str) -> Vec<Diagnostic> {
    let path = PathBuf::from(format!("{path}.organic"));
    let uri = format!("file://{}", path.display()).parse().unwrap();
    let text = read_to_string(&path).await.unwrap();

    let backend_service = new_backend_service();
    let backend = backend_service.inner();

    backend
      .did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem { uri, language_id: "organic".into(), version: 1, text },
      })
      .await;

    backend.diagnostics.lock().await.clone()
  }

  fn new_backend_service() -> LspService<LspBackend> {
    let (service, _socket) = LspService::new(LspBackend::new);
    service
  }
}
