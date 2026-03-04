use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use crate::{Component, Context};
use crate::components::button::{Button, ButtonStyle};

pub struct HeaderItem<A> {
    pub label: String,
    pub action: A,
    pub is_active: bool,
}

pub struct Header<A> {
    pub items: Vec<HeaderItem<A>>,
}

impl<A> Header<A> {
    pub fn new(items: Vec<HeaderItem<A>>) -> Self {
        Self { items }
    }
}

impl<S, A: Clone> Component<S, A> for Header<A> {
    fn render(&self, f: &mut Frame, state: &mut S, area: Rect, context: &mut Context<A>) {
        if self.items.is_empty() { return; }
        
        let constraints = vec![Constraint::Fill(1); self.items.len()];
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        for (i, item) in self.items.iter().enumerate() {
            let style = if item.is_active {
                ButtonStyle::Success
            } else {
                ButtonStyle::Ghost
            };

            Button::new(item.label.clone(), item.action.clone())
                .style(style)
                .render(f, state, chunks[i], context);
        }
    }
}
