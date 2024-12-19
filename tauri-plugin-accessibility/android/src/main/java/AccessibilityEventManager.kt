package com.plugin.accessibility.kt

import android.accessibilityservice.AccessibilityService
import android.view.accessibility.AccessibilityEvent
import java.util.LinkedList

object AccessibilityEventManager {
    val eventQueue = LinkedList<AccessibilityEvent>()
    var accessibilityService: AccessibilityService? = null

    fun registerService(service: AccessibilityService) {
        accessibilityService = service
    }

    fun unregisterService() {
        accessibilityService = null
    }
}

