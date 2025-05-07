import * as vscode from "vscode";
import {parseFileContent} from "./image";
import * as path from 'path';
import * as yaml from "yaml";
import {TextDocument} from "vscode";

export async function listenOpenFile(panel: vscode.WebviewPanel) {
    // Listen for json files
    vscode.workspace.onDidOpenTextDocument(async document => {
        let textDocument = document;
        await publishFileContent(textDocument, panel);
    });
    vscode.workspace.onDidChangeTextDocument(async document => {
        let textDocument = document.document;
        await publishFileContent(textDocument, panel);
    });
}

async function publishFileContent(textDocument: TextDocument, panel: vscode.WebviewPanel): Promise<void> {
    const text = textDocument.getText().trim();
    const baseDir = path.dirname(textDocument.uri.fsPath);
    switch (textDocument.languageId) {
        case 'json':
            let parsed_json = JSON.parse(text);
            await parseFileContent(parsed_json, baseDir);
            panel.webview.postMessage({type: 'JSON', content: JSON.stringify(parsed_json, null, 2)});
            break;
        case 'yaml':
            let parsed_yaml = yaml.parse(text);
            await parseFileContent(parsed_yaml, baseDir);
            panel.webview.postMessage({type: 'YAML', content: JSON.stringify(parsed_yaml, null, 2)});
            break;
        default:
            break;
    }
}