use crate::dom::Element;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

pub struct LogLine {
    pub text: String,
    pub style: Style,
}

impl LogLine {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Element for LogLine {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let widget = Paragraph::new(Line::from(Span::styled(&self.text, self.style)));
        f.render_widget(widget, area);
    }
}
