use druid::Color;
use once_cell::sync::Lazy;

pub mod grid;
pub mod image_button;
pub mod layout;
pub mod toolbar;

pub struct ColorScheme {
    pub bg: Color,
    pub fg: Color,
    pub grid: Color,
    pub cursor: Color,
    pub preview: Color,
    pub highlight: Color,
    pub selection: Color,
}

pub struct ThemeManager {
    colorscheme: ColorScheme,
    is_light: bool,
}

impl ThemeManager {
    pub fn light() -> ColorScheme {
        ColorScheme {
            bg: Color::WHITE,
            fg: Color::BLACK,
            grid: Color::rgb(0.96, 0.96, 0.96),
            cursor: Color::rgb(0.91, 0.91, 0.91).with_alpha(0.5),
            preview: Color::RED,
            highlight: Color::RED,
            selection: Color::rgb(0.08, 0.61, 0.99).with_alpha(0.5),
        }
    }

    pub fn dark() -> ColorScheme {
        ColorScheme {
            bg: Color::rgb(0.15, 0.15, 0.15),
            fg: Color::WHITE,
            grid: Color::rgb(0.20, 0.20, 0.20),
            cursor: Color::rgb(0.25, 0.25, 0.25).with_alpha(0.8),
            preview: Color::rgb(0.90, 0.33, 0.29).with_alpha(0.8),
            highlight: Color::rgb(0.90, 0.33, 0.29).with_alpha(0.8),
            selection: Color::rgb(0.33, 0.61, 0.96).with_alpha(0.5),
        }
    }

    pub fn new() -> Self {
        let mode = dark_light::detect();
        return match mode {
            dark_light::Mode::Dark => Self {
                colorscheme: ThemeManager::dark(),
                is_light: false,
            },
            dark_light::Mode::Light | dark_light::Mode::Default => Self {
                colorscheme: ThemeManager::light(),
                is_light: true,
            },
        };
    }

    pub fn current(&self) -> &ColorScheme {
        &self.colorscheme
    }

    pub fn toggle_theme(&mut self) {
        if self.is_light {
            self.is_light = false;
            self.colorscheme = ThemeManager::dark();
        } else {
            self.is_light = true;
            self.colorscheme = ThemeManager::light();
        }
    }
}

pub static mut CURRENT_THEME: Lazy<ThemeManager> = Lazy::new(|| ThemeManager::new());
