package com.sybernatus.sybmindmap

import com.intellij.openapi.Disposable
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.editor.EditorFactory
import com.intellij.openapi.editor.event.DocumentListener
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.openapi.vfs.isFile
import com.intellij.ui.jcef.JBCefApp
import com.intellij.ui.jcef.JBCefBrowser
import com.intellij.util.ui.HtmlPanel
import org.cef.browser.CefBrowser
import java.awt.BorderLayout
import javax.swing.JPanel


class HtmlPanel(private val listenerDisposable: Disposable) : JPanel(BorderLayout()) {

  private val LOG = Logger.getInstance(HtmlPanel::class.java)

  init {
    System.setProperty("ide.browser.jcef.debug.port", "9222")
    LOG.info("HtmlPanel initialized")
    require(JBCefApp.isSupported()) { "JCEF unsupported" }

    val browser = JBCefBrowser("http://local.plugin/index.html")

    browser.jbCefClient.addRequestHandler(HtmlResourcesRequestHandler(), browser.cefBrowser)
    postMessageToBrowserQueue(browser.cefBrowser)
    add(browser.component, BorderLayout.CENTER)
  }


  // function that will listen JSON & YAML file to post the message to browser queue
  private fun postMessageToBrowserQueue(browser: CefBrowser?) {
    EditorFactory.getInstance().eventMulticaster.addDocumentListener( object: DocumentListener {
      override fun documentChanged(event: com.intellij.openapi.editor.event.DocumentEvent) {
        println("[PLUGIN] Document changed: ${event.document}")
        // if the file is JSON or YAML
        val document = event.document
        val file = FileDocumentManager.getInstance().getFile(document);
        val documentText = document.text


        // if text contains IntellijIdeaRulezzz skip
        if (documentText.contains("IntellijIdeaRulezzz")) {
          println("[PLUGIN] IntellijIdeaRulezzz found, skipping")
          return
        }
        
        val documentTextEscaped = escapeForJavaScript(documentText)
        println("[PLUGIN] YAML document changed: $documentTextEscaped")

        if (file != null && file.isFile && isDocumentJson(file)) {

          val jsonString = escapeForJavaScript(documentTextEscaped)

          val jsCode = java.lang.String.format(
            "window.postMessage({type: 'JSON', content: '%s'}, '*');",
            jsonString
          )
          browser?.executeJavaScript(
            jsCode,
            browser.url,
            0
          );
        } else if (file != null && file.isFile && isDocumentYaml(file)) {
          val jsCode = java.lang.String.format(
            "window.postMessage({type: 'YAML', content: '%s'}, '*');",
            documentTextEscaped
          )
          browser?.executeJavaScript(
            jsCode,
            browser.url,
            0
          );
        }
      }

    }, listenerDisposable)
  }

  private fun escapeForJavaScript(s: String): String {
    return s
      .replace("\\", "\\\\")
      .replace("\"", "\\\"")
      .replace("\n", "\\n")
      .replace("\r", "")
      .replace("\t", "\\t")
  }

  private fun isDocumentJson(file: VirtualFile): Boolean {
    val extension = file.extension
    return extension != null
            && extension.equals("json", true)
  }

  private fun isDocumentYaml(file: VirtualFile): Boolean {
    val extension = file.extension
    return extension != null
            && extension.equals("yaml", true)
            || extension.equals("yml", true)
  }
}