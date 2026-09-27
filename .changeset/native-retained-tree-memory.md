---
'@gpuix/native': patch
'@gpuix/react': patch
'@gpuix/solid': patch
---

Reduce memory retained by large UI trees. Host elements without listeners or custom props no longer carry two empty hash maps; the maps are allocated only for nodes that use them and released when the last listener or prop is removed. A real 75,000-element chat uses 33.6 MB of Rust tree heap instead of 44.5 MB, with unchanged element behavior.
