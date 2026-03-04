use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::{Component, Context};
use crate::components::text_button::TextButton;

pub struct SectionHeader<A> {
    pub title: String,
    pub actions: Vec<SectionAction<A>>,
}

pub struct SectionAction<A> {
    pub label: String,
    pub action: A,
    pub style: Option<Style>,
}

impl<A: Clone> SectionHeader<A> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            actions: Vec::new(),
        }
    }

    pub fn add_action(mut self, label: impl Into<String>, action: A, style: Option<Style>) -> Self {
        self.actions.push(SectionAction {
            label: label.into(),
            action,
            style,
        });
        self
    }
}

impl<S, A: Clone> Component<S, A> for SectionHeader<A> {
    fn render(&self, f: &mut Frame, state: &mut S, area: Rect, context: &mut Context<A>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(context.theme.secondary));
        
        f.render_widget(block, area);
        
        let inner = area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
        
        let constraints = if self.actions.is_empty() {
            vec![Constraint::Fill(1)]
        } else {
            let mut c = vec![Constraint::Fill(1)];
            for _ in &self.actions {
                c.push(Constraint::Length(15)); // fixed width for actions for consistency
            }
            c
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner);

        // 1. Title
        f.render_widget(Paragraph::new(format!(" {} ", self.title)).style(Style::default().fg(context.theme.primary).add_modifier(ratatui::style::Modifier::BOLD)), chunks[0]);

        // 2. Actions
        for (i, action) in self.actions.iter().enumerate() {
            TextButton::new(format!(" [{}] ", action.label), action.action.clone())
                .style(action.style.unwrap_or(Style::default().fg(context.theme.accent)))
                .alignment(ratatui::layout::Alignment::Center)
                .render(f, state, chunks[i+1], context);
        }
    }
}
