use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear},
    style::Style,
    prelude::Alignment,
    Frame,
};
use crate::{Component, Context};
use crate::components::{
    label::Label,
    input::TextInput,
    button::{Button, ButtonStyle},
};

pub struct AuthCard<A> {
    pub title: String,
    pub username_value: String,
    pub password_value: String,
    pub focus_index: usize,
    pub notifications: Vec<String>,
    pub on_username_click: A,
    pub on_password_click: A,
    pub on_login: A,
    pub on_register: A,
}

impl<A: Clone> AuthCard<A> {
    pub fn new(
        title: impl Into<String>,
        username_value: String,
        password_value: String,
        focus_index: usize,
        on_username_click: A,
        on_password_click: A,
        on_login: A,
        on_register: A,
    ) -> Self {
        Self {
            title: title.into(),
            username_value,
            password_value,
            focus_index,
            notifications: Vec::new(),
            on_username_click,
            on_password_click,
            on_login,
            on_register,
        }
    }

    pub fn notifications(mut self, notes: Vec<String>) -> Self {
        self.notifications = notes;
        self
    }
}

impl<S, A: Clone> Component<S, A> for AuthCard<A> {
    fn render(&self, f: &mut Frame, state: &mut S, area: Rect, context: &mut Context<A>) {
        f.render_widget(Clear, area);
        f.render_widget(Block::default().borders(Borders::ALL).title(format!(" {} ", self.title)), area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Subtitle/Header
                Constraint::Length(3), // Username
                Constraint::Length(3), // Password
                Constraint::Length(3), // Buttons
                Constraint::Min(0),    // Error/Status
            ])
            .margin(2)
            .split(area);

        // 1. Title
        Label::new(self.title.to_uppercase())
            .alignment(Alignment::Center)
            .bold()
            .style(Style::default().fg(context.theme.primary))
            .render(f, state, chunks[0], context);

        // 2. Username Input
        TextInput::new(" Username ", self.username_value.clone(), self.on_username_click.clone())
            .placeholder("Enter username...")
            .focused(self.focus_index == 0)
            .render(f, state, chunks[1], context);

        // 3. Password Input
        TextInput::new(" Password ", self.password_value.clone(), self.on_password_click.clone())
            .placeholder("Enter password...")
            .focused(self.focus_index == 1)
            .masked('*')
            .render(f, state, chunks[2], context);

        // 4. Buttons
        let btn_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[3]);

        Button::new("Login", self.on_login.clone())
            .style(ButtonStyle::Primary)
            .render(f, state, btn_chunks[0], context);

        Button::new("Register", self.on_register.clone())
            .style(ButtonStyle::Secondary)
            .render(f, state, btn_chunks[1], context);

        // 5. Notifications
        if let Some(note) = self.notifications.last() {
            let color = if note.to_lowercase().contains("error") || note.to_lowercase().contains("failed") {
                context.theme.danger
            } else {
                context.theme.success
            };
            Label::new(note.clone())
                .alignment(Alignment::Center)
                .style(Style::default().fg(color))
                .render(f, state, chunks[4], context);
        }
    }
}
