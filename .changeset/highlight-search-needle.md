---
'@gpuix/native': patch
---

Typing in a find-in-page query no longer rebuilds the search for every block of text. The query is lowercased and compiled once per keystroke and shared by all the text it is matched against, so each keystroke over a long document costs a few percent less CPU. Matches stay leftmost, non-overlapping, and case-insensitive by default.

Fix Unicode matching in both `findRanges` and native highlights. Exact uppercase
Greek words match correctly, partial lowercase expansions such as `İ` / `i`
cover the complete original character, and ranges keep their original offsets.
Whole-word searches no longer treat combining marks as word boundaries, so
`cafe` does not match the prefix of `cafe\u0301` with `wholeWord: true`.
Matching still uses Unicode lowercasing, not accent removal or full case folding.

Fixes #95

Fixes #96
