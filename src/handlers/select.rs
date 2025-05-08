use ascii_forge::prelude::*;

use crate::app_data::AppData;

pub fn select(w: &mut Window, d: &mut AppData) {
    if event!(w, Event::Key(k) => k.code == KeyCode::Down || k.code == KeyCode::Char('j')) {
        d.select(1);
    }
    if event!(w, Event::Key(k) => k.code == KeyCode::Up || k.code == KeyCode::Char('k')) {
        d.select(-1);
    }
}
