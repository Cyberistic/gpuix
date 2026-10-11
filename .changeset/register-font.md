---
'@gpuix/native': minor
'@gpuix/react': minor
'@gpuix/solid': minor
---

`registerFont(bytes)` loads a TrueType or OpenType font so `fontFamily` can name it. Text that asked for the family before it was registered picks the font up on the next frame. Fonts GPUI would abort on, or cannot use, are rejected with an error instead.
