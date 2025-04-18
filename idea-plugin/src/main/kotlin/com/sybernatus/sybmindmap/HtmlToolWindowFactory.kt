package com.sybernatus

import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.ToolWindow
import com.intellij.openapi.wm.ToolWindowFactory
import com.intellij.ui.content.ContentFactory

class HtmlToolWindowFactory : ToolWindowFactory {
    override fun createToolWindowContent(project: Project, toolWindow: ToolWindow) {
        val contentManager = toolWindow.contentManager
        contentManager.removeAllContents(true)

        if (contentManager.contentCount == 0) {
            val htmlPanel = HtmlPanel()
            val content = ContentFactory.getInstance().createContent(htmlPanel.component, "My HTML", false)
            contentManager.addContent(content)
        } else {
            println("[PLUGIN] ToolWindow déjà initialisé, aucun ajout répété.")
        }
    }
}