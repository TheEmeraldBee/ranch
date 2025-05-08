use ascii_forge::{prelude::*, widgets::border::Border};

use crate::app_data::AppData;

pub fn render_logs(w: &mut Window, d: &mut AppData) {
    let log_border = Border::square(w.size().x - 10, w.size().y - 2);
    render!(w, (5, 1) => [ log_border ]);
    for (i, log) in d.log.iter().enumerate() {
        render!(w, (6, i as u16 + 2) => [ log ]);
    }
}
