use ascii_forge::prelude::*;

use crate::app_data::{AppData, AppState};

pub fn to_other(w: &mut Window, d: &mut AppData) {
    if event!(w, Event::Key(k) => k.code == KeyCode::Char('i')) {
        d.state = AppState::Search
    } else if event!(w, Event::Key(k) => k.code == KeyCode::Char('l')) {
        d.state = AppState::Logs
    }
}

pub fn up(w: &mut Window, d: &mut AppData) {
    if event!(w, Event::Key(k) => k.code == KeyCode::Esc) {
        d.state.up();
    }
}
