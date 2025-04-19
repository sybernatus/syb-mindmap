package com.sybernatus.sybmindmap

import org.cef.callback.CefCallback
import org.cef.handler.CefResourceHandler
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefRequest
import org.cef.network.CefResponse
import java.io.ByteArrayInputStream

class FallbackResourceHandler(
    private val resourcePath: String,
): CefResourceHandler {
    private val fallback = """<html><body><h2>Fichier manquant: $resourcePath</h2></body></html>"""
        .toByteArray()
    private val input = ByteArrayInputStream(fallback)

    override fun processRequest(request: CefRequest?, callback: CefCallback): Boolean {
        callback.Continue()
        return true
    }

    override fun getResponseHeaders(
        response: CefResponse?,
        responseLength: IntRef?,
        redirectUrl: StringRef?
    ) {
        response?.status = 200
        response?.statusText = "OK"
        response?.mimeType = "text/html"
        responseLength?.set(fallback.size)
    }

    override fun readResponse(
        dataOut: ByteArray?,
        bytesToRead: Int,
        bytesRead: IntRef?,
        callback: CefCallback?
    ): Boolean {
        if (dataOut == null || bytesRead == null) return false
        val actualRead = input.read(dataOut, 0, bytesToRead)
        if (actualRead <= 0) return false
        bytesRead.set(actualRead)
        return true
    }

    override fun cancel() {}
}
