use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{List as RatatuiList, ListItem, Block, Borders},
    text::Line,
    Frame,
};
use crate::{Component, Context, ClickableArea};

pub struct List<A> {
    items: Vec<ListRow<A>>,
    title: Option<String>,
    border_style: Option<Style>,
    selected: Option<usize>,
    pub scroll_offset: Option<usize>,
}

pub struct ListRow<A> {
    pub label: Line<'static>,
    pub style: Option<Style>,
    pub action: Option<A>,
}

impl<A: Clone> List<A> {
    pub fn new(items: Vec<ListRow<A>>) -> Self {
        Self {
            items,
            title: None,
            border_style: None,
            selected: None,
            scroll_offset: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(format!(" {} ", title.into()));
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    /// Keeps this row index scrolled into view. `None` (the default) disables scrolling.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn scroll_offset(mut self, offset: Option<usize>) -> Self {
        self.scroll_offset = offset;
        self
    }
}

impl<S, A: Clone> Component<S, A> for List<A> {
    fn render(&self, f: &mut Frame, _state: &mut S, area: Rect, context: &mut Context<A>) {
        let mut block = Block::default().borders(Borders::ALL);

        if let Some(t) = &self.title {
            block = block.title(t.clone());
        }

        let b_style = self.border_style.unwrap_or(Style::default().fg(context.theme.secondary));
        block = block.border_style(b_style);

        let viewport_height = area.height.saturating_sub(2) as usize;
        let mut offset = self.scroll_offset.unwrap_or_else(|| crate::scroll_offset(self.selected, self.items.len(), viewport_height));
        let max_offset = self.items.len().saturating_sub(viewport_height);
        offset = offset.min(max_offset);
        let visible = self.items.iter().enumerate().skip(offset).take(viewport_height);

        let mut r_items = Vec::new();
        for (_, item) in visible.clone() {
            let mut r_item = ListItem::new(item.label.clone());
            if let Some(s) = item.style {
                r_item = r_item.style(s);
            }
            r_items.push(r_item);
        }

        let list = RatatuiList::new(r_items).block(block);
        f.render_widget(list, area);

        for (i, item) in visible {
            if let Some(action) = &item.action {
                // Approximate item area (List items start at y + 1 because of borders)
                let item_area = Rect {
                    x: area.x + 1,
                    y: area.y + 1 + (i - offset) as u16,
                    width: area.width.saturating_sub(2),
                    height: 1,
                };
                if item_area.y < area.y + area.height - 1 {
                    context.clickable_areas.push(ClickableArea {
                        area: item_area,
                        action: action.clone(),
                    });
                }
            }
        }
    }
}
