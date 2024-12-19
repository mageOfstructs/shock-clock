
package com.plugin.accessibility

import android.view.accessibility.AccessibilityEvent
import java.util.LinkedList

object AccessibilityEventManager {
    val eventQueue = LinkedList<AccessibilityEvent>()
}
