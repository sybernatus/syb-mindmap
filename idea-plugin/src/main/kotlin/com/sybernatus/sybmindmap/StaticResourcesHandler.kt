package com.sybernatus.sybmindmap

import org.cef.callback.CefCallback
import org.cef.handler.CefResourceHandler
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefRequest
import org.cef.network.CefResponse
import java.io.ByteArrayInputStream

class StaticResourcesHandler(
    private val bytes: ByteArray,
    private val mimeType: String,
): CefResourceHandler {

    private var inputStream: ByteArrayInputStream? = null
    override fun processRequest(request: CefRequest?, callback: CefCallback): Boolean {
        inputStream = ByteArrayInputStream(bytes)
        callback.Continue()
        println("[PLUGIN] processRequest: ${request?.url}")
        return true
    }

    override fun getResponseHeaders(response: CefResponse?, responseLength: IntRef?, redirectUrl: StringRef?) {
        response?.status = 200
        response?.statusText = "OK"
        response?.mimeType = mimeType
        responseLength?.set(bytes.size)
    }

    override fun readResponse(
        dataOut: ByteArray?,
        bytesToRead: Int,
        bytesRead: IntRef?,
        callback: CefCallback?
    ): Boolean {
        if (dataOut == null || bytesRead == null || inputStream == null) return false
        val actualRead = inputStream!!.read(dataOut, 0, bytesToRead)
        if (actualRead == -1) return false // 🔁 plus de données
        bytesRead.set(actualRead)
        return true
    }

    override fun cancel() {

        inputStream?.close()
        inputStream = null
    }
}