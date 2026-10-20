---
'@gpuix/native': patch
---

Return independent window measurement snapshots, including separate safe-area,
IME, and effective insets. Mutating a result from `readWindowSize` or
`readWindowInsets` no longer changes later reads, other inset fields, or a
custom host's own values. The React and Solid window primitives share this fix.

Fixes #97
