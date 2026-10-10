---
'@gpuix/react': patch
'@gpuix/native': patch
'@gpuix/solid': patch
---

A re-render no longer resends styles and custom props that did not change. React calls `commitUpdate` for every element a parent re-rendered, and the reconciler used to serialize and send each one's `style` every time, which cost a JSON round trip and a style intern per element per commit. It now compares values, so `style={{ ... }}` rebuilt with the same contents sends nothing. The custom prop diff is linear instead of quadratic in the number of props, and the event handler diff walks the props an element has instead of every event GPUIX knows.

A style change on an element React has hidden (Suspense, Activity) no longer makes it visible again: it keeps `visibility: hidden` until React unhides it.

`@gpuix/native/host` exports `sameMutationValue(left, right)` and `EVENT_TYPE_BY_PROP` for adapters that diff the same payloads.
