# hookey

Cursor and buffer execution layer over `ropey`, the popular Rust text buffer library.

## Planned Features

- Editor: persistent cursor and text buffer
- Actions: user-level intents for manipulating editor. Starting with primitive, then increasingly complex commands for text and cursor manipulation. Goal of including all vim style commands (or sane approximations thereof).
- Editor hooks for efficient event-based API
- Command-based undo/redo tree

## Out of scope

- Registers: actions can return data through hooks, API client decides how to handle.
- Command interpreter/DSL: API client calls typed actions. Custom actions can be built with a sequence of provided ones. 
- File reading/management
- Rendering

## Example App

Repository contains a `hookey-playground` terminal app using crossterm and ratatui.
It's not optimized as a full file editor, but shows how to use `hookey` and test out actions/commands.

## Current TODOs

- Flesh out `hookey-playground` to load file contents to buffer