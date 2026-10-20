---
'@gpuix/native': patch
---

Return each element once from chained automation locators, in tree order, even
when the matching parent roots overlap. A single descendant under nested
containers no longer produces duplicate `all()` results, an inflated `count()`,
or a false ambiguity error from `element()` and actions.

Fixes #99
