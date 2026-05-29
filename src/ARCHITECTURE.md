# Architecture

```txt
  Buffer + Cursor
         ↑
      Editor → Hooks
         ↑
Vec<Editor::Operation>
         ↑
      Actions
```
