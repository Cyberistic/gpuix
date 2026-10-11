---
'@gpuix/native': patch
---

Document Windows ARM64 source-build prerequisites, including Visual Studio's
ARM64 MSVC and Clang tools, the Windows SDK, CMake, and matching ARM64 Node.js
and Bun runtimes. Add PowerShell commands for finding Clang and building the
native addon and adapters. Published Windows binaries and CI remain x64-only.

Fixes #73
