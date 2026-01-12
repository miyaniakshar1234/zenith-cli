use ratatui::style::Color;

#[allow(dead_code)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub accent_primary: Color,
    pub accent_secondary: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub inactive: Color,
    pub header_bg: Color,
    pub header_fg: Color,
}

pub const NEBULA: Theme = Theme {
    bg: Color::Rgb(20, 20, 30),                // Deep Space
    fg: Color::Rgb(220, 220, 240),             // Star White
    selection_bg: Color::Rgb(60, 20, 80),      // Deep Purple Selection
    selection_fg: Color::Rgb(255, 0, 255),     // Neon Magenta Text
    accent_primary: Color::Rgb(0, 240, 255),   // Electric Cyan
    accent_secondary: Color::Rgb(255, 0, 120), // Hot Pink
    border: Color::Rgb(80, 80, 120),           // Nebula Blue Border
    success: Color::Rgb(50, 255, 100),         // Kryptonite Green
    warning: Color::Rgb(255, 180, 0),          // Solar Flare Orange
    error: Color::Rgb(255, 50, 50),            // Supernova Red
    inactive: Color::Rgb(80, 80, 100),         // Stardust Grey
    header_bg: Color::Rgb(40, 30, 60),         // Header Gradient Base
    header_fg: Color::Rgb(0, 240, 255),        // Header Text
};
