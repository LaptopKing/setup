mod app;
mod ui;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the application state
    let mut app = App::new();

    // Start the TUI
    ui::run(&mut app)?;

    Ok(())
}
