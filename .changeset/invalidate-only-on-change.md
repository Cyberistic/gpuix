---
'@gpuix/native': patch
---

`applyBatch` asks the window to repaint only when the batch changed something a frame would draw. A batch that only resends the style, text, listeners or custom props an element already has no longer rebuilds and re-lays-out the whole element tree for a frame that comes out identical.
