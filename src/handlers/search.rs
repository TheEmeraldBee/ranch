use ascii_forge::prelude::*;

use crate::app_data::AppData;

pub fn clear(w: &mut Window, d: &mut AppData) {
    for event in w.events() {
        let Event::Key(k) = event else { continue };
        let Some(comb) = d.combiner.transform(k.clone()) else {
            continue;
        };
        if d.config.binds.clear.iter().any(|x| *x == comb) {
            d.cur_search = "".to_string();
            d.update_search();
        }
    }
}

pub fn input_handler(w: &mut Window, d: &mut AppData) {
    let mut changed = false;
    for event in w.events() {
        if let Event::Key(KeyEvent { code: k, kind, .. }) = event {
            if kind.is_release() {
                continue;
            }
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
