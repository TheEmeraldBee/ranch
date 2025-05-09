use ascii_forge::prelude::*;

use crate::{app_data::AppData, config::Entry};

pub fn select(w: &mut Window, d: &mut AppData) {
    for event in w.events() {
        let Event::Key(k) = event else { continue };
        let Some(comb) = d.combiner.transform(k.clone()) else {
            continue;
        };

        if d.config.binds.down.iter().any(|x| *x == comb) {
            d.select(1);
        }
        if d.config.binds.up.iter().any(|x| *x == comb) {
            d.select(-1);
        }

        if d.config.binds.enter.iter().any(|x| *x == comb) {
            let path = d.cur_items[d.selected].clone();
            if let Entry::Entry(_) = d.config.get_entry(path.0, path.1) {
                return;
            }
            d.path.push(d.cur_items[d.selected].1);
            d.cur_search.clear();
            d.update_search();
        }

        if d.config.binds.exit.iter().any(|x| *x == comb) {
            let old_selection = d.path.pop();
            d.cur_search.clear();
            d.update_search();
            if let Some(s) = old_selection {
                d.selected = d
                    .cur_items
                    .iter()
                    .enumerate()
                    .find_map(|(idx, x)| if x.1 == s { Some(idx) } else { None })
                    .unwrap_or(0);
            }
        }
    }
}
