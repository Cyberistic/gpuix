---
'@gpuix/native': patch
---

Subscribe before delivering the initial value in `observeSelectedText`, so a
synchronous selection change made by that callback is delivered too. Remove
the subscription if the initial read or callback throws, then propagate the
original error.

Fixes #98
