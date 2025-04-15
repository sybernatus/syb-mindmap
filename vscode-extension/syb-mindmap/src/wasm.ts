import { log } from "console";
import * as vscode from "vscode";

export async function listenOpenFile(panel: vscode.WebviewPanel) {
    // Listen for json files
    vscode.workspace.onDidOpenTextDocument(document => {
        let textDocument = document;
        switch (textDocument.languageId) {
            case 'json':
                panel.webview.postMessage({type: 'JSON', content: textDocument.getText().trim()});
                break;
            case 'yaml':
                panel.webview.postMessage({type: 'YAML', content: textDocument.getText().trim()});
                break;
            default:
                break;
        }
    });
    vscode.workspace.onDidChangeTextDocument(document => {
        let textDocument = document.document;
        switch (textDocument.languageId) {
            case 'json':
                panel.webview.postMessage({type: 'JSON', content: textDocument.getText().trim()});
                break;
            case 'yaml':
                panel.webview.postMessage({type: 'YAML', content: textDocument.getText().trim()});
                break;
            default:
                break;
        }
    });
}