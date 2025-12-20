import {
  createConnection,
  TextDocuments,
  Diagnostic,
  DiagnosticSeverity,
  ProposedFeatures,
  InitializeParams,
  DidChangeConfigurationNotification,
  CompletionItem,
  CompletionItemKind,
  TextDocumentPositionParams,
  TextDocumentSyncKind,
  InitializeResult
} from 'vscode-languageserver/node';

import { TextDocument } from 'vscode-languageserver-textdocument';

// Import the WASM compiler
let wasmCompiler: any;
let wasmInitialized = false;

async function initWasm() {
  if (wasmInitialized) return;
  try {
    wasmCompiler = await import('@htms/compiler');
    wasmInitialized = true;
  } catch (err) {
    console.error('Failed to initialize WASM compiler:', err);
  }
}

// Create a connection for the server
const connection = createConnection(ProposedFeatures.all);

// Create a simple text document manager
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;

connection.onInitialize((params: InitializeParams) => {
  const capabilities = params.capabilities;

  hasConfigurationCapability = !!(
    capabilities.workspace && !!capabilities.workspace.configuration
  );
  hasWorkspaceFolderCapability = !!(
    capabilities.workspace && !!capabilities.workspace.workspaceFolders
  );

  const result: InitializeResult = {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      completionProvider: {
        resolveProvider: true,
        triggerCharacters: ['@', '<', '{', '.']
      }
    }
  };

  if (hasWorkspaceFolderCapability) {
    result.capabilities.workspace = {
      workspaceFolders: {
        supported: true
      }
    };
  }

  return result;
});

connection.onInitialized(async () => {
  if (hasConfigurationCapability) {
    connection.client.register(DidChangeConfigurationNotification.type, undefined);
  }

  // Initialize WASM compiler
  await initWasm();
});

// The content of a text document has changed. This event is emitted
// when the text document first opened or when its content has changed.
documents.onDidChangeContent(change => {
  validateTextDocument(change.document);
});

async function validateTextDocument(textDocument: TextDocument): Promise<void> {
  if (!wasmInitialized) {
    await initWasm();
  }

  if (!wasmCompiler) {
    return;
  }

  const text = textDocument.getText();
  const diagnostics: Diagnostic[] = [];

  try {
    // Compile the document
    const result = wasmCompiler.compile(text);

    // Convert compiler diagnostics to LSP diagnostics
    if (result.diagnostics && Array.isArray(result.diagnostics)) {
      for (const diag of result.diagnostics) {
        const severity = diag.severity === 'Error'
          ? DiagnosticSeverity.Error
          : DiagnosticSeverity.Warning;

        const diagnostic: Diagnostic = {
          severity,
          range: {
            start: {
              line: (diag.location?.line || 1) - 1,
              character: (diag.location?.column || 1) - 1
            },
            end: {
              line: (diag.location?.line || 1) - 1,
              character: (diag.location?.column || 1) + (diag.location?.length || 1) - 1
            }
          },
          message: diag.message,
          source: 'htms'
        };

        if (diag.code) {
          diagnostic.code = diag.code;
        }

        diagnostics.push(diagnostic);
      }
    }
  } catch (err: any) {
    // If compilation throws an error, show it as a diagnostic
    diagnostics.push({
      severity: DiagnosticSeverity.Error,
      range: {
        start: { line: 0, character: 0 },
        end: { line: 0, character: Number.MAX_VALUE }
      },
      message: err.message || 'Unknown compilation error',
      source: 'htms'
    });
  }

  // Send the computed diagnostics to VSCode
  connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

// This handler provides the initial list of completion items
connection.onCompletion(
  (_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
    const items: CompletionItem[] = [];

    // Add HTMS keywords
    items.push(
      {
        label: 'component',
        kind: CompletionItemKind.Keyword,
        detail: 'Declare a component',
        insertText: 'component ${1:ComponentName} {\n\t$0\n}'
      },
      {
        label: 'section',
        kind: CompletionItemKind.Keyword,
        detail: 'Declare a section',
        insertText: 'section ${1:SectionName} {\n\t$0\n}'
      },
      {
        label: 'page',
        kind: CompletionItemKind.Keyword,
        detail: 'Declare a page',
        insertText: 'page ${1:home} "${2:/}" {\n\t$0\n}'
      },
      {
        label: '@if',
        kind: CompletionItemKind.Keyword,
        detail: 'Conditional rendering',
        insertText: '@if ${1:ctx.condition} {\n\t$0\n}'
      },
      {
        label: '@each',
        kind: CompletionItemKind.Keyword,
        detail: 'Loop over items',
        insertText: '@each ${1:ctx.items} as ${2:item} {\n\t$0\n}'
      },
      {
        label: '@slot',
        kind: CompletionItemKind.Keyword,
        detail: 'Insert slot',
        insertText: '@slot'
      },
      {
        label: '@else',
        kind: CompletionItemKind.Keyword,
        detail: 'Else branch',
        insertText: '@else {\n\t$0\n}'
      }
    );

    // Add common HTML elements
    const htmlElements = ['div', 'span', 'p', 'a', 'button', 'input', 'form', 'h1', 'h2', 'h3', 'ul', 'li', 'img'];
    for (const element of htmlElements) {
      items.push({
        label: element,
        kind: CompletionItemKind.Property,
        detail: `HTML <${element}> element`,
        insertText: `${element} {\n\t$0\n}`
      });
    }

    // Add common attributes
    const attributes = [
      { name: 'class', detail: 'CSS class name' },
      { name: 'id', detail: 'Element ID' },
      { name: 'href', detail: 'Link URL' },
      { name: 'src', detail: 'Image/script source' },
      { name: 'onClick', detail: 'Click event handler' },
      { name: 'bind', detail: 'Two-way data binding' },
      { name: 'value', detail: 'Input value' },
      { name: 'type', detail: 'Input type' }
    ];

    for (const attr of attributes) {
      items.push({
        label: attr.name,
        kind: CompletionItemKind.Field,
        detail: attr.detail,
        insertText: `${attr.name}: $0`
      });
    }

    return items;
  }
);

// This handler resolves additional information for the item selected in
// the completion list.
connection.onCompletionResolve(
  (item: CompletionItem): CompletionItem => {
    return item;
  }
);

// Make the text document manager listen on the connection
// for open, change and close text document events
documents.listen(connection);

// Listen on the connection
connection.listen();
