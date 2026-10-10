---
'@gpuix/native': patch
---

A box with a `scrollbarColor` and a `<virtual-list>` as a direct child draws the list's thumb at the box's own edge, past the box's padding, where a browser puts the scrollbar of a padded scroller. The list itself still draws its own thumb when it is the one that has the colour.
