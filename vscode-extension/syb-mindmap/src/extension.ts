// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
import * as fs from 'fs';
import {listenOpenFile} from "./wasm";


export async function loadWebview(context: vscode.ExtensionContext) {
	const panel = vscode.window.createWebviewPanel(
		'sybMindmap',
		'Sybernatus Mindmap as code',
		vscode.ViewColumn.Beside,
		{
			enableScripts: true,
			localResourceRoots: [
				vscode.Uri.joinPath(context.extensionUri),
				vscode.Uri.joinPath(context.extensionUri, 'media'),
				vscode.Uri.joinPath(context.extensionUri, 'assets'),
				vscode.Uri.joinPath(context.extensionUri, 'media', 'assets')
			]
		}
	)

	const _ = await listenOpenFile(panel);

	const htmlUri = vscode.Uri.joinPath(
		context.extensionUri,
		'media',
		'index.html'
	);
	
	const htmlContent = await fs.promises.readFile(htmlUri.fsPath, 'utf-8');
	panel.webview.html = fixLinksForWebview(htmlContent, panel, context);
}

function fixLinksForWebview(html: string, panel: vscode.WebviewPanel, context: vscode.ExtensionContext): string {
	return html
	.replace(/(src|href)="([^"]+)"/g, (_, attr, path) => {
	  const webviewUri = panel.webview.asWebviewUri(
		vscode.Uri.joinPath(context.extensionUri, 'media', path)
	  );
	  return `${attr}="${webviewUri}"`;
	})
	.replace(/(import|init)\("([^"]+)"/g, (_, attr, path) => {
	  const webviewUri = panel.webview.asWebviewUri(
		vscode.Uri.joinPath(context.extensionUri, 'media', path)
	  );
	  return `${attr}("${webviewUri}"`;
	});
  }
  
  

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "syb-mindmap" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	const disposable = vscode.commands.registerCommand('syb-mindmap.openWebview', async () => {
		// The code you place here will be executed every time your command is executed
		// Display a message box to the user
		vscode.window.showInformationMessage('Hello World from syb-mindmap!');
		console.log("Webview ouverte !");
		await loadWebview(context)

	});

	context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() {}

