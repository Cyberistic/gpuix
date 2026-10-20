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

Frames are rendered offscreen and shown with the kitty graphics protocol through
shared memory, so it works in kitty and Ghostty on the same machine. Mouse,
hover, scroll, keyboard with key-up, paste, focus and resize all reach the app.
ctrl-c quits, also from a focused text field. The terminal is restored on quit,
SIGTERM, SIGINT, a native panic and `process.exit()`. Output printed while the
app runs is shown after it exits; on Node, only for streams that had not printed
before `render()`.

`isTerminal()`, exported by `@gpuix/react` and `@gpuix/solid`, tells an app it is
in a terminal, so it can drop the space it reserves for the macOS traffic lights.
