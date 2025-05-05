import * as fs from 'fs/promises';
import * as path from 'path';

// Get the image MIME type based on the file extension
function getMimeType(filePath: string): string {
    const ext = path.extname(filePath).toLowerCase();
    if (ext === '.png') return 'image/png';
    if (ext === '.jpg' || ext === '.jpeg') return 'image/jpeg';
    if (ext === '.svg') return 'image/svg+xml';
    return 'application/octet-stream';
}

// Read image path from .image.path and encode it into .image.data
async function inlineImageField(node: any, baseDir: string): Promise<void> {
    const pathStr = node?.image?.path;
    if (typeof pathStr === 'string') {
        const filePath = path.resolve(baseDir, pathStr);
        try {
            const buffer = await fs.readFile(filePath);
            const mime = getMimeType(filePath);
            node.image.data = `data:${mime};base64,${buffer.toString('base64')}`;
        } catch (err) {
            console.warn(`Image not found or unreadable: ${filePath}`);
        }
    }
}

async function traverseNodeTree(node: any, baseDir: string): Promise<void> {
    if (typeof node !== 'object' || node === null) return;

    await inlineImageField(node, baseDir);

    if (Array.isArray(node.children)) {
        for (const child of node.children) {
            await traverseNodeTree(child, baseDir);
        }
    }

}

export async function parseFileContent(fileContent: any, baseDir: string): Promise<void> {
    if (typeof fileContent !== 'object' || fileContent === null) {
        return;
    }

    if (fileContent.data) {
        await traverseNodeTree(fileContent.data, baseDir);
    }
}