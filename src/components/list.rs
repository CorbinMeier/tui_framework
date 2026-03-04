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
}

impl<S, A: Clone> Component<S, A> for List<A> {
    fn render(&self, f: &mut Frame, _state: &mut S, area: Rect, context: &mut Context<A>) {
        let mut block = Block::default().borders(Borders::ALL);
        
        if let Some(t) = &self.title {
            block = block.title(t.clone());
        }
        
        let b_style = self.border_style.unwrap_or(Style::default().fg(context.theme.secondary));
        block = block.border_style(b_style);

        let mut r_items = Vec::new();
        for item in &self.items {
            let mut r_item = ListItem::new(item.label.clone());
            if let Some(s) = item.style {
                r_item = r_item.style(s);
            }
            r_items.push(r_item);
        }

        let list = RatatuiList::new(r_items).block(block);
        f.render_widget(list, area);

        for (i, item) in self.items.iter().enumerate() {
            if let Some(action) = &item.action {
                // Approximate item area (List items start at y + 1 because of borders)
                let item_area = Rect {
                    x: area.x + 1,
                    y: area.y + 1 + i as u16,
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
