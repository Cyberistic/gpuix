---
'@gpuix/native': patch
'@gpuix/react': patch
---

`collectGarbageWhenIdle()` runs one full garbage collection once the app has stopped committing for a while, so the heap of a long session comes back down after a burst of work. It watches the new `appliedBatchCount()` and the process's CPU, and returns a function that stops it.
