package com.sybernatus

import com.intellij.openapi.diagnostic.Logger
import com.intellij.ui.jcef.JBCefApp
import com.intellij.ui.jcef.JBCefBrowser
import com.intellij.util.ui.HtmlPanel
import com.sybernatus.sybmindmap.HtmlResourcesRequestHandler
import java.awt.BorderLayout
import javax.swing.JPanel


class HtmlPanel : JPanel(BorderLayout()) {

  private val LOG = Logger.getInstance(HtmlPanel::class.java)
  init {
    System.setProperty("ide.browser.jcef.debug.port", "9222")
    LOG.info("HtmlPanel initialized")
    require(JBCefApp.isSupported()) { "JCEF unsupported" }

    val browser = JBCefBrowser("http://local.plugin/index.html")

    browser.jbCefClient.addRequestHandler(HtmlResourcesRequestHandler(), browser.cefBrowser)
    add(browser.component, BorderLayout.CENTER)
  }
}