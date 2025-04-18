package com.sybernatus

import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.ToolWindow
import com.intellij.openapi.wm.ToolWindowFactory
import com.intellij.ui.content.ContentFactory

class HtmlToolWindowFactory : ToolWindowFactory {
    override fun createToolWindowContent(project: Project, toolWindow: ToolWindow) {
        val htmlPanel = HtmlPanel()
        val content = ContentFactory.getInstance().createContent(htmlPanel.component, "Syb Mindmap", false)
        toolWindow.contentManager.addContent(content)
    }
}