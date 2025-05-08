use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
    process::Command,
    thread::sleep,
    time::Duration,
};

use anyhow::anyhow;
use serde::Deserialize;

use ascii_forge::{
    prelude::Color,
    window::{
        EnableFocusChange, EnableMouseCapture, Stylize, Window,
        crossterm::{
            cursor::Hide,
            execute,
            terminal::{
                DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode,
            },
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
    pub each: Vec<ExecEvent>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    pub shell: String,
    pub executor: String,
    pub each: Vec<ExecEvent>,

    pub log: bool,
    pub max_log_lines: usize,

    pub max_search: usize,
    pub max_render: u16,

    pub events: Vec<ExecInfo>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: "sh".to_string(),
            executor: "hyprctl dispatch exec".to_string(),
            each: Vec::new(),

            log: false,
            max_log_lines: 30,

            max_search: 30,
            max_render: 30,

            events: Vec::new(),
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

    pub fn matching(&self, key: &str, max: usize) -> Vec<usize> {
        let mut results = vec![];
        for (i, event) in self.events.iter().enumerate() {
            let rank = rank(key, &event.name);
            results.push((rank, i));
        }
        let mut results = results
            .iter()
            .filter(|x| x.0.is_some())
            .map(|x| (x.0.unwrap(), x.1))
            .collect::<Vec<_>>();
        results.sort();

        results.iter().map(|x| x.1).take(max).collect()
    }
}
