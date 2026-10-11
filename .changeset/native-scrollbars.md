---
'@gpuix/native': patch
---

`scrollbarColor` and `scrollbarWidth` paint a native scrollbar thumb over a container with `overflow: "scroll"`, on each axis that has more to show. The thumb follows the scroll offset in the frame it paints, with no JavaScript in the loop and no extra frame, and it takes no pointer input.
