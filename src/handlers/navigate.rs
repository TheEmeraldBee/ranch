use ascii_forge::prelude::*;

use crate::app_data::{AppData, AppState};

pub fn to_other(w: &mut Window, d: &mut AppData) {
    for event in w.events() {
        let Event::Key(k) = event else { continue };
        let Some(comb) = d.combiner.transform(k.clone()) else {
            continue;
        };

        if d.config.binds.insert.iter().any(|x| *x == comb) {
            d.state = AppState::Search
        } else if d.config.binds.logs.iter().any(|x| *x == comb) {
            d.state = AppState::Logs
        }
    }
}

pub fn up(w: &mut Window, d: &mut AppData) {
    for event in w.events() {
        let Event::Key(k) = event else { continue };
        let Some(comb) = d.combiner.transform(k.clone()) else {
            continue;
        };
        if d.config.binds.normal.iter().any(|x| *x == comb) {
            d.state.up();
        }
    }
}
