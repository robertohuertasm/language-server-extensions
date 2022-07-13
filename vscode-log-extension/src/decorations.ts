import * as vscode from 'vscode';

export class DecorationProvider {
  private decorationType: vscode.TextEditorDecorationType;

  constructor(context: vscode.ExtensionContext) {
    const iconPath = vscode.Uri.joinPath(context.extensionUri, './logo.svg');
    const iconPathDark = vscode.Uri.joinPath(
      context.extensionUri,
      './logo_dark.svg',
    );
    this.decorationType = vscode.window.createTextEditorDecorationType({
      // backgroundColor: 'green',
      // border: '2px solid white',
      gutterIconPath: iconPath,
      light: {
        gutterIconPath: iconPathDark,
      },
      // before: {
      //   contentIconPath: iconPath,
      //   width: '14px',
      //   height: '14px',
      //   margin: '0px 10px 0px 10px',
      // },
    });
  }

  public async enableDecorations() {
    vscode.workspace.onWillSaveTextDocument(async (e) => {
      await this.decorate(e.document);
    });
    vscode.workspace.onDidOpenTextDocument(async (document) => {
      await this.decorate(document);
    });
  }

  public async decorate(document: vscode.TextDocument) {
    const openEditors = vscode.window.visibleTextEditors.filter(
      (editor) => editor.document.uri === document.uri,
    );
    if (openEditors.length) {
      await this.decorateEditor(openEditors[0]);
    } else if (vscode.window.activeTextEditor) {
      await this.decorateEditor(vscode.window.activeTextEditor);
    }
  }

  private async getLenses(
    document: vscode.TextDocument,
  ): Promise<vscode.CodeLens[]> {
    const lenses: vscode.CodeLens[] = await vscode.commands.executeCommand(
      'vscode.executeCodeLensProvider',
      document.uri,
    );
    console.log('received lenses', lenses);
    return lenses;
  }

  private async decorateEditor(editor: vscode.TextEditor) {
    const document = editor.document;
    const lenses = await this.getLenses(document);

    const decorations: vscode.DecorationOptions[] = lenses.map((lens) => ({
      range: lens.range,
    }));

    editor.setDecorations(this.decorationType, decorations);
  }
}
