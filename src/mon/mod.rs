mod app;
mod data;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{App, Overlay, Tab};

pub fn run(verbose: bool) {
    if verbose {
        println!("[verbose] Launching TUI monitor...");
    }

    if let Err(e) = run_tui() {
        eprintln!("lab mon failed: {e}");
    }
}

fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // If we panic mid-draw, restore the terminal instead of leaving the
    // user's shell stuck in raw mode / the alternate screen.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        default_hook(info);
    }));

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let result = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key(app, key.code);
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }

        app.on_tick();
    }
}

fn handle_key(app: &mut App, code: KeyCode) {
    if app.overlay.is_some() {
        match code {
            KeyCode::Esc | KeyCode::Char('q') => app.overlay = None,
            KeyCode::Char('h') | KeyCode::Char('?') => toggle_overlay(app, Overlay::Help),
            KeyCode::Char('g') => toggle_overlay(app, Overlay::Guide),
            _ => {}
        }
        return;
    }

    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Char('r') => app.force_refresh_active(),
        KeyCode::Char('h') | KeyCode::Char('?') => toggle_overlay(app, Overlay::Help),
        KeyCode::Char('g') => toggle_overlay(app, Overlay::Guide),
        KeyCode::Char('1') => app.set_tab(Tab::Overview),
        KeyCode::Char('2') => app.set_tab(Tab::Stats),
        KeyCode::Char('3') => app.set_tab(Tab::Disk),
        KeyCode::Char('4') => app.set_tab(Tab::Network),
        KeyCode::Char('5') => app.set_tab(Tab::Processes),
        _ => {}
    }
}

fn toggle_overlay(app: &mut App, overlay: Overlay) {
    app.overlay = if app.overlay == Some(overlay) { None } else { Some(overlay) };
}
