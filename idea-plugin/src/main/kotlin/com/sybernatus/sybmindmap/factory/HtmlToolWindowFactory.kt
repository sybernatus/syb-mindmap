package com.sybernatus.sybmindmap.factory

import com.intellij.openapi.Disposable
import com.intellij.openapi.project.Project
import com.intellij.openapi.util.Disposer
import com.intellij.openapi.wm.ToolWindow
import com.intellij.openapi.wm.ToolWindowFactory
import com.intellij.ui.content.ContentFactory
import com.sybernatus.sybmindmap.HtmlPanel

class HtmlToolWindowFactory : ToolWindowFactory {
    private var listenerDisposable: Disposable = Disposer.newDisposable("MindmapFileWatcher")
    override fun createToolWindowContent(project: Project, toolWindow: ToolWindow) {
        val content = ContentFactory.getInstance().createContent(HtmlPanel(listenerDisposable), "Mindmap", false)
        Disposer.register(content, listenerDisposable)
        toolWindow.contentManager.removeAllContents(true)
        toolWindow.contentManager.addContent(content)
    }
}