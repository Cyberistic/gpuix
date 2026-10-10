---
'@gpuix/native': patch
---

Typing in a find-in-page query no longer rebuilds the search for every block of text. The query is lowercased and compiled once per keystroke and shared by all the text it is matched against, so each keystroke in a long chat costs a few percent less CPU. Matches are the same as before: leftmost, non-overlapping, case-insensitive by default.
