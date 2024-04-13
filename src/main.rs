use ratatui_templates::app::{App, AppResult};
use ratatui_templates::connection::get_temperature;
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app: App = App::new().await;


    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events: EventHandler = EventHandler::new(250);
    // TODO:  the terminal user interface
    let mut tui = Tui::new(terminal, events);
    // TODO: init the terminal
    tui.init();
    // Start the main loop.
    while app.running {
        // TODO: Render the user interface.
        tui.draw(&mut app);

        app.get_temp();
        // TODO: Handle events.
        match tui.events.next().await {
            Ok(event) => {
                match event {
                    Event::Key(key) => {
                        handle_key_events(key, &mut app).await;
                    }
                    Event::Mouse(mouse) => {

                    }
                    Event::Tick => {

                    }
                    Event::Resize(_, _) => {

                    }
                }
            }
            Err(error) => {

            }
        }
    }

    // TODO: Reset the terminal if the app has been terminated
    tui.exit();
    Ok(())
}
