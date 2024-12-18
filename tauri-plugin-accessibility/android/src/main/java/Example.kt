package com.plugin.accessibility

import android.accessibilityservice.AccessibilityService
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityNodeInfo
import java.util.concurrent.ConcurrentLinkedQueue
import android.util.Log

class Example : AccessibilityService() {
    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent) {
        // Check if the event has a source node and handle only relevant events
        val rootNode: AccessibilityNodeInfo? = event.source
        if (rootNode == null) {
            // If no source node is found, return early
            Log.d("AccessibilityService", "No source node found for event: ${event.eventType}")
            return
        }

        traverseAccessibilityTree(rootNode)
    }

    private fun traverseAccessibilityTree(node: AccessibilityNodeInfo?) {
        node?.let {
            // Get the text from the node, ensuring it's not null
            val text = it.text
            // println(text)
            if (!text.isNullOrEmpty()) {
                println(text)
            }

            // Recursively explore all child nodes, ensuring child nodes are not null
            for (i in 0 until it.childCount) {
                val childNode = it.getChild(i)
                childNode?.let { traverseAccessibilityTree(it) } // Call recursively on child node
            }
        }
    }

    override fun onInterrupt() {
        // Handle service interruption if needed
    }
}
