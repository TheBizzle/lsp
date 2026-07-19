use std::collections::HashMap;

use tower_lsp_server::LanguageServer;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::{
  CodeActionParams, CodeActionProviderCapability, CodeActionResponse, CompletionOptions, CompletionParams,
  CompletionResponse, DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentFormattingParams,
  GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability,
  InitializeParams, InitializeResult, InitializedParams, Location, MarkedString, MessageType, OneOf,
  Position, Range, ReferenceParams, RenameParams, SemanticTokensFullOptions, SemanticTokensLegend,
  SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult, SemanticTokensServerCapabilities,
  ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Uri,
  WorkDoneProgressOptions, WorkspaceEdit,
};

pub mod lsp_backend;
pub mod semantic_tokens;

use crate::errors::LspError::LspLexerError;
use crate::errors::as_diagnostic;
use crate::lexer::doc_loc::DocLoc;
use crate::lexer::lex;
use crate::lsp::lsp_backend::LspBackend;
use crate::lsp::semantic_tokens::{TOKEN_TYPES, calc_semantic_tokens};

const DEBUG: MessageType = MessageType::ERROR;

impl LanguageServer for LspBackend {
  async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
    Ok(InitializeResult {
      capabilities: ServerCapabilities {
        // TODO: TextDocumentSyncKind::INCREMENTAL
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),

        hover_provider: Some(HoverProviderCapability::Simple(true)),

        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
          SemanticTokensOptions {
            legend: SemanticTokensLegend { token_types: TOKEN_TYPES.to_vec(), token_modifiers: vec![] },
            range: Some(false),
            full: Some(SemanticTokensFullOptions::Bool(true)),
            work_done_progress_options: WorkDoneProgressOptions::default(),
          },
        )),

        completion_provider: Some(CompletionOptions::default()),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
        document_formatting_provider: Some(OneOf::Left(true)),

        ..ServerCapabilities::default()
      },

      ..InitializeResult::default()
    })
  }

  async fn initialized(&self, _: InitializedParams) {
    self.client.log_message(DEBUG, "Organic LSP initialized!").await;
  }

  async fn shutdown(&self) -> Result<()> {
    Ok(())
  }

  async fn code_action(&self, _params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
    Ok(Some(vec![])) // TODO
  }

  async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
    Ok(None) // TODO
  }

  async fn did_open(&self, params: DidOpenTextDocumentParams) {
    store_and_reanalyze(self, params.text_document.uri, params.text_document.text).await;
  }

  async fn did_change(&self, params: DidChangeTextDocumentParams) {
    if let Some(change) = params.content_changes.into_iter().next() {
      let uri = params.text_document.uri;
      let doc_loc = DocLoc::new(uri.to_string());
      self.documents.write().await.insert(doc_loc, change.text.clone());
      store_and_reanalyze(self, uri, change.text).await;
    }
  }

  async fn formatting(&self, _params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
    Ok(Some(vec![])) // TODO
  }

  async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;

    let location = Location {
      uri,
      range: Range {
        // TODO
        start: Position::new(0, 0),
        end: Position::new(0, 0),
      },
    };

    Ok(Some(GotoDefinitionResponse::Scalar(location)))
  }

  async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
    // TODO
    Ok(Some(Hover {
      contents: HoverContents::Scalar(MarkedString::String("Organic LSP sends its regards".to_string())),
      range: None,
    }))
  }

  async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
    let location = Location {
      uri: params.text_document_position.text_document.uri,
      range: Range {
        // TODO
        start: Position::new(0, 0),
        end: Position::new(0, 0),
      },
    };

    Ok(Some(vec![location]))
  }

  async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
    let pos = params.text_document_position;
    Ok(Some(WorkspaceEdit {
      changes: Some(HashMap::from([(
        pos.text_document.uri,
        vec![TextEdit {
          range: Range {
            // TODO
            start: Position::new(0, 0),
            end: Position::new(0, 0),
          },
          new_text: params.new_name,
        }],
      )])),
      document_changes: None,
      change_annotations: None,
    }))
  }

  async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
    let uri = DocLoc::new(params.text_document.uri.to_string());
    let text = self.documents.read().await.get(&uri).cloned();

    if let Some(text) = text {
      Ok(calc_semantic_tokens(&uri, &text).await)
    } else {
      let msg = format!("No known document for URI: {uri:?}");
      self.client.log_message(DEBUG, msg).await;
      Result::Ok(None)
    }
  }
}

async fn store_and_reanalyze(this: &LspBackend, uri: Uri, text: String) {
  let doc_loc = DocLoc::new(uri.to_string());
  this.documents.write().await.insert(doc_loc.clone(), text.clone());

  let (_tokens, errors) = lex(&doc_loc, &text);
  let diagnostics: Vec<_> = errors.into_iter().map(LspLexerError).map(as_diagnostic).collect();
  this.diagnostics.lock().await.extend(diagnostics);

  let diagnostics = this.diagnostics.lock().await.clone();
  this.client.publish_diagnostics(uri, diagnostics, None).await;
}
