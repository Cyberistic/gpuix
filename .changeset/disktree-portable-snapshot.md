---
'disktree': patch
---

Fix the scan/navigation snapshot on Windows by normalizing the actual displayed
fixture path instead of assuming Unix path separators. The test still checks
the same scan totals and selection panel contents.

Fixes #76
