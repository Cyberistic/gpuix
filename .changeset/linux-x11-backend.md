---
'@gpuix/native': patch
---

Fix Linux apps silently opening no window under X11. Enable GPUI's Wayland and
X11 compositor-selection features alongside the platform backends, including
in release builds without `test-support`. No application changes are needed.

The upstream search found PR #56322, which fixes the inverse feature mismatch
in GPUI examples, not this addon's disabled compositor selection:
https://github.com/zed-industries/zed/pull/56322

Fixes #93
