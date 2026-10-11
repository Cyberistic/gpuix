---
'@gpuix/react': minor
---

Added `createPortal(children, target)`, which mounts children into an element that is already mounted. Take `target` from a `ref`. The portal's children are appended to the target and kept in key order, so a toast layer, a drag preview or an overlay root can live in another part of the tree than the component that renders it. See "Portals" in the README.
