use std::{fs::File, io::Read, path::PathBuf, process::Command};

use anyhow::anyhow;
use crokey::KeyCombination;
use serde::Deserialize;

use ascii_forge::{
    prelude::Color,
    window::{
        EnableFocusChange, EnableMouseCapture, Stylize, Window,
        crossterm::{
            cursor::Hide,
            execute,
            terminal::{DisableLineWrap, EnterAlternateScreen, enable_raw_mode},
        },
    },
};

use crate::{app_data::AppData, rank::rank};

#[derive(Deserialize, Debug, Clone)]
pub enum ExecEvent {
    #[serde(rename = "cmd")]
    Cmd(String),
    #[serde(rename = "exec")]
    Exec(String),
    #[serde(rename = "shell")]
    Shell(String),
    #[serde(rename = "exit")]
    Exit,
}

impl ExecEvent {
    pub fn run(&self, w: &mut Window, d: &mut AppData) {
        d.log(vec![
            "   ".to_string().stylize(),
            "Running Event ".to_string().stylize(),
            format!("{:?}", self).stylize(),
        ]);
        let output = match self {
            Self::Cmd(cmd) => Command::new(d.config.shell.clone())
                .arg("-c")
                .arg(cmd)
                .output()
                .unwrap(),

            Self::Exec(exec) => Command::new(d.config.shell.clone())
                .arg("-c")
                .arg(format!("{} {}", d.config.executor, exec))
                .output()
                .unwrap(),

            Self::Shell(cmd) => {
                w.restore().unwrap();
                Command::new(d.config.shell.clone())
                    .arg("-c")
                    .arg(cmd)
                    .status()
                    .unwrap();

                enable_raw_mode().unwrap();
                execute!(
                    w.io(),
                    EnterAlternateScreen,
                    EnableMouseCapture,
                    EnableFocusChange,
                    Hide,
                    DisableLineWrap,
                )
                .unwrap();

                w.buffer_mut().clear();
                w.swap_buffers();

                return;
            }

            Self::Exit => {
                d.should_exit = true;
                return;
            }
        };

        if !output.status.success() {
            panic!("{:?}", output);
        }
    }
}

fn reset() -> Color {
    Color::Reset
}

#[derive(Deserialize, Debug, Clone)]
pub enum Entry {
    #[serde(rename = "folder")]
    Folder {
        name: String,
        #[serde(default = "Vec::new")]
        entries: Vec<Entry>,
    },
    #[serde(rename = "entry")]
    Entry(ExecInfo),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExecInfo {
    pub icon: String,
    pub name: String,

    #[serde(default)]
    pub ignore_each: bool,

    #[serde(default = "reset")]
    pub icon_color: Color,
    #[serde(default = "reset")]
    pub text_color: Color,

    #[serde(default = "Vec::new")]
    pub events: Vec<ExecEvent>,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct BindMap {
    pub logs: Vec<KeyCombination>,
    pub insert: Vec<KeyCombination>,
    pub normal: Vec<KeyCombination>,

    pub quit: Vec<KeyCombination>,

    pub run: Vec<KeyCombination>,

    pub clear: Vec<KeyCombination>,

    pub up: Vec<KeyCombination>,
    pub down: Vec<KeyCombination>,
    pub exit: Vec<KeyCombination>,
    pub enter: Vec<KeyCombination>,
}

impl Default for BindMap {
    fn default() -> Self {
        Self {
            logs: vec![crokey::key!(shift - l)],
            insert: vec![crokey::key!(i)],
            normal: vec![crokey::key!(esc), crokey::key!(q)],

            quit: vec![crokey::key!(esc), crokey::key!(q)],

            run: vec![crokey::key!(enter)],

            clear: vec![crokey::key!(ctrl - l), crokey::key!(c)],

            up: vec![crokey::key!(up), crokey::key!(k)],
            down: vec![crokey::key!(down), crokey::key!(j)],
            exit: vec![crokey::key!(left), crokey::key!(h)],
            enter: vec![crokey::key!(right), crokey::key!(l), crokey::key!(enter)],
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub shell: String,
    pub executor: String,
    pub each: Vec<ExecEvent>,

    pub binds: BindMap,

    pub max_log_lines: usize,

    pub max_search: usize,
    pub max_render: u16,

    pub entries: Vec<Entry>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: "sh".to_string(),
            executor: "hyprctl dispatch exec".to_string(),
            each: Vec::new(),

            max_log_lines: 30,

            max_search: 30,
            max_render: 30,

            entries: Vec::new(),

            binds: BindMap::default(),
        }
    }
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> anyhow::Result<Self> {
        let directory = match path {
            Some(t) => t,
            None => dirs::config_dir()
                .ok_or(anyhow!("Failed to find config dir"))?
                .join("ranch/config.yaml"),
        };

        let mut file = File::open(directory)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        let code = serde_yaml::from_str(&text)?;
        Ok(code)
    }

    pub fn get_entry(&self, path: Vec<usize>, item: usize) -> Entry {
        if path.is_empty() {
            return self.entries[item].clone();
        }
        let mut entry_list = &self.entries;
        for row in &path {
            if let Entry::Folder { entries, .. } = &entry_list[*row] {
                entry_list = entries;
            } else {
                panic!("Path encountered a entry and not a folder")
            }
        }

        entry_list[item].clone()
    }

    pub fn list_path(&self, path: Vec<usize>) -> Vec<(Vec<usize>, usize)> {
        if path.is_empty() {
            return (0..self.entries.len())
                .into_iter()
                .map(|x| (path.clone(), x))
                .collect();
        }
        let mut entry_list = &self.entries;
        for row in &path {
            if let Entry::Folder { entries, .. } = &entry_list[*row] {
                entry_list = entries;
            } else {
                panic!("Path encountered a entry and not a folder")
            }
        }

        return (0..entry_list.len())
            .into_iter()
            .map(|x| (path.clone(), x))
            .collect();
    }

    pub fn matching(&self, key: &str, max: usize) -> Vec<(Vec<usize>, usize)> {
        let r = entry_rank(&self.entries, key, vec![]);
        let mut results = r
            .into_iter()
            .filter(|x| x.1.is_some())
            .map(|x| (x.0, x.1.unwrap(), x.2))
            .collect::<Vec<_>>();
        results.sort();

        results.into_iter().map(|x| (x.0, x.2)).take(max).collect()
    }
}

fn entry_rank(
    entries: &[Entry],
    key: &str,
    cur_path: Vec<usize>,
) -> Vec<(Vec<usize>, Option<i32>, usize)> {
    let mut results = vec![];
    for (i, entry) in entries.iter().enumerate() {
        match entry {
            Entry::Folder { entries, .. } => {
                let mut new_path = cur_path.clone();
                new_path.push(i);
                results.extend(entry_rank(entries, key, new_path));
            }
            Entry::Entry(e) => {
                let rank = rank(key, &e.name);
                results.push((cur_path.clone(), rank, i));
            }
        }
    }
    results
}
