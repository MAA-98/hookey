use std::{
    cell::{Cell, RefCell},
    io,
    rc::Rc,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use hookey::{
    actions::Action,
    buffer::Buffer,
    editor::Editor,
};

#[derive(Default)]
struct Snapshot {
    text: String,
    cursor_row: usize,
    cursor_col: usize,
}

/// The application state.
///
/// We keep the snapshot in `Rc<RefCell<_>>` so the editor hook can update it.
struct App {
    editor: Editor,
    snap: Rc<RefCell<Snapshot>>,
    dirty: Rc<Cell<bool>>,
}

impl App {
    fn new() -> Self {
        let mut editor = Editor::new(Buffer::from_str("Hello world\n How's it going?"));

        // Shared render snapshot.
        let snap = Rc::new(RefCell::new(Snapshot {
            text: editor.buffer().as_string(),
            cursor_row: 0,
            cursor_col: 0,
        }));

        // Shared redraw flag.
        let dirty = Rc::new(Cell::new(true));

        // Clone handles for the hook.
        let snap_for_hook = Rc::clone(&snap);
        let dirty_for_hook = Rc::clone(&dirty);

        // The hook updates the snapshot and marks the UI as dirty.
        // The main loop will notice and redraw.
        editor.set_post_action_hook(move |buffer, cursor| {
            let mut s = snap_for_hook.borrow_mut();

            // Update the rendered text.
            s.text = buffer.as_string();

            snap_for_hook.borrow_mut().text = buffer.as_string();
            dirty_for_hook.set(true);

            // For now, we don't calculate row/col.
            // Just store something simple so we can see cursor state changing.
            s.cursor_row = 0;
            s.cursor_col = cursor.char_index().raw();
        });

        Self { editor, snap, dirty }
    }

    /// Render the current snapshot.
    fn render(
        &self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Clone the text out before drawing so we don't hold a RefCell borrow
        // across the terminal render call.
        let text = self.snap.borrow().text.clone();

        terminal.draw(|frame| {
            let area = frame.area();

            let block = Block::default()
                .title("Hookey")
                .borders(Borders::ALL);

            let inner = block.inner(area);
            frame.render_widget(block, area);

            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, inner);
        })?;

        self.dirty.set(false);
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    // Initial render
    app.render(terminal)?;

    loop {
        if event::poll(Duration::from_millis(25))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Left => {
                        app.editor.apply_action(Action::MoveCursorLeft)?;
                    }

                    KeyCode::Right => {
                        app.editor.apply_action(Action::MoveCursorRight)?;
                    }

                    _ => {}
                }
            }
        }

        // Redraw if the hook marked the app dirty
        if app.dirty.get() {
            app.render(terminal)?;
        }
    }

    Ok(())
}