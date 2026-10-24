---
"@gpuix/native": minor
---

Add `rollbackUpdate()` to put back the version the last update replaced.
`downloadAndInstall()` now keeps that bundle at `<cache>/gpuix/updater/previous/app`
instead of a temp dir that vanished with the installing process. Windows
installers cannot be reversed in place, so there the call reports that the
previous version must be reinstalled.

Fixes #107
