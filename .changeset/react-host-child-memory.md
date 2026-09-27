---
'@gpuix/react': patch
---

Reduce React renderer memory for large trees by releasing temporary child-placement lists after each host node mounts. A real 5,000-turn chat uses about 2 MiB less retained JavaScript heap, with no change to rendering or the mutation protocol.
