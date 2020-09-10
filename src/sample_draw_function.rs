pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  let margin = util::get_main_layout_margin(app);
  let parent_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(6),
      ]
      .as_ref(),
    )
    .margin(margin)
    .split(f.size());
  // Search input and help
  draw_input_and_help_box(f, app, parent_layout[0]);
  // Nested main block with potential routes
  draw_routes(f, app, parent_layout[1]);
  // Currently playing
  draw_playbar(f, app, parent_layout[2]);
  // Possibly draw confirm dialog
  draw_dialog(f, app);
}


pub fn draw_help_menu<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(100)].as_ref())
    .margin(2)
    .split(f.size());

  let white = Style::default().fg(app.user_config.theme.text);
  let gray = Style::default().fg(app.user_config.theme.text);
  let header = ["Description", "Event", "Context"];

  let help_docs = get_help_docs();
  let help_docs = &help_docs[app.help_menu_offset as usize..];

  let rows = help_docs
    .iter()
    .map(|item| Row::StyledData(item.iter(), gray));

  let help_menu = Table::new(header.iter(), rows)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(white)
        .title(Span::styled("Help (press <Esc> to go back)", gray))
        .border_style(gray),
    )
    .style(Style::default().fg(app.user_config.theme.text))
    .widths(&[
      Constraint::Length(50),
      Constraint::Length(40),
      Constraint::Length(20),
    ]);
  f.render_widget(help_menu, chunks[0]);
}
