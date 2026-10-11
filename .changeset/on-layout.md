---
'@gpuix/native': minor
'@gpuix/react': minor
'@gpuix/solid': minor
---

`onLayout` on `<div>` and `<text>` reports the element's border box after its first paint and each time its size changes, with no polling of `getElementBounds`. The event payload gains `width` and `height`.

`visibility: "hidden"` elements have never reported bounds, because gpui paints nothing for them and nothing inside them. The canvas fallback that claimed otherwise could not run and is gone.
