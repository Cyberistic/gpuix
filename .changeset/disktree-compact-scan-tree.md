---
'disktree': patch
---

Reduce memory retained after large scans by keeping directory-only data off file tiles. On a 350,000-file scan, the retained Bun tree heap fell from about 32 MiB to 24 MiB without changing scan totals or the treemap.
