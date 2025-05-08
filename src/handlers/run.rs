use ascii_forge::prelude::*;

use crate::app_data::AppData;

pub fn run(w: &mut Window, d: &mut AppData) {
    if event!(w, Event::Key(k) => k.code == KeyCode::Enter) {
        d.run(w);
    }
}
