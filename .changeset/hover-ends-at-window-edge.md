---
'@gpuix/native': patch
---

Leaving the window ends hover. The `hover` style of the element the pointer last touched used to stay painted, and the cursor and tooltips stayed with it, until the pointer came back. `onMouseLeave` already fired. The test renderer gains `simulateMouseExit` and `nativeSimulateMouseExit`.
