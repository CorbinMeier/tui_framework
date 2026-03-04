use ratatui::{
    layout::{Rect, Alignment},
    style::Style,
    widgets::Paragraph,
    Frame,
};
use crate::{Component, Context, ClickableArea};

pub struct TextButton<A> {
    label: String,
    action: A,
    style: Option<Style>,
    alignment: Alignment,
}

impl<A> TextButton<A> {
    pub fn new(label: impl Into<String>, action: A) -> Self {
        Self {
            label: label.into(),
            action,
            style: None,
            alignment: Alignment::Left,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<S, A: Clone> Component<S, A> for TextButton<A> {
    fn render(&self, f: &mut Frame, _state: &mut S, area: Rect, context: &mut Context<A>) {
        let style = self.style.unwrap_or(Style::default().fg(context.theme.primary));
        
        let p = Paragraph::new(self.label.clone())
            .style(style)
            .alignment(self.alignment);
            
        f.render_widget(p, area);

        context.clickable_areas.push(ClickableArea {
            area,
            action: self.action.clone(),
        });
    }
}
