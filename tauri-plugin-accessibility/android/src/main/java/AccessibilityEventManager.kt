
package com.plugin.accessibility.kt

import android.view.accessibility.AccessibilityEvent
import java.util.LinkedList

object AccessibilityEventManager {
    val eventQueue = LinkedList<AccessibilityEvent>()
}
