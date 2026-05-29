# Hookey

Execution layer over `ropey`, the popular Rust text buffer library.

## Scope

- Persistent cursor
- Hooks over common text structures to query or modify.
- Primitive and increasingly complex commands for text and cursor manipulation, goal of including all nvim style commands (or sane approximations thereof).
- Command-based undo/redo tree

Out of scope:
- Registers: commands are either destructive (do not return values) or return values and let library consumers decide what to do with it.
- Command interpreter/DSL: Consumers have to call the APIs typed commands or hooks. 
- File reading/management
- Rendering

## Architecture

- Execution is through an action queue with inversion of control; actions on the editor (buffer+cursor) are added to a FIFO queue, which executes without explicit call. 
- Optimizations like parallel execution on subsections can be tried.