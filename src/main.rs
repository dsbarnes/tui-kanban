mod util;
use util::event::{Event, Events};

use std::io;

use termion::{
    event::Key,
    input::MouseTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen
};

use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

struct Lane {
    name: String,
}

struct Card {
    title: String,
    description: String,
    priority: u8,
    time: u8,
    // due: timestamp,
    lane: Lane,
}

struct Goal {
    title: String,
    description: String,
    lanes: Vec<Lane>,
    cards: Vec<Card>,
}

enum InputMode {
    Normal,
    Title,
    Description,
}

struct App {
    input: String,
    input_mode: InputMode,
    goals: Vec<Goal>,
    lanes: Vec<Lane>,
    cards: Vec<Card>,
}
impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            goals: vec![],
            lanes: vec![],
            cards: vec![],
        }
    }
}


fn main() -> Result<(), io::Error> {
    let app = App::default();
    let events = Events::new();

    // The double stdout is what the actual documentation suggests
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        //let chunks;
        //let help_message;
        //let (title, description) = match app.input_mode {}
        //let cards;

        // Handle input
        if let Event::Input(input) = events.next().expect("") {
            match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('t') => {
                        print!("Do stuff with the t key\r\n");
                    },
                    Key::Char('d') => {
                        println!("Do stuff with the d key");
                    },
                    Key::Char('q') => {
                        break;
                    },
                    _ => {},
                },
                InputMode::Title => match input {
                    Key::Char('q') => {
                        break;
                    },
                    _ => {},
                },
                InputMode::Description => {
                    break;
                },
            }
        }
    }

    // So rust doesn't complain
    Ok(())
}
