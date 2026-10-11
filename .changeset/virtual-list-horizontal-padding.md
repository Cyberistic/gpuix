---
'@gpuix/native': patch
---

`padding` on a `<virtual-list>` now insets its rows on the left and right as well as the top and bottom. Rows used to span the list's full width and ignore left and right padding. A `scrollbarColor` thumb stays at the list's own edge, past the padding.
