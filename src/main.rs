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
    style::{ Color, Modifier, Style },
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
    current_lane: usize,
    cards: Vec<Card>,
}
impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            lanes: Vec::new(),
            current_lane: 0,
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
    let title = match app.input_mode {
        InputMode::Normal => { "Normal" },
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
    for index in 0..app.lanes.len() {
        let current_lane = app.lanes[index].items.clone();
        let title = match index {
            0 => { "Todo" },
            1 => { "In Progress" },
            2 => { "Finished" },
            3 => { "In review" },
            _ => { "How'd you get here?" },
        };

        let current_cards: Vec<ListItem> = current_lane
            .iter()
            .map(|card|{
                let li = vec![Spans::from(card.title.as_ref())];
                ListItem::new(li).style( Style::default())
            })
            .collect();

        let current_cards = List::new(current_cards.as_ref())
                .block(Block::default().borders(Borders::ALL)
                    .title(title)
                )
                .highlight_style(Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

    f.render_stateful_widget(current_cards, chunk[index], &mut app.lanes[index].state);
    }
}

fn draw_description<B>(f: &mut Frame<B>, chunk: Vec<Rect>, app: &App)
    where
        B: Backend,
{
    let description = Paragraph::new("Description dummy data")
        .block(Block::default()
            .borders(Borders::ALL)
        );
    f.render_widget(description, chunk[0]);
}

// Not sure why Box<dyn Error>> instead of just io::Error??
fn main() -> Result<(), Box<dyn Error>> {
    // Create the app and default lanes:
    let mut app = App::default();
    for _ in 0..4 { app.lanes.push(StatefulList::with_items(Vec::new())); }
    // Listen for events:
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
                    // Needs to be first:
                    Key::Char('\n') => {
                        // Select the current card
                        // for editing title / description
                    },
                    // Display the help screen
                    Key::Char('h') => { },
                    Key::Char('q') => { break; },
                    Key::Char('t') => { app.input_mode = InputMode::Title }, 
                    Key::Char('d') => { app.input_mode = InputMode::Description },

                    Key::Up => { app.lanes[app.current_lane].previous(); },
                    Key::Down => { app.lanes[app.current_lane].next(); },
                    Key::Left => { },
                    Key::Right => { },
                    Key::Ctrl('l') => {
                        if app.current_lane != 0 {
                            // Get the card that is currently selected:
                            let current_index = app.lanes[app.current_lane]
                                .state.selected().unwrap();
                            let current_card = app.lanes[app.current_lane]
                                .items[current_index].clone();

                            // Push the card to the previous lane:
                            app.lanes[app.current_lane-1].items.push(current_card);
                            // Unselect and Remove from the current lane:
                            app.lanes[app.current_lane].unselect();
                            app.lanes[app.current_lane].items.remove(current_index);
                            // Switch to that lane:
                            app.current_lane -= 1; 
                            // Select the 'next' card in that lane
                            app.lanes[app.current_lane].next();
                        }
                    },
                    Key::Ctrl('r') => {
                        if app.current_lane != 3 {
                            // Get the card that is currently selected:
                            let current_index = app.lanes[app.current_lane]
                                .state.selected().unwrap();
                            let current_card = app.lanes[app.current_lane]
                                .items[current_index].clone();

                            // Push the card to the next lane:
                            app.lanes[app.current_lane+1].items.push(current_card);
                            // Unselect and Remove from the current lane:
                            app.lanes[app.current_lane].unselect();
                            app.lanes[app.current_lane].items.remove(current_index);
                            // Switch to that lane:
                            app.current_lane += 1; 
                            // Select the 'next' card in that lane
                            app.lanes[app.current_lane].next();
                        }
                    },


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
    } // loop

    Ok(())
}
