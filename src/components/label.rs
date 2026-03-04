use ratatui::{
    layout::{Rect, Alignment},
    style::{Style, Modifier},
    widgets::Paragraph,
    Frame,
};
use crate::{Component, Context};

pub struct Label {
    text: String,
    alignment: Alignment,
    style: Style,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            alignment: Alignment::Left,
            style: Style::default(),
        }
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.alignment = a;
        self
    }

    pub fn style(mut self, s: Style) -> Self {
        self.style = s;
        self
    }

    pub fn bold(mut self) -> Self {
        self.style = self.style.add_modifier(Modifier::BOLD);
        self
    }
}

impl<S, A> Component<S, A> for Label {
    fn render(&self, f: &mut Frame, _state: &mut S, area: Rect, context: &mut Context<A>) {
        let style = if self.style == Style::default() {
            Style::default().fg(context.theme.text)
        } else {
            self.style
        };
        let p = Paragraph::new(self.text.clone())
            .alignment(self.alignment)
            .style(style);
        f.render_widget(p, area);
    }
}
