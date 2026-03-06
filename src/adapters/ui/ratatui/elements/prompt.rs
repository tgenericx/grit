use crate::dom::Element;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub struct Prompt {
    pub label: String,
    pub value: String,
}

impl Prompt {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            value: String::new(),
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.value.push(c);
    }

    pub fn pop_char(&mut self) {
        self.value.pop();
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }
}

impl Element for Prompt {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let line = Line::from(vec![
            Span::styled(
                format!("{}: ", self.label),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(&self.value),
            Span::styled("█", Style::default().fg(Color::White)),
        ]);
        f.render_widget(Paragraph::new(line), area);
    }
}
