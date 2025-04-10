import { log } from "console";
import * as vscode from "vscode";

export async function listenOpenFile(panel: vscode.WebviewPanel) {
    // Listen for json files
    vscode.workspace.onDidOpenTextDocument(document => {
        let textDocument = document;
        if (document.languageId === 'json') {
            panel.webview.postMessage({
                event: 'json-open-file',
                type: 'json-data',
                name: textDocument.fileName,
                content: {
                    text: textDocument.getText()
                }
            });
        }
    });
    vscode.workspace.onDidChangeTextDocument(document => {
        let textDocument = document.document;
        if (textDocument.languageId === 'json') {
            panel.webview.postMessage({
                event: 'json-change-file',
                type: 'json-data',
                name: textDocument.fileName,
                content: {
                    text: textDocument.getText()
                }
            });
        }
    });
}