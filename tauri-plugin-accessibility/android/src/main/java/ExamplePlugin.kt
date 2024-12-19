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
import android.view.accessibility.AccessibilityEvent

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

        val event = AccessibilityEventManager.eventQueue.poll()
        val rootNode: AccessibilityNodeInfo? = event?.source
        if (event == null || rootNode == null) {
            ret.put("text", "")
            ret.put("package", "")
            ret.put("event_type", "")
        } else {
            val stringBuilder = StringBuilder()
            traverseAccessibilityTree(rootNode, stringBuilder)
            val text = stringBuilder.toString()
            ret.put("text", text)
            ret.put("package", event.packageName ?: "")
            ret.put("event_type", AccessibilityEvent.eventTypeToString(event.eventType))

        }
        invoke.resolve(ret)
    }

    private fun traverseAccessibilityTree(node: AccessibilityNodeInfo?, stringBuilder: StringBuilder) {
        node?.let {
            val text = it.text
            if (!text.isNullOrEmpty()) {
                stringBuilder.append(text)
            }

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

