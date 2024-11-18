use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::Color,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::app::App;

pub fn run(app: &mut App) -> Result<(), io::Error> {
    // Setup terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Enter alternate screen and enable raw mode
    enable_raw_mode()?;
    terminal.clear()?;

    // Main event loop
    loop {
        // Draw the UI
        terminal.draw(|f| draw_main_ui(f, app))?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break, // Quit on 'q'
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                _ => {}
            }
        }
    }

    // Restore the terminal state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw_main_ui<B: Backend>(frame: &mut tui::Frame<B>, app: &mut App) {
    let main_border = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title("Setup - Project Manager");

    frame.render_widget(main_border, frame.size());

    let working_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .margin(1)
        .split(frame.size());

    let left_block = app.projects.projects_as_widget_list();
    // let left_block = Block::default()
    //     .border_style(Style::default().fg(tui::style::Color::Magenta))
    //     .borders(Borders::all())
    //     .title("Projects");
    let right_block = Block::default().borders(Borders::all());

    frame.render_stateful_widget(left_block, working_chunk[0], &mut app.state);
    frame.render_widget(right_block, working_chunk[1]);
}
