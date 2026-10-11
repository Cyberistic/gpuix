---
'disktree': patch
'@gpuix/cli': patch
---

Fix source builds on Windows by replacing the shell `chmod` command with
Node.js file permissions on Unix. Both CLI entry points remain executable
on macOS and Linux; Windows no longer needs an extra Unix tool.

Fixes #74
