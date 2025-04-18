package com.sybernatus

import com.intellij.openapi.diagnostic.Logger
import com.intellij.ui.jcef.JBCefApp
import com.intellij.ui.jcef.JBCefBrowser
import java.io.File
import java.net.URL
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
    val rawHtml = HtmlPanel::class.java.classLoader
      .getResource("html/index.html")
      ?.readTextFromResource()
      ?: throw IllegalStateException("index.html not found")

    val tempDir = Files.createTempDirectory("html-view").toFile()
    val resourcePaths = extractResourcePaths(rawHtml)
    copyResourcesToTemp(resourcePaths, tempDir)
    File(tempDir, "index.html").writeText(rawHtml)
    val indexFile = File(tempDir, "index.html")
    if (!indexFile.exists()) {
      throw IllegalStateException("index.html non copié dans le dossier temporaire.")
    }

    val browser = JBCefBrowser()
    browser.loadURL(indexFile.toURI().toString())

    // DevTools facultatif
    SwingUtilities.invokeLater {
      val devTools = browser.cefBrowser.devTools
              browser.openDevtools()
      val devToolsBrowser = JBCefBrowser.createBuilder()
        .setCefBrowser(devTools)
        .setClient(browser.jbCefClient)
        .build()
    }

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