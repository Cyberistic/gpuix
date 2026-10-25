# Host capabilities

What a GPUIX app can ask the operating system to do, and what each renderer
supports. Everything here is a **command**: it changes host state and returns,
so none of it is a hook.

| Capability | JS API | macOS | Windows | Linux | Terminal | Browser |
|---|---|---|---|---|---|---|
| Write clipboard text | `copyText(text)`, `renderer.writeToClipboard(text)` | yes | yes | yes | OSC 52 | — |
| Read clipboard text | `renderer.readFromClipboard()` | yes | yes | yes | own writes only | — |
| Window title | `renderer.setWindowTitle(title)` | yes | yes | yes | OSC 2 | document title |
| Window controls | `renderer.minimizeWindow()`, `zoomWindow()`, `toggleFullscreen()` | yes | yes | yes | — | — |
| File picker | `renderer.promptForPaths(options)` | yes | yes | yes | rejects | rejects |
| Custom fonts | `renderer.registerFont(bytes)` | yes | yes | yes | — | — |
| Focus | `renderer.focusElement(id)`, `focusNext()`, `focusPrevious()` | yes | yes | yes | yes | yes |
| Text selection | `renderer.getSelectedText()`, `clearSelection()` | yes | yes | yes | yes | yes |
| Frame overlay | `renderer.setDebugFrameOverlay(mode)` | yes | yes | yes | yes | yes |

`—` means the method is absent on that renderer: a `?`-call does nothing, and
`copyText` throws, because a silent no-op would claim a copy that never
happened. A browser app uses `navigator.clipboard`, `FontFace`, and the
`document` title instead.

## Clipboard

Both clipboard calls are synchronous. GPUI's desktop platforms read and write
the OS clipboard on the calling thread, so a promise would only delay the
answer:

- **macOS** uses `NSPasteboard` directly.
- **Windows and Linux** run the call on the GPUI UI thread and hand the result
  back to JavaScript, the same path `cycleDebugFrameOverlay()` and the clock
  automation use. Only the JavaScript thread waits.
- **Terminal** writes OSC 52 and keeps a local copy, because a terminal has no
  protocol for reading the system clipboard. A read returns only text this app
  wrote.
- **Browser** has no clipboard binding yet.

### Empty, absent, and non-text content

- An empty string is written as an empty entry. It is not a no-op, and it reads
  back as `''`.
- `readFromClipboard()` returns `null` when the clipboard holds no text at all,
  including an empty clipboard, image-only content, and file lists. A file list
  is not turned into paths, because pasting `/tmp/photo.png` is not what the
  user copied.
- Only plain text is written. Metadata, multi-entry items, images, and files
  are not exposed.
