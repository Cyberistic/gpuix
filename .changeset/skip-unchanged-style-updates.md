---
'@gpuix/react': patch
'@gpuix/native': patch
---

A re-render no longer resends styles and custom props that did not change. React calls `commitUpdate` for every element a parent re-rendered, and the reconciler used to serialize and send each one's `style` every time, which cost a JSON round trip and a style intern per element per commit. It now compares values, so `style={{ ... }}` rebuilt with the same contents sends nothing. The custom prop diff is also linear instead of quadratic in the number of props.

A style change on an element React has hidden (Suspense, Activity) no longer makes it visible again: it keeps `visibility: hidden` until React unhides it.

`@gpuix/native/host` exports `sameMutationValue(left, right)` for adapters that diff the same payloads.
