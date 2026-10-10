---
'@gpuix/native': patch
---

`<div>` and `<text>` no longer add a hidden child element to record their bounds. Each host element used to carry an extra `canvas` node, which doubled the nodes gpui laid out, prepainted and painted every frame. The bounds now come from the element's own paint, so a frame over the same tree is about 15% cheaper. An element with `visibility: hidden` keeps the child, because gpui does not paint a hidden element and its layout box must stay readable.

`getElementBounds()`, automation locators and text selection regions now use the element's border box. They used the padding box before, which was narrower by the border width on an element with a `borderWidth`.
