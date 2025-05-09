use ascii_forge::prelude::*;

use crate::app_data::AppData;

pub fn run(w: &mut Window, d: &mut AppData) {
    for event in w.events() {
        let Event::Key(k) = event else { continue };
        let Some(comb) = d.combiner.transform(k.clone()) else {
            continue;
        };
        if d.config.binds.run.iter().any(|x| *x == comb) {
            d.run(w);
            return;
        }
    }
}
