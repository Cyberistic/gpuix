---
'@gpuix/native': patch
---

`getElementsAt(x, y)` lists the elements under a window point, topmost first, like the DOM's `elementsFromPoint`. It clips at `overflow: hidden | scroll`, skips `pointer-events: none`, and puts a deferred `<anchored>` layer above the rest. The test renderer has it too.
