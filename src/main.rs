mod util;
use util::{
    event::{ Event, Events },
    signal::StatefulList,
    draw::{
        App,
        Card,
        InputMode,
        draw_help_text, 
        draw_input_box, 
        draw_lanes, 
        draw_description
    }
};

use std::{ error::Error, io };

use termion::{
    event::Key,
    raw::IntoRawMode,
    screen::AlternateScreen
};

use tui::{
    backend::TermionBackend,
    layout::{ Constraint, Direction, Layout },
    Terminal,
};

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
                    Key::Char('?') => { },
                    Key::Char('q') => { break; },
                    Key::Char('t') => { app.input_mode = InputMode::Title }, 
                    Key::Char('d') => { app.input_mode = InputMode::Description },

                    Key::Up => { app.lanes[app.current_lane].previous(); },
                    Key::Down => { app.lanes[app.current_lane].next(); },
                    
                    Key::Left => {
                        if app.current_lane != 0 {
                            app.lanes[app.current_lane].unselect();
                            app.current_lane -= 1;
                            app.lanes[app.current_lane].next();
                        }

                    },
                    Key::Right => {
                        if app.current_lane != 3 {
                            app.lanes[app.current_lane].unselect();
                            app.current_lane += 1;
                            app.lanes[app.current_lane].next();
                        }
                    },
                    
                    Key::Ctrl(',') => {
                        if app.current_lane != 0 {
                            // Get the card that is currently selected:
                            let current_index = app.lanes[app.current_lane]
                                .state.selected().unwrap();

                            if let Some(current_card) = app.get_current_card(){
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
                        }
                    },
                    Key::Ctrl('.') => {
                        if app.current_lane != 3 {
                            // Get the card that is currently selected:
                            let current_index = app.lanes[app.current_lane]
                                .state.selected().unwrap();

                            if let Some(current_card) = app.get_current_card(){
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
                        }
                    },
                    _ => { },
                },

                InputMode::Title => match input {
                    Key::Char('\n') => {
                        // Pressing enter in Title mode will switch to
                        // Description mode, unless app.input.len() < 7
                        print!("{:?}", app.current_lane);

                        if app.input.len() > 6 {
                            // Create the card
                            let new_card = Card {
                                title: String::from(&app.input),
                                description: Vec::new(),
                                lane: 0,
                                priority: 0,
                            };
                            app.lanes[app.current_lane].items.push(new_card);
                            app.input = "".to_string();
                            if let None = app.get_current_card(){
                                app.get_current_card();
                            }
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
                        if let Some(mut current_card) = app.get_current_card(){
                            current_card.description.push(String::from(&app.input));
                            app.input = "".to_string();
                        }
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
