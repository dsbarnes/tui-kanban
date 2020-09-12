mod util;
use util::event::{ Event, Events };

use std::{ error::Error, io };

use termion::{
    event::Key,
    input::MouseTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen
};

use tui::{
    backend::{ TermionBackend, Backend },
    layout::{Constraint, Direction, Layout, Rect},
    style::{ Color, Modifier, Style },
    text::{ Span, Spans, Text },
    widgets::{ Block, Borders, List, ListItem, Paragraph },
    Frame,
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

fn draw_help_text<B>(f: &mut Frame<B>, chunk: Rect, app: &App)
    where
        B: Backend,
    {
    
    let help_text = match app.input_mode {
        InputMode::Normal => {
            vec![
                Span::raw("Press 'h' for HELP or 'q' to EXIT"),
            ]
        },
        InputMode::Title |
        InputMode::Description => {
            vec![
                Span::raw("Press ESC to enter NORMAL mode"),
            ]
        },
    };

    let mut help_message = Text::from(Spans::from(help_text));
    let help_menu = Paragraph::new(help_message);
    f.render_widget(help_menu, chunk);
}


// Not sure why Box<dyn Error>> instead of just io::Error??
fn main() -> Result<(), Box<dyn Error>> {
    let app = App::default();
    let events = Events::new();

    // The double stdout is what the actual documentation suggests
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Draw the layout
        terminal.draw(|f| {
            // Drawn on the screen.
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(5),
                    Constraint::Percentage(15),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                ].as_ref())
                .split(f.size());

            draw_help_text(f, main_layout[0], &app);

            // Display the cursor if in Title or Description mode
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
        })?;

        // Handle basic input
        if let Event::Input(input) = events.next().unwrap() {
            match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('q') => { break; },
                    _ => { },
                },

                InputMode::Title => match input {
                    Key::Char('\n') => { },
                    Key::Char(c) => { },
                    Key::Backspace => { },
                    Key::Esc => { },
                    _ => { },
                },

                InputMode::Description => match input {
                    Key::Char('i') => { break; },
                    _ => { },
                },
            }
        }
    }

    // So rust doesn't complain
    Ok(())
}
