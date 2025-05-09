use ascii_forge::prelude::*;
use clap::*;

use widget_forge::prelude::*;

pub mod config;
use config::Config;

pub mod args;
use args::Args;

pub mod rank;

pub mod app_data;
use app_data::{AppData, AppState};

pub mod handlers;
use handlers::*;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = Config::load(args.config)?;

    let mut window = Window::init()?;
    window.keyboard().unwrap();
    handle_panics();
    Scene::new(window, AppData::new(config))
        .insert_conditional_widgets(|_w, d| d.state == AppState::Search, (input_handler,))
        .insert_conditional_widgets(
            |_w, d| d.state == AppState::Select,
            (quit_handler, to_other, select, clear),
        )
        .insert_conditional_widgets(|_w, d| d.state != AppState::Logs, (run, render))
        .insert_conditional_widgets(|_w, d| d.state == AppState::Logs, (render_logs,))
        .insert_widget(up)
        .run(50, |s| !s.data().should_exit)?;

    Ok(())
}
