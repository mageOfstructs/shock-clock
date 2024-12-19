package com.plugin.accessibility

import android.app.Activity
import android.view.accessibility.AccessibilityNodeInfo
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import org.json.JSONObject

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity) : Plugin(activity) {
    private val implementation = Example()

    @Command
    fun getEvent(invoke: Invoke) {
        val ret = JSObject()
        println("yüah")

        // Get the first event from the shared queue
        val event = AccessibilityEventManager.eventQueue.poll()

        val rootNode: AccessibilityNodeInfo? = event?.source
        val stringBuilder = StringBuilder()

        // Traverse the accessibility tree to gather text content
        traverseAccessibilityTree(rootNode, stringBuilder)

        val text = stringBuilder.toString()
        if (text == "") return
        val test = JSONObject()
        test.put("text", "weoih")
        println(test)

        ret.put("text", "marp")
        println(ret)

        invoke.resolve(ret)
    }

    private fun traverseAccessibilityTree(node: AccessibilityNodeInfo?, stringBuilder: StringBuilder) {
        node?.let {
            // Get the text from the node, ensuring it's not null
            val text = it.text
            if (!text.isNullOrEmpty()) {
                stringBuilder.append(text)
            }

            // Traverse all child nodes
            for (i in 0 until it.childCount) {
                val childNode = it.getChild(i)
                childNode?.let { traverseAccessibilityTree(it, stringBuilder) }
            }
        }
    }

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }
}

