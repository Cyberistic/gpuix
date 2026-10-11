---
'@gpuix/react': minor
---

`@gpuix/react` now uses `react-reconciler@^0.33.0` and peers on `react@^19.2.0`. React 19.2 pairs with reconciler 0.33, and the old `^0.31.0` pin forced every app on 19.2 to override it. The `^18.0.0` half of the old peer range was never true, since reconciler 0.31 already required React 19.
