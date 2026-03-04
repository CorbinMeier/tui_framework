use ratatui::style::Color;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub primary: Color,
    pub on_primary: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub accent: Color,
    pub on_accent: Color,
    pub background: Color,
    pub text: Color,
    pub focus: Color,
    pub danger: Color,
    pub on_danger: Color,
    pub success: Color,
    pub on_success: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            on_primary: Color::Black,
            secondary: Color::DarkGray,
            on_secondary: Color::White,
            accent: Color::Yellow,
            on_accent: Color::Black,
            background: Color::Reset,
            text: Color::White,
            focus: Color::Yellow,
            danger: Color::Red,
            on_danger: Color::White,
            success: Color::Green,
            on_success: Color::Black,
        }
    }
}
