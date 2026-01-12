use ratatui::style::Color;

#[allow(dead_code)]
pub struct Theme {
    pub bg: Color,
    pub surface: Color, // Slightly lighter bg for panels
    pub fg: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub accent: Color,    // Main action color (Gold)
    pub secondary: Color, // Secondary info (Blue/Purple)
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub dimmed: Color,
}

pub const HORIZON: Theme = Theme {
    bg: Color::Rgb(28, 30, 38),              // Deep Void
    surface: Color::Rgb(45, 45, 55),         // Panel Background
    fg: Color::Rgb(224, 224, 224),           // Clean White
    selection_bg: Color::Rgb(225, 178, 105), // Horizon Gold (Selection)
    selection_fg: Color::Rgb(28, 30, 38),    // Dark Text on Gold
    accent: Color::Rgb(225, 178, 105),       // Horizon Gold
    secondary: Color::Rgb(173, 169, 255),    // Soft Lavender
    border: Color::Rgb(60, 60, 75),          // Subtle Border
    success: Color::Rgb(163, 235, 163),      // Soft Green
    warning: Color::Rgb(235, 203, 139),      // Soft Yellow
    error: Color::Rgb(235, 135, 135),        // Soft Red
    dimmed: Color::Rgb(100, 100, 115),       // Dimmed Text
};
