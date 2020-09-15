use super::signal::StatefulList;

use tui::{
    backend::{ Backend },
    layout::{ Rect },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans, Text },
    widgets::{ Block, Borders, List, ListItem, Paragraph, },
    Frame,
};

#[derive(Clone)]
pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub lanes: Vec<StatefulList<Card>>,
    pub current_lane: usize,
    pub cards: Vec<Card>,
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
impl App {
    pub fn get_current_card(&self) -> Option<Card> {
        // There should be some check if there are cards
        // that can be selected or not.
        // If this is called w/o a card selected - the program panics and crashes

        if self.lanes[self.current_lane].state.selected() == None {
            return None;
        }
        let current_index = self.lanes[self.current_lane]
            .state.selected().unwrap();
        let current_card = self.lanes[self.current_lane]
            .items[current_index].clone();
        Some(current_card)
    }
}

#[derive(Clone, PartialEq)]
pub struct Card {
    pub title: String,
    pub description: Vec<String>,
    pub lane: u8,
    pub priority: u8,
}
impl Default for Card {
    fn default() -> Self {
        Card {
            title: String::new(),
            description: Vec::new(),
            lane: 0,
            priority: 0,
        }
    }
}

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Title,
    Description,
}

pub fn draw_help_text<B>(f: &mut Frame<B>, chunk: Rect, app: &App)
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


pub fn draw_input_box<B>(f: &mut Frame<B>, chunk: Rect, app: &App)
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
            .title(title)
            .borders(Borders::ALL)
        );
    f.render_widget(input_box, chunk)
}


pub fn draw_lanes<B>(f: &mut Frame<B>, chunk: Vec<Rect>, app: &mut App)
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

pub fn draw_description<B>(f: &mut Frame<B>, chunk: Vec<Rect>, app: &App)
    where
        B: Backend,
{
//     let text = 
//     vec![ Spans::from(
//         vec![ Span::raw("First"),
//              Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
//              Span::raw("."),
//              ]),
//     Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
//     ];
//
//
//     Paragraph::new(text)
//         .block(Block::default().title("Paragraph").borders(Borders::ALL))
//         .style(Style::default().fg(Color::White).bg(Color::Black))
//         .alignment(Alignment::Center)
//         .wrap(Wrap { trim: true });

    // let current_card = 
    //     if app.lanes[0].items.len() > 0 { app.get_current_card() }
    //     else { Card::default() };
    //     
    // let current_card = app.get_current_card();

    if let Some(card) = app.get_current_card() {
        let text = vec![
            Spans::from( vec![
                Span::raw(card.description[0].as_str())
            ]
        )];

        let description = Paragraph::new(text)
            .block(Block::default()
                .borders(Borders::ALL)
            );

        f.render_widget(description, chunk[0]);
    }
}
