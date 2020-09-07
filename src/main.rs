/*
 * The Smart Goal Kanban!
 *
 * Create a kanban board in the terminal with 3-5 lanes.
 * A menu at the top to create a new ticket.
 * When a ticket is selected we can edit the name, text, priority, due date,
 * whatever we'd like bout the ticket
 * Include a menue screen
 * 
 * Learn by doing!
 */
use std::io;

use termion::raw::IntoRawMode;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Tabs};
use tui::style::{Style, Color};
use tui::layout::{Layout, Direction, Rect};

fn main() {
    // Initial clap options to load config
    // 'select goal' or 'add new goal' screen
    // Once selected it's a kanban!
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tabs = ["S.M.A.R.T Goals", "Kanban"]
        .iter().cloned().map(Spans::from).collect();

    let block = Tabs::new(tabs)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("|");

    let chunk = Layout::default()
        .direction(Direction::Horizontal)
        .split(Rect {
            x: 2,
            y: 2,
            width: 10,
            height: 10,
        });

    terminal.draw(|t|{
        t.render_widget(block, chunk[0])
    }).unwrap();
}
