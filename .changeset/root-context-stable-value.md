---
'@gpuix/react': patch
---

`root.render()` no longer re-renders every `useGpuix()` consumer. The root's context value was a new object on each call, so calling `render()` again (a hot reload, a test, an app that renders the root with new props) counted as a context change and re-rendered each consumer and, through its new callbacks, every `memo` row below it. In the chat example this was 305 row renders and about 9 ms per keystroke of a find bar; it is now 0.2 ms.
