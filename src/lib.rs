use ratatui::{
    layout::Rect,
    Frame,
};
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};

pub mod components;
pub mod theme;

pub use theme::Theme;

/// Calculates the rendered width of a string in terminal cells.
pub fn text_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Truncates a string to fit within a target width in terminal cells.
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    let mut current_width = 0;
    let mut result = String::new();
    for c in s.chars() {
        let w = UnicodeWidthChar::width(c).unwrap_or(0);
        if current_width + w > max_width {
            break;
        }
        result.push(c);
        current_width += w;
    }
    result
}

/// Pads a string with spaces to reach a target width in terminal cells.
pub fn pad_to_width(s: &str, target_width: usize) -> String {
    let mut current_width = text_width(s);
    let mut result = s.to_string();
    while current_width < target_width {
        result.push(' ');
        current_width += 1;
    }
    result
}

/// Computes a scroll offset so `selected` stays within a `viewport_height`-row window out of
/// `len` total rows. Recomputed fresh every frame (no persisted scroll state needed).
pub fn scroll_offset(selected: Option<usize>, len: usize, viewport_height: usize) -> usize {
    if viewport_height == 0 || len <= viewport_height {
        return 0;
    }
    let max_offset = len - viewport_height;
    let sel = match selected {
        Some(s) => s,
        None => return 0,
    };
    let offset = if sel < viewport_height { 0 } else { sel + 1 - viewport_height };
    offset.min(max_offset)
}

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
