mod util;
use util::{
    event::{ Event, Events },
    signal::StatefulList,
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
    style::{ Modifier, Style },
    text::{ Span, Spans, Text },
    widgets::{ Block, Borders, List, ListItem, Paragraph, },
    Frame,
    Terminal,
};

#[derive(Clone)]
struct App {
    input: String,
    input_mode: InputMode,
    lanes: Vec<StatefulList<Card>>,
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

#[derive(Clone)]
struct Card {
    title: String,
    description: String,
    lane: u8,
    priority: u8,
}
impl Default for Card {
    fn default() -> Self {
        Card {
            title: String::new(),
            description: String::new(),
            lane: 0,
            priority: 0,
        }
    }
}

#[derive(Clone)]
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
    let title = match app.input_mode {
        InputMode::Normal => { "Input" },
        InputMode::Title => { "Title" },
        InputMode::Description => { "Description" },
    };

    let input_box = Paragraph::new(app.input.as_ref())
        .block(Block::default()
            // Should change based on mode
            .title(title)
            .borders(Borders::ALL)
        );
    f.render_widget(input_box, chunk)
}

fn draw_lanes<B>(f: &mut Frame<B>, chunk: Vec<Rect>, app: &mut App)
    where
        B: Backend,
{
    let todo_lane = app.lanes[0].items.clone();
    let todo_cards: Vec<ListItem> = todo_lane
        .iter()
        .map(|card|{
            // Push each card title to the list
            // this does not do anything with the description
            let li = vec![Spans::from(card.title.as_ref())];
            ListItem::new(li).style(Style::default())
        })
        .collect();

    let todo_cards = List::new(todo_cards.as_ref())
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default()
                .add_modifier(Modifier::BOLD),
            );

    f.render_stateful_widget(todo_cards, chunk[0], &mut app.lanes[0].state);
    // f.render_stateful_widget(in_progress_cards, chunk[1], &mut app.lanes[1].state);
    // f.render_stateful_widget(finished_cards, chunk[2], &mut app.lanes[2].state);
    // f.render_stateful_widget(review_cards, chunk[3], &mut app.lanes[3].state);
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
}

// Not sure why Box<dyn Error>> instead of just io::Error??
fn main() -> Result<(), Box<dyn Error>> {
    // Create the app and default lanes:
    let mut app = App::default();

    // Creates the lanes, but this is a silly way off doing it.
    let lane_names = vec!["TODO", "In Progress", "Finished", "Review"];
    for _ in lane_names {
        app.lanes.push(StatefulList::with_items(Vec::new()));
    }

    let events = Events::new();

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
            draw_lanes(f, card_layout, &mut app);
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
                    Key::Char('\n') => {
                        // Select the current card
                        // for editing title / description
                    },

                    Key::Char('q') => { break; },
                    Key::Char('t') => { app.input_mode = InputMode::Title }, 
                    Key::Char('d') => { app.input_mode = InputMode::Description },

                    Key::Up => {},
                    Key::Down => {},
                    Key::Left => {},
                    Key::Right => {},

                    // Display the help screen
                    Key::Char('h') => { },
                    _ => { },
                },

                InputMode::Title => match input {
                    Key::Char('\n') => {
                        // Pressing enter in Title mode will switch to
                        // Description mode, unless app.input.len() < 7
                        if app.input.len() > 6 {
                            // Create the card
                            let new_card = Card {
                                title: String::from(&app.input),
                                description: String::new(),
                                lane: 0,
                                priority: 0,
                            };
                            app.lanes[0].items.push(new_card);
                            app.input = "".to_string();
                            app.input_mode = InputMode::Description;
                        }
                    },
                    Key::Esc => { app.input_mode = InputMode::Normal; },
                    Key::Char(c) => { app.input.push(c); },
                    Key::Backspace => { app.input.pop(); },
                    _ => { },
                },

                InputMode::Description => match input {
                    Key::Char('\n') => {
                        let mut card = app.lanes[0].items.pop().unwrap();
                        card.description = String::from(&app.input);
                        app.lanes[0].items.push(card);
                    },
                    Key::Esc => { app.input_mode = InputMode::Normal; },
                    Key::Char(c) => { app.input.push(c); },
                    Key::Backspace => { app.input.pop(); },
                    _ => { },
                },
            }
        }
    }

    Ok(())
}
