import * as lc from 'vscode-languageclient/node';
import * as vscode from 'vscode';

export async function startServer(
  context: vscode.ExtensionContext,
): Promise<lc.LanguageClient> {
  // bundled
  const server = vscode.Uri.joinPath(
    context.extensionUri,
    '../language-server/target/release/language-server',
  ).fsPath;

  console.log(`#### LANG SERVER LOCATION: ${server}`);

  const run = {
    command: server,
    options: { env: process.env },
  };

  // lsp
  const client = new lc.LanguageClient(
    'log_lang_server',
    'Log Language Server',
    {
      run: run,
      debug: run,
    },
    {
      documentSelector: [
        { scheme: 'file', language: 'typescript' },
        { scheme: 'file', language: 'javascript' },
      ],
      initializationOptions:
        vscode.workspace.getConfiguration('log-lang-server'),
      markdown: { supportHtml: true },
      traceOutputChannel: vscode.window.createOutputChannel('Log Server Trace'),
      outputChannel: vscode.window.createOutputChannel('Log Server'),
      synchronize: {
        fileEvents: vscode.workspace.createFileSystemWatcher('**/*.js|ts'),
      },
    },
  );

  try {
    await client.start();
  } catch (error) {
    console.log(error);
  }
  return client;
}
