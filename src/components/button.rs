use ratatui::{
    layout::{Rect, Alignment},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    prelude::Stylize,
    Frame,
};
use crate::{Component, Context, ClickableArea};

pub struct Button<A> {
    label: String,
    action: A,
    style: ButtonStyle,
    is_disabled: bool,
}

#[derive(Clone, Copy)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    Success,
    Ghost,
    Plain,
    Info,
}

impl<A> Button<A> {
    pub fn new(label: impl Into<String>, action: A) -> Self {
        Self {
            label: label.into(),
            action,
            style: ButtonStyle::Primary,
            is_disabled: false,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.is_disabled = d;
        self
    }
}

impl<S, A: Clone> Component<S, A> for Button<A> {
    fn render(&self, f: &mut Frame, _state: &mut S, area: Rect, context: &mut Context<A>) {
        let (mut fg, mut bg) = match self.style {
            ButtonStyle::Primary => (context.theme.on_primary, context.theme.primary),
            ButtonStyle::Secondary => (context.theme.on_secondary, context.theme.secondary),
            ButtonStyle::Danger => (context.theme.on_danger, context.theme.danger),
            ButtonStyle::Success => (context.theme.on_success, context.theme.success),
            ButtonStyle::Ghost => (context.theme.primary, context.theme.background),
            ButtonStyle::Plain => (context.theme.text, context.theme.background),
            ButtonStyle::Info => (context.theme.on_accent, context.theme.accent),
        };

        if self.is_disabled {
            fg = context.theme.secondary;
            bg = context.theme.background;
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(fg))
            .bg(bg);

        let p = Paragraph::new(self.label.clone())
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(fg));

        f.render_widget(p, area);

        if !self.is_disabled {
            context.clickable_areas.push(ClickableArea {
                area,
                action: self.action.clone(),
            });
        }
    }
}
