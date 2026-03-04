use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    widgets::{Block, Borders, Clear, Paragraph},
    style::Style,
    Frame,
};
use crate::{Component, Context, ClickableArea};
use crate::components::button::{Button, ButtonStyle};

pub struct ConfirmationDialog<A> {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub on_confirm: A,
    pub on_cancel: A,
}

impl<A: Clone> ConfirmationDialog<A> {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        on_confirm: A,
        on_cancel: A,
    ) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            confirm_label: "Yes".to_string(),
            cancel_label: "No".to_string(),
            on_confirm,
            on_cancel,
        }
    }

    pub fn labels(mut self, confirm: impl Into<String>, cancel: impl Into<String>) -> Self {
        self.confirm_label = confirm.into();
        self.cancel_label = cancel.into();
        self
    }
}

impl<S, A: Clone> Component<S, A> for ConfirmationDialog<A> {
    fn render(&self, f: &mut Frame, state: &mut S, area: Rect, context: &mut Context<A>) {
        f.render_widget(Clear, area);
        
        // 1. Block clicks on the background from falling through (Lowest precedence)
        context.clickable_areas.push(ClickableArea {
            area,
            action: self.on_cancel.clone(), // Default to cancel if clicking background
        });

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.title))
            .border_style(Style::default().fg(context.theme.accent));
        f.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),    // Message
                Constraint::Length(3), // Buttons
            ])
            .margin(2)
            .split(area);

        // 2. Message
        let p = Paragraph::new(self.message.as_str())
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(p, chunks[0]);

        // 3. Buttons
        let btn_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[1]);

        Button::new(self.confirm_label.clone(), self.on_confirm.clone())
            .style(ButtonStyle::Danger)
            .render(f, state, btn_chunks[0], context);

        Button::new(self.cancel_label.clone(), self.on_cancel.clone())
            .style(ButtonStyle::Secondary)
            .render(f, state, btn_chunks[1], context);
    }
}
