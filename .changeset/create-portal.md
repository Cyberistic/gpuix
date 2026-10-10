---
'@gpuix/react': minor
---

Added `createPortal(children, target)`, which mounts children into an element that is already mounted. Take `target` from a `ref`. The portal's children are appended to the target and kept in key order, so a toast layer, a drag preview or an overlay root no longer needs a patched host config. See "Portals" in the README.
