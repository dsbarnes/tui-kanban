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
        

        // Draw the layout
        terminal.draw(|f| {
            // to the point where I need the small help menu
            // the input
            // the lanes w/ lists in them
            // and the description box
            // drawn on the screen.
            
            let main_layout = Display::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(5),
                    Constraint::Percentage(15),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                ].as_ref())
                .split(f.size())
            // Create the help menu widget
            // Create the input widget

            let help_text = match app.input_mode {
                InputMode::Normal => {
                    vec![
                        Span::raw("Press `t` to enter TITLE mode or `d` to enter DESCRIPTION mode, q to break"),
                    ];
                },
                InputMode::Title |
                InputMode::Description => {
                    vec![
                        Span::raw("Press ESC to enter NORMAL mode"),
                    ];
                },
            };

            let mut help_message = Text::from(Spans::from(help_text));
            let help_menu = Paragraph::new(help_message);
            f.render_widget(help_menu, main_layout[0]);

            // Display the cursor
            match app.input_mode {
                // The cursor is hidden by default on the alt screen
                // so we don't actually need to do anything for the
                // case of Normal mode
                InputMode::Normal => {},
                InputMode::Title |
                InputMode::Description => {
                    // Show the cursor
                },
            } 

            // Handle input
            if let Event::Input(input) = events.next().unwrap() {
                match app.input_mode {
                    InputMode::Normal => match input {
                        Key::Char('t') => {
                            app.input_mode = InputMode::Title;
                        },
                        Key::Char('d') => {
                            app.input_mode = InputMode::Description;
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
                        Key::Char('z') => {
                            break;
                        },
                    },
                }
            }
        }
    })


    // So rust doesn't complain
    Ok(())
}
