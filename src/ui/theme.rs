use ratatui::style::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThemeType {
    Horizon,
    Nebula,
    Nord,
    Cyberpunk,
}

impl ThemeType {
    pub fn next(&self) -> Self {
        match self {
            ThemeType::Horizon => ThemeType::Nebula,
            ThemeType::Nebula => ThemeType::Nord,
            ThemeType::Nord => ThemeType::Cyberpunk,
            ThemeType::Cyberpunk => ThemeType::Horizon,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub bg: Color,
    pub surface: Color,
    pub fg: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub accent: Color,
    pub secondary: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub dimmed: Color,
}

pub fn get_theme(t: ThemeType) -> Theme {
    match t {
        ThemeType::Horizon => HORIZON,
        ThemeType::Nebula => NEBULA,
        ThemeType::Nord => NORD,
        ThemeType::Cyberpunk => CYBERPUNK,
    }
}

pub const HORIZON: Theme = Theme {
    bg: Color::Rgb(28, 30, 38),
    surface: Color::Rgb(45, 45, 55),
    fg: Color::Rgb(224, 224, 224),
    selection_bg: Color::Rgb(225, 178, 105),
    selection_fg: Color::Rgb(28, 30, 38),
    accent: Color::Rgb(225, 178, 105),
    secondary: Color::Rgb(173, 169, 255),
    border: Color::Rgb(60, 60, 75),
    success: Color::Rgb(163, 235, 163),
    warning: Color::Rgb(235, 203, 139),
    error: Color::Rgb(235, 135, 135),
    dimmed: Color::Rgb(100, 100, 115),
};

pub const NEBULA: Theme = Theme {
    bg: Color::Rgb(20, 20, 30),
    surface: Color::Rgb(35, 30, 45),
    fg: Color::Rgb(220, 220, 240),
    selection_bg: Color::Rgb(60, 20, 80),
    selection_fg: Color::Rgb(0, 240, 255),
    accent: Color::Rgb(0, 240, 255),
    secondary: Color::Rgb(255, 0, 120),
    border: Color::Rgb(80, 80, 120),
    success: Color::Rgb(50, 255, 100),
    warning: Color::Rgb(255, 180, 0),
    error: Color::Rgb(255, 50, 50),
    dimmed: Color::Rgb(80, 80, 100),
};

pub const NORD: Theme = Theme {
    bg: Color::Rgb(46, 52, 64),
    surface: Color::Rgb(59, 66, 82),
    fg: Color::Rgb(236, 239, 244),
    selection_bg: Color::Rgb(136, 192, 208),
    selection_fg: Color::Rgb(46, 52, 64),
    accent: Color::Rgb(136, 192, 208),
    secondary: Color::Rgb(129, 161, 193),
    border: Color::Rgb(76, 86, 106),
    success: Color::Rgb(163, 190, 140),
    warning: Color::Rgb(235, 203, 139),
    error: Color::Rgb(191, 97, 106),
    dimmed: Color::Rgb(216, 222, 233),
};

pub const CYBERPUNK: Theme = Theme {
    bg: Color::Rgb(10, 10, 15),
    surface: Color::Rgb(20, 20, 25),
    fg: Color::Rgb(255, 255, 255),
    selection_bg: Color::Rgb(255, 0, 120),
    selection_fg: Color::White,
    accent: Color::Rgb(255, 238, 0),
    secondary: Color::Rgb(0, 243, 255),
    border: Color::Rgb(100, 100, 100),
    success: Color::Rgb(57, 255, 20),
    warning: Color::Rgb(255, 150, 0),
    error: Color::Rgb(255, 0, 50),
    dimmed: Color::Rgb(80, 80, 80),
};
