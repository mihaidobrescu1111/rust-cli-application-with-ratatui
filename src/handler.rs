use crate::app::{App, AppResult};
use crossterm::event::KeyEvent;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // TODO: define actions for quitting the app
        // TODO: define actions for apps functionalities
        _ => {}
    }
    Ok(())
}
