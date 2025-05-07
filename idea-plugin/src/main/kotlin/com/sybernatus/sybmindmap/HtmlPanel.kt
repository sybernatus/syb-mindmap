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
import com.sybernatus.sybmindmap.enums.MessageType
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

        if (file != null && file.isFile) {
          publishFileContent(file, documentTextEscaped, browser)
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

  private fun publishFileContent(file: VirtualFile, documentTextEscaped: String, browser: CefBrowser?) {
    fun send(messageType: String, content: String) {
      val jsCode = java.lang.String.format(
        "window.postMessage({type: '%s', content: '%s'}, '*');",
        messageType,
        content
      )
      browser?.executeJavaScript(
        jsCode,
        browser.url,
        0
      )
    }

    when {
      file.extension.equals("json", true) -> {
        println("[PLUGIN] JSON file changed: $documentTextEscaped")
        val jsonString = escapeForJavaScript(documentTextEscaped)
        send(MessageType.JSON.name, jsonString)
      }
      file.extension.equals("yaml", true)
              || file.extension.equals("yml", true) -> {
        println("[PLUGIN] YAML file changed: $documentTextEscaped")
        send(MessageType.YAML.name, documentTextEscaped)
      }
      else -> {
        println("[PLUGIN] Unknown file type: ${file.extension}")
      }
    }
  }
}