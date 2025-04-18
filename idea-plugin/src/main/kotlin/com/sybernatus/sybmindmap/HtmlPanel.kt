package com.sybernatus

import com.intellij.openapi.diagnostic.Logger
import com.intellij.ui.jcef.JBCefApp
import com.intellij.ui.jcef.JBCefBrowser
import org.cef.browser.CefBrowser
import org.cef.browser.CefFrame
import org.cef.callback.CefCallback
import org.cef.handler.CefRequestHandlerAdapter
import org.cef.handler.CefResourceHandler
import org.cef.handler.CefResourceRequestHandler
import org.cef.handler.CefResourceRequestHandlerAdapter
import org.cef.misc.BoolRef
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefRequest
import org.cef.network.CefResponse
import java.io.ByteArrayInputStream
import java.io.File
import java.net.URL
import java.net.URLConnection
import java.nio.file.Files
import javax.swing.JComponent
import javax.swing.SwingUtilities


class HtmlPanel {
  val component: JComponent
  private val LOG = Logger.getInstance(HtmlPanel::class.java)

  init {
    System.setProperty("ide.browser.jcef.debug.port", "9222")
    LOG.info("HtmlPanel initialized")
  }


  init {
    check(JBCefApp.isSupported()) { "JCEF n’est pas supporté sur cette version d’IntelliJ." }
//    val rawHtml = HtmlPanel::class.java.classLoader
//      .getResource("html/index.html")
//      ?.readTextFromResource()
//      ?: throw IllegalStateException("index.html not found")
//
//    val tempDir = Files.createTempDirectory("html-view").toFile()
//    val resourcePaths = extractResourcePaths(rawHtml)
//    copyResourcesToTemp(resourcePaths, tempDir)
//    File(tempDir, "index.html").writeText(rawHtml)
//    val indexFile = File(tempDir, "index.html")
//    if (!indexFile.exists()) {
//      throw IllegalStateException("index.html non copié dans le dossier temporaire.")
//    }

    val browser = JBCefBrowser("http://local.plugin/index.html")
    SwingUtilities.invokeLater {
      val devTools = browser.cefBrowser.devTools
      browser.openDevtools()
      JBCefBrowser.createBuilder()
        .setCefBrowser(devTools)
        .setClient(browser.jbCefClient)
        .build()
    }
    browser.jbCefClient.addRequestHandler(object : CefRequestHandlerAdapter() {
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

          if (!url.startsWith("http://local.plugin/")) return null
          println("[PLUGIN] Requête interceptée: $url")

          val path = url.removePrefix("http://local.plugin/").ifBlank { "index.html" }
          if (path.endsWith("favicon.ico") || path.endsWith("manifest.json") || path.endsWith("robots.txt")) {
            return null
          }

          val resourcePath = "html/$path"
          val resourceStream = HtmlPanel::class.java.classLoader.getResourceAsStream(resourcePath) ?: return null
          val bytes = resourceStream.readBytes()
          val mimeType = guessMimeType(path)

          return object : CefResourceHandler {
            override fun processRequest(request: CefRequest?, callback: CefCallback): Boolean {
              callback.Continue()
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
              val stream = ByteArrayInputStream(bytes)
              val actualRead = dataOut?.let { stream.read(it, 0, bytesToRead) }
              if (actualRead != null) {
                if (actualRead <= 0) return false
              }
              if (actualRead != null) {
                bytesRead?.set(actualRead)
              }
              return true
            }

            override fun cancel() {}
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
        println("[PLUGIN] 🔒 Blocage navigation externe vers: $url ?????")
        val isInternal = url.startsWith("http://local.plugin/")
        if (!isInternal) {
          println("[PLUGIN] 🔒 Blocage navigation externe vers: $url")
          return true // ⛔ BLOQUE la navigation externe
        }
        return false
      }
    }, browser.cefBrowser)

//    component = browser.component
//    browser.loadURL(indexFile.toURI().toString())
//
    // DevTools facultatif
//
    component = browser.component
    // Replace srx & href links
//    val htmlWithCorrectedLinks = rawHtml.replace(Regex("""(src|href)="([^"]+)"""")) { match ->
//      val attr = match.groupValues[1]
//      val path = match.groupValues[2]
//      val uri = resourceToUri(path)
//      """$attr="$uri""""
//    }
//
//    // Replace import & init links
//    val finalHtml = htmlWithCorrectedLinks.replace(Regex("""(import|init)\("([^"]+)"""")) { match ->
//      val attr = match.groupValues[1]
//      val path = match.groupValues[2]
//      val uri = resourceToUri(path)
//      """$attr("$uri""""
//    }

    // Temporary file creation for index.html
//    val tempFile = Files.createTempFile("index", ".html")
//    tempFile.write(rawHtml)
//
//    val browser = JBCefBrowser()
////    SwingUtilities.invokeLater {
////      val devTools = browser.cefClient.createDevTools()
////      devTools.setSize(800, 600)
////      devTools.setLocationRelativeTo(null)
////      devTools.isVisible = true
////    }
//    browser.loadURL(tempFile.toUri().toString())
//    LOG.info("Loading rawHtml: ${rawHtml}")
//
//    component = browser.component
  }

  private fun guessMimeType(fileName: String): String =
    URLConnection.guessContentTypeFromName(fileName) ?: "application/octet-stream"

  fun extractResourcePaths(html: String): List<String> {
    val pattern1 = Regex("""(?:src|href)=["'](.+?)["']""")
    val pattern2 = Regex("""(?:import|init)\(["'](.+?)["']""")

    return (pattern1.findAll(html) + pattern2.findAll(html))
        .map { it.groupValues[1] }
        .filter { !it.startsWith("http") && !it.startsWith("data:") }
        .map { it.removePrefix("./") }
        .distinct().toList()
  }

  private fun resourceToUri(relativePath: String): String {
    val normalized = if (relativePath.startsWith("./")) relativePath.substring(2) else relativePath
    return HtmlPanel::class.java.classLoader
      .getResource("html/$normalized")
      ?.toURI()
      ?.toString()
      ?: throw IllegalArgumentException("Ressource non trouvée: $relativePath")
  }

  private fun URL.readTextFromResource(): String =
    this.openStream().bufferedReader().use { it.readText() }

  private fun copyResourcesToTemp(paths: List<String>, tempDir: File) {
    val classLoader = HtmlPanel::class.java.classLoader
    for (relativePath in paths) {
      val stream = classLoader.getResourceAsStream("html/$relativePath")
        ?: throw IllegalStateException("Fichier manquant : $relativePath")
      val outFile = File(tempDir, relativePath)
      outFile.parentFile.mkdirs()
      stream.use { input -> outFile.outputStream().use { input.copyTo(it) } }
    }
  }
}