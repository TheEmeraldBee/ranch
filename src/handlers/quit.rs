use ascii_forge::prelude::*;

use crate::app_data::AppData;

pub fn quit_handler(w: &mut Window, d: &mut AppData) {
    if event!(w, Event::Key(k) => k.code == KeyCode::Char('q')) {
        d.should_exit = true;
    }
}
