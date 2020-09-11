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

enum ActiveWidget {
    Tab,
    Input,
    Lane,
    Card,
}

struct App {
    input: String,
    input_mode: InputMode,
    active_widget: ActiveWidget,
    goals: Vec<Goal>,
    lanes: Vec<Lane>,
    cards: Vec<Card>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            active_widget: ActiveWidget::Input,
            goals: Vec::new(),
            lanes: Vec::new(),
            cards: Vec::new(),
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

        /*
         * The input box is a 'paragraph' widget
         * we set the value of the paragraph to the value of some string
         * this is what makes it look like we're typing
         * 
         * We'll need to position the cursor one space after the String
         *
         * learn to change between 'widgets'
         * and handle the same key in a different way depending on
         * the 'active' widget
         */

        // Create a small help message
        // Create an empty string that represents out input
        
        // Display the cursor
        match app.input_mode {
            // The cursor is hidden by default on the alt screen
            // so we don't actually need to do anything for the
            // case of Normal mode
            InputMode::Normal => {},
            InputMode::Title |
            InputMode::Description => {
                // Show the cursor
                f.set_cursot()
            },
        } 

        // Draw the layout
        terminal.draw(|f| {
            // Put everything else in here
            // to the point where I need the small help menu
            // the input
            // the lanes w/ lists in them
            // and the description box
            // drawn on the screen.
        })

        // Handle input
        if let Event::Input(input) = events.next().unwrap() {
            match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('t') => {
                        print!("Do stuff with the t key\n\r");
                    },
                    Key::Char('d') => {
                        print!("Do stuff with the d key\n\r");
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
