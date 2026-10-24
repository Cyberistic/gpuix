---
"@gpuix/native": minor
"@gpuix/solid": minor
---

Crates that link `gpuix-native` can add their own native elements with
`register_custom_element`. The `CustomElement` and `CustomElementFactory`
traits, `CustomRenderContext`, `GpuixView`, `custom_surface`,
`wire_standard_events`, `emit_event_full` and `EventCallback` are public, and
`gpuix_native::gpui` re-exports the GPUI the renderer uses. Names must contain a
`-`; built-in and duplicate names are refused. `@gpuix/solid` accepts those
hyphenated tags instead of throwing.

Fixes #110
