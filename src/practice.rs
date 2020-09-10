use std::io;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::input::Events;

use tui::Terminal;
use tui::symbols::DOT;
use tui::text::Spans;
use tui::style::{Style, Color};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Tabs};
use tui::layout::{Layout, Constraint, Direction};

/*
 * Looks like to their utils are somewhat important for all this
 * We want to use Termion to take input
 * 
 *
 * TODO:
 * make each section that is drawn it's own function
 * make the program persistent until I hit an escape key
 * take Input
 * changing between tabs
 * learn to redraw the screen (when I change something)
 * move things from one section to another (and redraw the screen)
 * change what a box says as you select other boxes
 */

// let tabs = ["one", "two", "three"]
//     // All this is from the documentation
//     .iter()
//     .cloned()
//     .map(Spans::from)
//     .collect();
// 
// let tabs_widget = Tabs::new(tabs)
//     .block(
//         Block::default()
//             .title("tabs")
//             .borders(Borders::ALL)
//     )
//     .style(Style::default().fg(Color::White))
//     .highlight_style(Style::default().fg(Color::Yellow))
//     .divider(DOT);

// The app that is the graphic on the repo:
// https://github.com/fdehau/tui-rs/blob/master/examples/demo/app.rs
// https://github.com/fdehau/tui-rs/blob/master/examples/demo/ui.rs

fn main() -> Result<(), io::Error> {
    // Creates the terminal - the thing we can draw on
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // f is a tui Frame
    loop {
        terminal.draw(|f| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(100),
                ].as_ref())
                .split(f.size());

            let block0 = Block::default()
                .title("block0")
                .borders(Borders::ALL);

            f.render_widget(block0, main_layout[0])
        })?;

    }

}
