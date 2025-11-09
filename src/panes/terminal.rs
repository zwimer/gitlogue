use crate::animation::{ActivePane, AnimationEngine};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TerminalPane;

impl TerminalPane {
    pub fn render(&self, f: &mut Frame, area: Rect, engine: &AnimationEngine) {
        let block = Block::default()
            .title("Terminal")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        // Get visible lines based on area height
        let content_height = area.height.saturating_sub(2) as usize; // Subtract borders
        let total_lines = engine.terminal_lines.len();

        let lines: Vec<Line> = if total_lines > 0 {
            let start_idx = total_lines.saturating_sub(content_height);
            engine.terminal_lines[start_idx..]
                .iter()
                .enumerate()
                .map(|(idx, line)| {
                    let is_last_line = start_idx + idx == total_lines - 1;
                    let show_cursor = is_last_line
                        && engine.cursor_visible
                        && engine.active_pane == ActivePane::Terminal;

                    if line.starts_with("$ ") {
                        // Command line - show in green with bold
                        if show_cursor {
                            // Add cursor at the end of the line
                            let mut spans = vec![Span::styled(
                                line.clone(),
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            )];
                            spans.push(Span::styled(
                                " ",
                                Style::default()
                                    .bg(Color::White)
                                    .fg(Color::Black)
                                    .add_modifier(Modifier::BOLD),
                            ));
                            Line::from(spans)
                        } else {
                            Line::from(Span::styled(
                                line.clone(),
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            ))
                        }
                    } else {
                        // Output line - normal style
                        Line::from(line.clone())
                    }
                })
                .collect()
        } else {
            vec![Line::from("")]
        };

        let content = Paragraph::new(lines).block(block);
        f.render_widget(content, area);
    }
}
