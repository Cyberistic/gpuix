---
'@gpuix/native': minor
'@gpuix/react': minor
'@gpuix/solid': minor
---

Add `terminal: true` to render an app inside the current terminal instead of a
native window, on macOS and Linux. `GPUIX_TERMINAL=1` turns it on without a code
change.

```tsx
render(<App />, { terminal: true })
```

Frames are rendered offscreen and shown with the kitty graphics protocol, in kitty
and Ghostty. On the same machine they go through shared memory. Over SSH, or when
the terminal cannot open the shared memory, each frame is sent inline, zlib
compressed; at most two frames wait in the link, so input stays responsive. Mouse,
hover, scroll, keyboard with key-up, paste, focus and resize all reach the app.
ctrl-c quits, also from a focused text field. The terminal is restored on quit,
SIGTERM, SIGINT, a native panic and `process.exit()`. Output printed while the
app runs is shown after it exits; on Node, only for streams that had not printed
before `render()`.

`isTerminal()`, exported by `@gpuix/react` and `@gpuix/solid`, tells an app it is
in a terminal, so it can drop the space it reserves for the macOS traffic lights.
