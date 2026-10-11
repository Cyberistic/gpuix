---
'disktree': patch
---

Use Explorer on Windows, Finder on macOS, and `xdg-open` on Linux for the
selection panel's file-manager action. The button names the platform's file
manager, and launch errors appear in the panel instead of crashing the app.
On Linux, selecting a file opens its containing folder.

Fixes #75
