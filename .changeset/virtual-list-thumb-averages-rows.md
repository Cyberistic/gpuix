---
'@gpuix/native': patch
---

The thumb of a `<virtual-list>` counts the rows the list has not measured yet as the average measured row, not as `estimatedItemHeight`. A list of tall rows with a short estimate no longer draws a thumb several times too long that shrinks as rows scroll into view; the thumb is right once a few rows have been measured.
