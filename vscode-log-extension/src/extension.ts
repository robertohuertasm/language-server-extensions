import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';
import { startServer } from './server';
import { DecorationProvider } from './decorations';

let client: LanguageClient;

export async function activate(context: vscode.ExtensionContext) {
  console.log(
    'Congratulations, your extension "vscode-log-extension" is now active!',
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('vscode-log-extension.enableServer', () => {
      vscode.window.showInformationMessage('Log Server Enabled!');
    }),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('log-server.log', () => {
      vscode.window.showInformationMessage('Show me the logs!');
    }),
  );

  client = await startServer(context);

  // decorations on save
  const decorations = new DecorationProvider(context);
  decorations.enableDecorations();
}

// this method is called when your extension is deactivated
export function deactivate() {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
