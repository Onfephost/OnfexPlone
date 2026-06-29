const vscode = require('vscode');
const path = require('path');

function activate(context) {
  const runFile = vscode.commands.registerCommand('onfex.runFile', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      vscode.window.showErrorMessage('No active editor found.');
      return;
    }

    const document = editor.document;
    if (!document.fileName.endsWith('.onfex')) {
      vscode.window.showErrorMessage('Run is only available for .onfex files.');
      return;
    }

    if (document.isDirty) {
      await document.save();
    }

    const workspaceFolder = vscode.workspace.workspaceFolders
      ? vscode.workspace.workspaceFolders[0].uri.fsPath
      : path.dirname(document.fileName);
    const runnerPath = path.join(workspaceFolder, '.vscode', 'run_onfex.py');

    const terminal = vscode.window.createTerminal({ name: 'Onfex Run' });
    terminal.show(true);
    terminal.sendText(`python3 "${runnerPath}" "${document.fileName}"`);
  });

  context.subscriptions.push(runFile);
}

function deactivate() {}

module.exports = {
  activate,
  deactivate,
};
