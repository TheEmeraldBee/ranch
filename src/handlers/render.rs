use ascii_forge::{prelude::*, widgets::border::Border};

use crate::app_data::{AppData, AppState};

pub fn render(w: &mut Window, d: &mut AppData) {
    let max_elements = w.size().y - 13;
    d.update_scroll(max_elements.min(d.config.max_render).into());

    let input_border = Border::square(w.size().x / 2, 3);
    let list_border = Border::square(w.size().x / 2, w.size().y - 10);

    let left_x = w.size().x / 2 - (w.size().x / 2) / 2;

    render!(w,
        (left_x, 5) => [ input_border ],
        (left_x, 8) => [ list_border ]
    );

    if d.state == AppState::Search {
        render!(w, (left_x + 1, 6) => [ d.cur_search, "█".blue() ]);
    } else {
        render!(w, (left_x + 1, 6) => [ d.cur_search ]);
    }

    let mut rendered = 0;
    let mut selected = d.scroll;

    for idx in d.cur_items.iter().skip(d.scroll) {
        let item = d.config.events[*idx].clone();
        if selected == d.selected {
            render!(w,
            (left_x + 1, rendered + 9) => [
                item.icon.with(item.icon_color),
                "  ",
                item.name.with(item.text_color),
                " < Enter > ".blue()
            ]);
        } else {
            render!(w,
            (left_x + 1, rendered + 9) => [
                item.icon.with(item.icon_color),
                "  ",
                item.name.with(item.text_color)
            ]);
        }

        rendered += 1;
        selected += 1;
        if rendered > max_elements || rendered > d.config.max_render {
            break;
        }
    }
}
