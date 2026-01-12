use ratatui::style::Color;

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub text_main: Color,
    pub text_dim: Color,
}

pub const NEON_CYBERPUNK: Theme = Theme {
    primary: Color::Rgb(0, 243, 255),     // Electric Cyan
    secondary: Color::Rgb(255, 0, 255),   // Neon Magenta
    accent: Color::Rgb(255, 238, 0),      // Cyber Yellow
    background: Color::Rgb(10, 10, 15),   // Deep Space Dark
    success: Color::Rgb(57, 255, 20),     // Neon Green
    warning: Color::Rgb(255, 102, 0),     // Neon Orange
    error: Color::Rgb(255, 0, 50),        // Neon Red
    text_main: Color::Rgb(230, 230, 230), // Off White
    text_dim: Color::Rgb(100, 100, 120),  // Dimmed Blue-Grey
};
