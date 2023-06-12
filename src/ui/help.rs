use crate::app::App;

use tui::style::{Color, Modifier, Style};
use tui::text::{self, Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

pub fn render(app: &App) -> Paragraph {
  let text = vec![
    Spans::from(vec![
      Span::raw(" hl ←→  "),
      Span::raw(" — "),
      Span::raw("navigate panes"),
    ]),
    Spans::from(vec![
      Span::raw(" <tab>  "),
      Span::raw(" — "),
      Span::raw("cycle through panes"),
    ]),
    Spans::from(vec![
      Span::raw(" jk ↑↓  "),
      Span::raw(" — "),
      Span::raw("next / previous"),
    ]),
    Spans::from(vec![Span::raw(" f␣     "), Span::raw(" — "), Span::raw("page forward")]),
    Spans::from(vec![
      Span::raw(" b      "),
      Span::raw(" — "),
      Span::raw("page backwards"),
    ]),
    Spans::from(vec![
      Span::raw(" d      "),
      Span::raw(" — "),
      Span::raw("open git difftool"),
    ]),
    Spans::from(vec![
      Span::raw(" r      "),
      Span::raw(" — "),
      Span::raw("refresh repos"),
    ]),
    Spans::from(vec![Span::raw("   ")]),
    Spans::from(vec![
      Span::raw(" s      "),
      Span::raw(" — "),
      Span::raw("open / close calendar"),
    ]),
    Spans::from(vec![Span::raw(" <enter>"), Span::raw(" — "), Span::raw("select date")]),
    Spans::from(vec![Span::raw(" <esc>  "), Span::raw(" — "), Span::raw("close modal")]),
    Spans::from(vec![Span::raw("   ")]),
    Spans::from(vec![Span::raw(" q <esc>"), Span::raw(" — "), Span::raw("quit")]),
  ];

  Paragraph::new(text).block(
    Block::default()
      .title(title(app))
      .borders(Borders::ALL)
      .style(
        Style::default()
          .fg(Color::LightCyan)
          .add_modifier(Modifier::DIM),
      ),
  )
}

fn title(_app: &App) -> text::Span {
  let text_style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);
  text::Span::styled(" Help ", text_style)
}
