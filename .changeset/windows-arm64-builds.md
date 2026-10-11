---
'@gpuix/native': patch
'@gpuix/examples': patch
---

Add Windows ARM64 builds alongside x64. CI uses a native `windows-11-arm`
runner, configures Visual Studio's ARM64 Clang, and runs the existing Windows
binding, renderer, React, Solid, and example checks on both architectures.
Fix the shared paint-report import so native builds compile on Windows and Linux.
The release pipeline publishes `@gpuix/native-win32-arm64-msvc` and includes
`example-chat-aarch64-pc-windows-msvc.exe` in the chat release assets.

Use ARM64 Node.js or Bun to load the ARM64 addon. An x64 runtime on an ARM64
machine still selects the x64 package. The README includes ARM64 source-build
prerequisites and PowerShell commands for local development.

Fixes #73

Fixes #78
