mod icons;
mod weather;

use clap::Parser;
use once_cell::sync::OnceCell;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::Alignment::Center;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

use crate::weather::WeatherResult;

/// Command line arguments.
#[derive(Parser, Debug)]
#[clap(version = "0.0.0", author = "George S. <smyrgeorge@gmail.com>")]
struct Opts {
    /// Sets a custom config file.
    #[clap(short, long, default_value = "Barcelona")]
    pub query: String,

    #[clap(short, long, default_value = "")]
    pub api_key: String,
}

// Lazy static declaration of configuration.
static OPTS: OnceCell<Opts> = OnceCell::new();

fn main() -> Result<(), io::Error> {
    // Parse command line arguments.
    // TODO: handle unwraps, use something like anyerror.
    OPTS.set(Opts::parse()).unwrap();
    let opts: &Opts = OPTS.get().unwrap();
    println!("{:?}", opts);

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear the screen.
    terminal.clear()?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        // Now
        let block = Block::default()
            .title(" Now ")
            .title_alignment(Center)
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        // Next days
        let block = Block::default()
            .title(" Next days ")
            .title_alignment(Center)
            .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;

    terminal.clear()?;

    // Test http client.
    let api = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        opts.query, opts.api_key
    );
    let resp = reqwest::blocking::get(api)
        .unwrap()
        .json::<WeatherResult>()
        .unwrap();

    println!("{:#?}", resp);

    Ok(())
}
