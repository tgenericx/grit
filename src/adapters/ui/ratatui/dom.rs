use ratatui::{Frame, layout::Rect};

pub trait Element {
    fn height(&self) -> u16;
    fn render(&self, f: &mut Frame, area: Rect);
}

pub struct Dom {
    pub elements: Vec<Box<dyn Element>>,
}

impl Dom {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn push(&mut self, element: Box<dyn Element>) {
        self.elements.push(element);
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let mut y = area.y;
        for element in &self.elements {
            let height = element.height();
            let el_area = Rect {
                x: area.x,
                y,
                width: area.width,
                height: height.min(area.height.saturating_sub(y - area.y)),
            };
            element.render(f, el_area);
            y += height;
            if y >= area.y + area.height {
                break;
            }
        }
    }
}
