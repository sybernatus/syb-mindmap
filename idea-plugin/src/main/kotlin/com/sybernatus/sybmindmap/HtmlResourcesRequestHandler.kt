package com.sybernatus.sybmindmap

import org.cef.browser.CefBrowser
import org.cef.browser.CefFrame
import org.cef.handler.CefRequestHandlerAdapter
import org.cef.handler.CefResourceHandler
import org.cef.handler.CefResourceRequestHandler
import org.cef.handler.CefResourceRequestHandlerAdapter
import org.cef.misc.BoolRef
import org.cef.network.CefRequest
import java.net.URLConnection

class HtmlResourcesRequestHandler : CefRequestHandlerAdapter() {
    override fun getResourceRequestHandler(
        browser: CefBrowser?,
        frame: CefFrame?,
        request: CefRequest?,
        isNavigation: Boolean,
        isDownload: Boolean,
        requestInitiator: String?,
        disableDefaultHandling: BoolRef?
    ): CefResourceRequestHandler = object : CefResourceRequestHandlerAdapter() {
        override fun getResourceHandler(
            browser: CefBrowser?,
            frame: CefFrame?,
            request: CefRequest?
        ): CefResourceHandler? {
            val url = request?.url ?: return null
            println("[PLUGIN] Request intercepted: $url")

            if (!url.startsWith("http://local.plugin/")) return null

            val path = url.removePrefix("http://local.plugin/").ifBlank { "index.html" }
            if (path.endsWith("favicon.ico") || path.endsWith("manifest.json") || path.endsWith("robots.txt")) {
                return null
            }

            val resourcePath = "html/$path"
            val resourceStream = javaClass.classLoader.getResourceAsStream(resourcePath)

            if (resourceStream == null) {
                println("[PLUGIN] Ressource not found: $resourcePath")
                return FallbackResourceHandler(resourcePath)
            }

            val bytes = resourceStream.readBytes()
            val mimeType = guessMimeType(path)

            return if (path == "index.html") {
                val html = String(bytes, Charsets.UTF_8)
                    .replace(Regex("(src|href)=\"(?!https?://|data:|chrome:|chrome-extension:)([^\"]+)\"")) {
                        val attr = it.groupValues[1]
                        val value = it.groupValues[2]
                        "$attr=\"http://local.plugin/$value\""
                    }
                    .replace(Regex("(import|init)\"(?!https?://|data:|chrome:|chrome-extension:)([^\"]+)\"")) {
                        val keyword = it.groupValues[1]
                        val pathVal = it.groupValues[2]
                        "$keyword(\"http://local.plugin/$pathVal\""
                    }
                StaticResourcesHandler(html.toByteArray(Charsets.UTF_8), mimeType)
            } else {
                StaticResourcesHandler(bytes, mimeType)
            }
        }
    }

    override fun onBeforeBrowse(
        browser: CefBrowser?,
        frame: CefFrame?,
        request: CefRequest?,
        user_gesture: Boolean,
        is_redirect: Boolean
    ): Boolean {
        val url = request?.url ?: return false
        val isInternal = url.startsWith("http://local.plugin/")
        if (!isInternal) {
            return true
        }
        return false
    }

    private fun guessMimeType(fileName: String): String =
        URLConnection.guessContentTypeFromName(fileName) ?: "application/octet-stream"

}