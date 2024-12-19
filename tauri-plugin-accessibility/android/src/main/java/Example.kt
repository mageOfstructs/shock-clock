package com.plugin.accessibility

import android.accessibilityservice.AccessibilityService
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityNodeInfo
import android.util.Log

class Example : AccessibilityService() {

    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent) {
        // Add event to shared queue
        AccessibilityEventManager.eventQueue.add(event)
    }

    override fun onInterrupt() {
        // Handle service interruption if needed
    }

    fun goToHomeScreen() {
        println("Yeah I did this")
        performGlobalAction(GLOBAL_ACTION_HOME)
    }
}

