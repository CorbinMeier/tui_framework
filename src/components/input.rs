use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::{Component, Context, ClickableArea};

pub struct TextInput<A> {
    value: String,
    placeholder: String,
    label: String,
    is_focused: bool,
    is_disabled: bool,
    cursor_pos: usize,
    on_click: A,
    mask_char: Option<char>,
}

impl<A> TextInput<A> {
    pub fn new(label: impl Into<String>, value: impl Into<String>, on_click: A) -> Self {
        let val = value.into();
        let len = val.len();
        Self {
            label: label.into(),
            value: val,
            placeholder: String::new(),
            is_focused: false,
            is_disabled: false,
            cursor_pos: len,
            on_click,
            mask_char: None,
        }
    }

    pub fn cursor_pos(mut self, pos: usize) -> Self {
        self.cursor_pos = pos;
        self
    }

    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    pub fn focused(mut self, f: bool) -> Self {
        self.is_focused = f;
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.is_disabled = d;
        self
    }

    pub fn masked(mut self, c: char) -> Self {
        self.mask_char = Some(c);
        self
    }
}

impl<S, A: Clone> Component<S, A> for TextInput<A> {
    fn render(&self, f: &mut Frame, _state: &mut S, area: Rect, context: &mut Context<A>) {
        let border_color = if self.is_disabled {
            context.theme.secondary
        } else if self.is_focused {
            context.theme.focus
        } else {
            context.theme.secondary
        };
        
        let display_text = if self.value.is_empty() && !self.is_focused {
            self.placeholder.clone()
        } else if let Some(c) = self.mask_char {
            let mut s = String::with_capacity(self.value.len() + 1);
            for _ in 0..self.value.len() {
                s.push(c);
            }
            s.push(' ');
            s
        } else {
            format!("{} ", self.value)
        };

        let style = if self.is_disabled {
            Style::default().fg(context.theme.secondary)
        } else if self.value.is_empty() && !self.is_focused {
            Style::default().fg(context.theme.secondary)
        } else {
            Style::default().fg(context.theme.text)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.label.clone())
            .border_style(Style::default().fg(border_color));

        let p = Paragraph::new(display_text)
            .block(block)
            .style(style);

        f.render_widget(p, area);

        if self.is_focused && !self.is_disabled {
            f.set_cursor_position((
                area.x + self.cursor_pos as u16 + 1,
                area.y + 1,
            ));
        }

        if !self.is_disabled {
            context.clickable_areas.push(ClickableArea {
                area,
                action: self.on_click.clone(),
            });
        }
    }
}
