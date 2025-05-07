package com.sybernatus.sybmindmap.compute

import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.node.ArrayNode
import com.fasterxml.jackson.databind.node.ObjectNode
import com.intellij.openapi.vfs.VirtualFile
import com.sybernatus.sybmindmap.enums.MimeType
import net.coobird.thumbnailator.Thumbnails
import java.io.ByteArrayOutputStream
import java.io.File
import java.nio.file.Files
import java.nio.file.Path
import java.util.*

class DataCompute(private val mapper: ObjectMapper) {

    fun parseFileContent(documentText: String, file: VirtualFile): String {

        val rootNode = mapper.readTree(documentText) as ObjectNode

        // Enrich with image.data
        val dataNode = rootNode.get("data")
        if (dataNode != null) {
            traverseNodeTree(dataNode, file.toNioPath().parent)
        }

        return mapper.writeValueAsString(rootNode)
    }

    private fun traverseNodeTree(node: JsonNode, baseDir: Path) {
        if (node !is ObjectNode) return

        val imageNode = node.get("image")
        if (imageNode is ObjectNode) {
            val pathNode = imageNode.get("path")
            if (pathNode != null && pathNode.isTextual) {
                val relativePath = pathNode.textValue()
                val imagePath = baseDir.resolve(relativePath).normalize()
                if (Files.exists(imagePath)) {
                    val bytes = Files.readAllBytes(imagePath)
                    when (val mimeType = getMimeType(imagePath)) {
                        MimeType.PNG.mimeType, MimeType.JPEG.mimeType -> {
                            println("[PLUGIN] Compressing image: $imagePath")
                            val base64 = compressImageToBase64(imagePath.toString())
                            imageNode.put("data", "data:$mimeType;base64,$base64")
                        }
                        MimeType.SVG.mimeType -> {
                            println("[PLUGIN] Encoding SVG image: $imagePath")
                            // For SVG, we can directly encode the bytes to base64
                            val base64 = Base64.getEncoder().encodeToString(bytes)
                            imageNode.put("data", "data:$mimeType;base64,$base64")
                        }
                        else -> {
                            println("[PLUGIN] Unsupported image type: $mimeType")
                        }
                    }
                } else {
                    println("[PLUGIN] Image file not found: $imagePath")
                }
            }
        }

        // recursively enrich children nodes
        val childrenNode = node.get("children")
        if (childrenNode is ArrayNode) {
            for (child in childrenNode) {
                traverseNodeTree(child, baseDir)
            }
        }
    }

    private fun getMimeType(filePath: Path) : String {
        return try {
            if (Files.probeContentType(filePath) != null) {
                Files.probeContentType(filePath)
            } else {
                val extension = filePath.fileName.toString().substringAfterLast('.')
                when (extension) {
                    "png" -> MimeType.PNG.mimeType
                    "jpg", "jpeg" -> MimeType.JPEG.mimeType
                    "svg" -> MimeType.SVG.mimeType
                    else -> MimeType.OCTET_STREAM.mimeType
                }
            }
        } catch (e: Exception) {
            println("[PLUGIN] Error getting MIME type: ${e.message}")
            MimeType.OCTET_STREAM.mimeType
        }
    }

    private fun compressImageToBase64(filePath: String): String {
        val inputFile = File(filePath)
        if (!inputFile.exists()) {
            throw IllegalArgumentException("File not found: $filePath")
        }

        val outputStream = ByteArrayOutputStream()

        Thumbnails.of(inputFile)
            .width(1024)
            .outputFormat("png")
            .toOutputStream(outputStream)

        val compressedBytes = outputStream.toByteArray()
        return Base64.getEncoder().encodeToString(compressedBytes)
    }
}