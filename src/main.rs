use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    // let app = 

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // TODO:  the terminal user interface
    // let mut tui =
    
    // TODO: init the terminal

    // Start the main loop.
    // while app.running {
        // TODO: Render the user interface.

        // TODO: Handle events.
        
    // }

    // TODO: Reset the terminal if the app has been terminated

    Ok(())
}
