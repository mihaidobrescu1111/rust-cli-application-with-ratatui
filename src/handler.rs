use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // TODO: define actions for quitting the app
        // TODO: define actions for apps functionalities
        KeyCode::Delete => {
            app.running = false;
        }
        KeyCode::Down => {
            app.down().await;
        }
        KeyCode::Up => {
            app.up().await;
        }
        _ => {}
    }
    Ok(())
}
