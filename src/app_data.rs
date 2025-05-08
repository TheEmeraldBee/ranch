use ascii_forge::window::{StyledContent, Stylize, Window};

use crate::config::Config;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Select,
    Search,
    Logs,
}

impl AppState {
    pub fn up(&mut self) {
        *self = match self {
            Self::Select => Self::Select,
            Self::Search => Self::Select,
            Self::Logs => Self::Select,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppData {
    pub should_exit: bool,
    pub state: AppState,
    pub config: Config,

    pub selected: usize,
    pub cur_search: String,
    pub cur_items: Vec<usize>,
    pub scroll: usize,

    pub log: Vec<Vec<StyledContent<String>>>,
}

impl AppData {
    pub fn new(config: Config) -> Self {
        let mut ret = Self {
            should_exit: false,
            state: AppState::Select,

            selected: 0,
            cur_search: "".to_string(),
            cur_items: vec![],
            scroll: 0,

            config,

            log: vec![],
        };
        ret.update_search();
        ret
    }

    pub fn run_each(&mut self, window: &mut Window) {
        self.config
            .each
            .clone()
            .iter()
            .for_each(|x| x.run(window, self));
    }

    pub fn run(&mut self, window: &mut Window) {
        let event = self.config.events[self.cur_items[self.selected]].clone();
        self.log(vec![
            "[EVENT] ".to_string().red(),
            format!(
                "Beginning Run of {} : {}",
                self.cur_items[self.selected], event.name
            )
            .green(),
        ]);
        if !event.ignore_each {
            self.run_each(window);
        }
        event.each.iter().for_each(|x| x.run(window, self));
    }

    pub fn update_search(&mut self) {
        self.selected = 0;
        self.cur_items = self
            .config
            .matching(&self.cur_search, self.config.max_search);
    }

    pub fn select(&mut self, dist: isize) {
        if dist == 0 {
            return;
        }

        self.selected =
            (self.selected as isize + dist).clamp(0, self.cur_items.len() as isize - 1) as usize;
    }

    pub fn update_scroll(&mut self, max_items: usize) {
        if self.selected <= self.scroll {
            if self.selected == 0 {
                self.scroll = 0;
            } else {
                self.scroll = self.selected - 1
            }
        }
        while self.selected >= self.scroll + max_items {
            self.scroll += 1
        }
    }

    pub fn log(&mut self, log: Vec<StyledContent<String>>) {
        self.log.push(log);
        while self.log.len() > self.config.max_log_lines {
            self.log.remove(0);
        }
    }
}
