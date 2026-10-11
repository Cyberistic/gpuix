---
'@gpuix/native': patch
---

`scrollbarColor` and `scrollbarWidth` now also draw a thumb on a `<virtual-list>`. It is painted over the list in the frame the list scrolls, from the heights the list has measured, so it never lags the scroll position and costs no frame of its own. The list's layout is unchanged.
