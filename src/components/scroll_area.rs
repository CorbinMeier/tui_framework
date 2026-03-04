use ratatui::{
    layout::{Rect, Margin},
    style::Style,
    widgets::{Block, Borders, Scrollbar as RatatuiScrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use crate::{Component, Context};

pub struct ScrollArea<C> {
    pub offset: usize,
    pub content_length: usize,
    pub viewport_length: usize,
    pub child: C,
    title: Option<String>,
}

impl<C> ScrollArea<C> {
    pub fn new(child: C, offset: usize, content_length: usize, viewport_length: usize) -> Self {
        Self {
            child,
            offset,
            content_length,
            viewport_length,
            title: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(format!(" {} ", title.into()));
        self
    }
}

impl<S, A, C: Component<S, A>> Component<S, A> for ScrollArea<C> {
    fn render(&self, f: &mut Frame, state: &mut S, area: Rect, context: &mut Context<A>) {
        let mut block = Block::default().borders(Borders::ALL);
        if let Some(t) = &self.title {
            block = block.title(t.clone());
        }
        block = block.border_style(Style::default().fg(context.theme.secondary));

        f.render_widget(block, area);

        let inner = area.inner(Margin { horizontal: 1, vertical: 1 });
        
        // Render child
        self.child.render(f, state, inner, context);

        if self.content_length > self.viewport_length {
            let scrollbar_area = Rect {
                x: area.x + area.width - 1,
                y: area.y + 1,
                width: 1,
                height: area.height.saturating_sub(2),
            };
            
            let mut scrollbar_state = ScrollbarState::new(self.content_length)
                .position(self.offset)
                .viewport_content_length(self.viewport_length);

            let scrollbar = RatatuiScrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }
}
