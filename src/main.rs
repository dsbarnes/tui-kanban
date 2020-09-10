use std::io;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::Terminal;
use tui::symbols::DOT;
use tui::text::Spans;
use tui::style::{Style, Color};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Tabs};
use tui::layout::{Layout, Constraint, Direction};

/*
 * Generally, this is what I've learned,
 *
 * the terminal is the outermost 'frame'
 * this frame is divides into chunks.
 * those chunks can be used to render more frames, which again, chunks
 * the chunks are places to put widgets.
 * 
 * Looks like to their utils are somewhat necessary for all this
 * We want to use Termion to take input
 *
 * TODO:
 * make each section that is drawn it's own function
 * Input
 * Tabs, and changing between them
 * learn to redraw the screen
 * move things from one section to another
 * change what a box says as you select other boxes
 *
 * */

fn main() -> Result<(), io::Error> {
    // Creates the terminal - the thing we can draw on
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // f is a tui Frame
    terminal.clear()?;
    terminal.draw(|f| {
        // Master layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(35),
                Constraint::Percentage(50),
            ].as_ref())
            .split(f.size());

        // Sub layout
        let smaller_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref())
            .split(chunks[0]);

        let smaller_chunks1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ].as_ref())
            .split(chunks[2]);

        let smaller_chunks2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref())
            .split(chunks[1]);

        let tabs = ["one", "two", "three"]
            // All this is from the documentation
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();

        let tabs_widget = Tabs::new(tabs)
            .block(
                Block::default()
                    .title("tabs")
                    .borders(Borders::ALL)
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT);

        let block0 = Block::default()
            .title("block0")
            .borders(Borders::ALL);

        let block1 = Block::default()
            .title("block1")
            .borders(Borders::ALL);

        let block3 = Block::default()
            .title("block3")
            .borders(Borders::ALL);

        let block4 = Block::default()
            .title("block4")
            .borders(Borders::ALL);

        let block9 = Block::default()
            .title("block4")
            .borders(Borders::ALL);

        let block8 = Block::default()
            .title("block4")
            .borders(Borders::ALL);

        let block5 = Block::default()
            .title("block5")
            .borders(Borders::ALL);

        let block6 = Block::default()
            .title("block6")
            .borders(Borders::ALL);



        f.render_widget(tabs_widget, smaller_chunks[0]);
        f.render_widget(block1, smaller_chunks[1]);

        f.render_widget(block5, smaller_chunks2[0]);
        f.render_widget(block6, smaller_chunks2[1]);

        f.render_widget(block3, smaller_chunks1[0]);
        f.render_widget(block4, smaller_chunks1[1]);
        f.render_widget(block9, smaller_chunks1[2]);
        f.render_widget(block8, smaller_chunks1[3]);
    })
}
