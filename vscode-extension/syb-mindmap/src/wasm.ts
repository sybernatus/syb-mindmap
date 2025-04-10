import { log } from "console";
import * as vscode from "vscode";

export async function listenOpenFile(panel: vscode.WebviewPanel) {
    // Listen for json files
    vscode.workspace.onDidOpenTextDocument(document => {
        log('aaaaaaaaaa', document.languageId)
        if (document.languageId === 'json') {
            panel.webview.postMessage({
                type: 'json-data',
                content: document.getText()
            });
        }
    });
    // Listen for json files
    vscode.workspace.onDidChangeTextDocument(document => {
        log('aaaaaaaaaa', document.document.languageId)
        if (document.document.languageId === 'json') {
            panel.webview.postMessage({
                type: 'json-data',
                content: document.document.getText()
            });
        }
    });
}