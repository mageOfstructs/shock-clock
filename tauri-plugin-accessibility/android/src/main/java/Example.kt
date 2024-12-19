package com.plugin.accessibility

import android.accessibilityservice.AccessibilityService
import android.view.accessibility.AccessibilityEvent
import android.util.Log
import com.plugin.accessibility.kt.AccessibilityEventManager

class Example : AccessibilityService() {

    override fun onCreate() {
        super.onCreate()
        AccessibilityEventManager.registerService(this)
    }

    override fun onDestroy() {
        super.onDestroy()
        AccessibilityEventManager.unregisterService()
    }

    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent) {
        AccessibilityEventManager.eventQueue.add(event)
    }

    override fun onInterrupt() {
        // Handle service interruption if needed
    }

    fun goToHomeScreen() {
        Log.i("Example", "Performing global action: HOME")
        performGlobalAction(GLOBAL_ACTION_HOME)
    }
}
