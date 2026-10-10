---
'@gpuix/native': minor
'@gpuix/react': minor
'@gpuix/solid': minor
---

New styles for what gpui already draws and GPUIX did not expose:

- `aspectRatio` sizes the axis that has no definite size.
- `borderStyle: "dashed"` draws every border side dashed.
- `boxShadow` accepts an array of shadows, and each shadow takes `inset: true`.
- `alignItems` accepts `"stretch"` and `"baseline"`, which `alignSelf` already did.
- `justifyContent` accepts `"space-evenly"`, which `alignContent` already did.
