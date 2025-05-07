import * as fs from 'fs/promises';
import * as path from 'path';
import sharp from 'sharp';

enum MIME_TYPE {
    PNG = 'image/png',
    JPEG = 'image/jpeg',
    SVG = 'image/svg+xml',
    OCTET_STREAM = 'application/octet-stream',
}

async function compressImageToBase64(filePath: string): Promise<string> {

    const buffer = await fs.readFile(filePath);
    const resized = await sharp(buffer)
        .resize(1024)
        .toBuffer();

    return resized.toString('base64');
}

// Get the image MIME type based on the file extension
function getMimeType(filePath: string): string {
    const ext = path.extname(filePath).toLowerCase();
    if (ext === '.png') return MIME_TYPE.PNG;
    if (ext === '.jpg' || ext === '.jpeg') return MIME_TYPE.JPEG;
    if (ext === '.svg') return MIME_TYPE.SVG;
    return MIME_TYPE.OCTET_STREAM;
}

// Read image path from .image.path and encode it into .image.data
async function inlineImageField(node: any, baseDir: string): Promise<void> {
    const pathStr = node?.image?.path;
    if (typeof pathStr === 'string') {
        const filePath = path.resolve(baseDir, pathStr);
        try {
            const mime = getMimeType(filePath);
            switch (mime) {
                case MIME_TYPE.PNG:
                case MIME_TYPE.JPEG:
                    const buffer = await compressImageToBase64(filePath);
                    node.image.data = `data:${mime};base64,${buffer}`;
                    break;
                case MIME_TYPE.SVG:
                    const svgContent = await fs.readFile(filePath, 'utf-8');
                    node.image.data = `data:${mime};utf8,${encodeURIComponent(svgContent)}`;
                    return;
                default:
                    console.warn(`Unsupported image type: ${mime}`);
                    return;
            }
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