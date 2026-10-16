---
'@gpuix/native': patch
'@gpuix/react': patch
'@gpuix/solid': patch
---

Pace macOS frames with GPUI's display link instead of an independent 8 ms timer.
The pump stays non-blocking, so Bun and Node.js I/O can run between frames.
Wakes coalesce while JavaScript is busy. A 100 ms fallback handles hidden or
occluded windows; input then waits for that fallback. This does not eliminate
idle CPU use while a visible window's display link is running.

`startFrameLoop(renderer, { frameMs })` keeps explicit timer polling.
`renderer.setFrameWakeCallback(callback | null)` exposes the native wake for
manual pumps and returns whether the platform supports it.

The GPUI hook lives in the GPUIX fork. The upstream embedding discussion is
https://github.com/zed-industries/zed/pull/63077 (closed as a stale draft).

Fixes #91
