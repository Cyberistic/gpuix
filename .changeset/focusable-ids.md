---
'@gpuix/native': patch
---

Every frame no longer walks every element to find the ones that take focus. The tree now keeps that set up to date as props and listeners change, so an idle frame of a tree with thousands of elements costs about a third less CPU. Focus, tab order, `autoFocus`, and `onKeyDown`/`onFocus`/`onBlur` behave as before.
