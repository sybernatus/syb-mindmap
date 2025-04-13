import { log } from "console";
import * as vscode from "vscode";

export async function listenOpenFile(panel: vscode.WebviewPanel) {
    // Listen for json files
    vscode.workspace.onDidOpenTextDocument(document => {
        let textDocument = document;
        if (textDocument.languageId === 'json') {
            panel.webview.postMessage({
                type: 'JSON',
                content: textDocument.getText().trim()
            });
        }
    });
    vscode.workspace.onDidChangeTextDocument(document => {
        let textDocument = document.document;
        if (textDocument.languageId === 'json') {
            panel.webview.postMessage({
                type: 'JSON',
                content: textDocument.getText().trim()
            });
        }
    });
}