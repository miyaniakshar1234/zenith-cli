use ratatui::style::Color;

#[allow(dead_code)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub accent: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub inactive: Color,
}

pub const NORD_PRO: Theme = Theme {
    bg: Color::Rgb(46, 52, 64),            // Nord 0 (Dark Slate)
    fg: Color::Rgb(216, 222, 233),         // Nord 4 (Snow)
    selection_bg: Color::Rgb(76, 86, 106), // Nord 2 (Highlight)
    selection_fg: Color::White,
    accent: Color::Rgb(136, 192, 208),   // Nord 8 (Frost)
    border: Color::Rgb(59, 66, 82),      // Nord 1 (Subtle Split)
    success: Color::Rgb(163, 190, 140),  // Nord 14 (Green)
    warning: Color::Rgb(235, 203, 139),  // Nord 13 (Yellow)
    error: Color::Rgb(191, 97, 106),     // Nord 11 (Red)
    inactive: Color::Rgb(100, 110, 130), // Dimmed
};
