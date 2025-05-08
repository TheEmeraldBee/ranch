use ascii_forge::prelude::*;

use crate::app_data::AppData;

pub fn clear(w: &mut Window, d: &mut AppData) {
    if event!(w, Event::Key(k) => k.code == KeyCode::Down || k.code == KeyCode::Char('c')) {
        d.cur_search = "".to_string();
        d.update_search();
    }
}

pub fn input_handler(w: &mut Window, d: &mut AppData) {
    let mut changed = false;
    for event in w.events() {
        if let Event::Key(KeyEvent { code: k, .. }) = event {
            if let KeyCode::Char(c) = k {
                d.cur_search.push(*c);
                changed = true;
            } else if *k == KeyCode::Backspace {
                d.cur_search.pop();
                changed = true;
            }
        }
    }
    if changed {
        d.update_search();
    }
}
