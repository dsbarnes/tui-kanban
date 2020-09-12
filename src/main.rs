mod util;
use util::{
    event::{ Event, Events },
    // signal::StatefulList,
};

use std::{ error::Error, io };

use termion::{
    event::Key,
    // input::MouseTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen
};

use tui::{
    backend::{ TermionBackend, Backend },
    layout::{Constraint, Direction, Layout, Rect},
    style::{ Color, Modifier, Style },
    text::{ Span, Spans, Text },
    widgets::{ Block, Borders, List, ListItem, Paragraph, },
    Frame,
    Terminal,
};

struct App {
    input: String,
    input_mode: InputMode,
    lanes: Vec<Lane>,
    cards: Vec<Card>,
}
impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            lanes: Vec::new(),
            cards: Vec::new(),
        }
    }
}

struct Lane {
    name: String,
    cards: Vec<Card>,
}
impl Lane {
    fn new(name: String, cards: Vec<Card>) -> Self{
        Lane {
            name,
            cards,
        }
    }

}

struct Card {
    title: String,
    description: String,
    priority: u8,
}
impl Default for Card {
    fn default() -> Self {
        Card {
            title: String::new(),
            description: String::new(),
            priority: 0,
        }
    }
}

enum InputMode {
    Normal,
    Title,
    Description,
}


fn draw_help_text<B>(f: &mut Frame<B>, chunk: Rect, app: &App)
    where
        B: Backend,
{
    
    let help_text = match app.input_mode {
        InputMode::Normal => {
            vec![ Span::raw("Press 'h' for HELP or 'q' to EXIT"), ]
        },

        InputMode::Title |
        InputMode::Description => {
            vec![ Span::raw("Press ESC to enter NORMAL mode"), ]
        },
    };

    let help_message = Text::from(Spans::from(help_text));
    let help_menu = Paragraph::new(help_message);
    f.render_widget(help_menu, chunk);
}

fn draw_input_box<B>(f: &mut Frame<B>, chunk: Rect, app: &App)
    where
        B: Backend,
{
    // Create a new paragraph with a value of app.input
    // See 'handle input'
    let input_box = Paragraph::new(app.input.as_ref())
        .block(Block::default()
            // Should change based on mode
            .title("Title/Description")
            .borders(Borders::ALL)
        );
    f.render_widget(input_box, chunk)
}

fn draw_lanes<B>(f: &mut Frame<B>, chunk: Vec<Rect>, app: &App)
    where
        B: Backend,
{
    // Still working this out
    let lane0 = Block::default()
        .title(app.lanes[0].name.as_ref())
        .borders(Borders::ALL);

    let lane1 = Block::default()
        .title(app.lanes[1].name.as_ref())
        .borders(Borders::ALL);

    let lane2 = Block::default()
        .title(app.lanes[2].name.as_ref())
        .borders(Borders::ALL);

    let lane3 = Block::default()
        .title(app.lanes[3].name.as_ref())
        .borders(Borders::ALL);

    f.render_widget(lane0, chunk[0]);
    f.render_widget(lane1, chunk[1]);
    f.render_widget(lane2, chunk[2]);
    f.render_widget(lane3, chunk[3]);
}

fn draw_description<B>(f: &mut Frame<B>, chunk: Vec<Rect>, app: &App)
    where
        B: Backend,
{
    let description = Paragraph::new("This is the card description")
        .block(Block::default()
            .borders(Borders::ALL)
        );
    f.render_widget(description, chunk[0]);
    // f.render_widget(something_else, chunk[1]);
}

// Not sure why Box<dyn Error>> instead of just io::Error??
fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();
    let events = Events::new();

    // Create the Lanes for the Kanban
    let lane_names = vec![ "TODO", "In Progress", "Finished", "Review", ];
    for name in lane_names {
        app.lanes.push(Lane::new(name.to_string(), Vec::new()))
    }

    // The double stdout is what the actual documentation suggests
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    // Help text 
                    Constraint::Percentage(8),
                    // Search Bar
                    Constraint::Percentage(12),
                    // Lanes and Cards
                    Constraint::Percentage(50),
                    // Description
                    Constraint::Percentage(30),
                ].as_ref())
                .split(f.size());

            let card_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ].as_ref())
                .split(main_layout[2]);

            let description_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(75),
                    Constraint::Percentage(25),
                ].as_ref())
                .split(main_layout[3]);

            draw_help_text(f, main_layout[0], &app);
            draw_input_box(f, main_layout[1], &app);
            draw_lanes(f, card_layout, &app);
            draw_description(f, description_layout, &app);


            // Display the cursor if in Title or Description mode
            match app.input_mode {
                InputMode::Normal => {},
                InputMode::Title |
                InputMode::Description => {
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        main_layout[1].x + app.input.len() as u16 + 1,
                        // Move one line down, from the border to the input line
                        main_layout[1].y + 1,
                    )
                },
            }
        })?; // End closure

        // Handle input
        if let Event::Input(input) = events.next().unwrap() {
            match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('q') => { break; },
                    Key::Char('t') => { app.input_mode = InputMode::Title }, 
                    // Display the help screen
                    Key::Char('h') => { },
                    _ => { },
                },

                InputMode::Title => match input {
                    Key::Char('\n') => {
                        // Push the title to a card
                    },
                    Key::Esc => { app.input_mode = InputMode::Normal; },
                    Key::Char(c) => { app.input.push(c); },
                    Key::Backspace => { app.input.pop(); },
                    Key::Char('\n') => {
                        // Add a 'Card' to the 'Todo' 'Lane'
                        let card = Card {
                            title: app.input.clone(),
                            description: app.input.clone(),
                            priority: 0,
                        };

                        let todo_lane = &app.lanes[0];
                    }
                    _ => { },
                },

                InputMode::Description => match input {
                    Key::Char('i') => { break; },
                    _ => { },
                },
            }
        }
    }

    Ok(())
}
