use ratatui::{
    layout::Rect,
    Frame,
};

pub mod components;
pub mod theme;

pub use theme::Theme;

pub trait Component<S, A> {
    fn render(&self, f: &mut Frame, state: &mut S, area: Rect, context: &mut Context<A>);
}

/// Information passed down during rendering to register interactivity
pub struct Context<'a, A> {
    pub clickable_areas: &'a mut Vec<ClickableArea<A>>,
    pub theme: Theme,
}

impl<'a, A: Clone> Context<'a, A> {
    pub fn new(clickable_areas: &'a mut Vec<ClickableArea<A>>, theme: Theme) -> Self {
        Self {
            clickable_areas,
            theme,
        }
    }

    pub fn handle_click(&self, column: u16, row: u16) -> Option<A> {
        self.handle_click_ca(column, row).map(|ca| ca.action.clone())
    }

    pub fn handle_click_ca(&self, column: u16, row: u16) -> Option<&ClickableArea<A>> {
        for area in self.clickable_areas.iter().rev() {
            if area.area.x <= column && column < area.area.x + area.area.width &&
               area.area.y <= row && row < area.area.y + area.area.height {
                return Some(area);
            }
        }
        None
    }
}

pub struct ClickableArea<A> {
    pub area: Rect,
    pub action: A,
}
