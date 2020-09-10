mod util;

use util::event::*;

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

fn main() -> Result<(), io::Error> {
    /*
     * Tabs, one for each goal
     * Input for Card title and Card description
     * Kanban Sections
     * Cards
     *
     * */
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

    loop {
        println!("In here we handle the things...");
        break;
    }

    // So rust doesn't complain
    Ok(())
}
