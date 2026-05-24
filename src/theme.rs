//! Demo theme configuration.

use std::cell::Cell;

use tuie::prelude::*;
use tuie::render::border;
use tuie::widget::scrollbar;
use tuie::widget::widgets::input;
#[cfg(feature = "gui")]
use tuie::theme::Theme;

thread_local! {
    static ACCENT_COLOR: Cell<Color> = Cell::new(Color::BLUE);
    static ACCENT: Cell<Style> = Cell::new(Style::new().fg(Color::BLUE));
}

/// Selectable accent color options with display labels.
pub const ACCENT_COLORS: &[(Color, &str)] = &[
    (Color::RED, "Red"),
    (Color::GREEN, "Green"),
    (Color::BLUE, "Blue"),
    (Color::YELLOW, "Yellow"),
    (Color::MAGENTA, "Magenta"),
    (Color::CYAN, "Cyan"),
];

/// Selectable scrollbar thumb options with display labels.
pub const SCROLLBAR_THUMBS: &[(ScrollbarThumb, &str)] = &[
    (ScrollbarThumb::SINGLE, "Single"),
    (ScrollbarThumb::DOUBLE, "Double"),
    (ScrollbarThumb::THICK, "Thick"),
    (ScrollbarThumb::BLOCK, "Block"),
    (ScrollbarThumb::ASCII, "Ascii"),
];

/// Selectable border options with display labels.
pub const BORDERS: &[(&'static Border, &str)] = &[
    (Border::SINGLE, "Single"),
    (Border::DOUBLE, "Double"),
    (Border::ROUND, "Round"),
    (Border::THICK, "Thick"),
    (Border::ASCII, "Ascii"),
];

/// Returns the current accent style.
pub fn get_accent() -> Style {
    ACCENT.with(|c| c.get())
}

/// Returns the current accent color.
pub fn get_accent_color() -> Color {
    ACCENT_COLOR.with(|c| c.get())
}

/// Sets the accent color.
pub fn set_accent_color(color: Color) {
    ACCENT_COLOR.with(|c| c.set(color));
    ACCENT.with(|c| c.set(Style::new().fg(color)));
    input::config::update(|cfg| cfg.highlight_style = Style::new().fg(color).reverse());
}

/// Sets the global scrollbar thumb glyph.
pub fn set_scrollbar_thumb(thumb: ScrollbarThumb) {
    scrollbar::config::update(|cfg| cfg.thumb = thumb);
}

/// Returns the index into [`ACCENT_COLORS`] of the current accent color.
pub fn get_accent_index() -> usize {
    let current = get_accent_color();
    ACCENT_COLORS.iter().position(|(c, _)| *c == current).unwrap_or(2)
}

/// Returns the index into [`SCROLLBAR_THUMBS`] of the current thumb.
pub fn get_thumb_index() -> usize {
    let current = scrollbar::config::get().thumb;
    SCROLLBAR_THUMBS.iter().position(|(t, _)| *t == current).unwrap_or(2)
}

/// Sets the global border glyph set.
pub fn set_border(border: &'static Border) {
    border::config::update(|cfg| {
        cfg.border = border;
        cfg.selected_border = if std::ptr::eq(border, Border::SINGLE) {
            Border::THICK
        } else {
            border
        };
    });
}

/// Returns the index into [`BORDERS`] of the current border.
pub fn get_border_index() -> usize {
    let current = border::config::get().border;
    BORDERS.iter().position(|(b, _)| std::ptr::eq(*b, current)).unwrap_or(0)
}

/// Selectable GUI color scheme options with display labels.
#[cfg(feature = "gui")]
pub const COLOR_SCHEMES: &[(&str, Theme, Theme)] = &[
    ("Century", Theme::CENTURY_LIGHT, Theme::CENTURY_DARK),
    ("One", Theme::ONE_LIGHT, Theme::ONE_DARK),
    ("Solarized", Theme::SOLARIZED_LIGHT, Theme::SOLARIZED_DARK),
    ("Gruvbox", Theme::GRUVBOX_LIGHT, Theme::GRUVBOX_DARK),
    ("Everforest", Theme::EVERFOREST_LIGHT, Theme::EVERFOREST_DARK),
];

/// Selectable GUI appearance options with display labels, where `None` follows the OS appearance.
#[cfg(feature = "gui")]
pub const APPEARANCES: &[(Option<ColorScheme>, &str)] = &[
    (Some(ColorScheme::Light), "Light"),
    (Some(ColorScheme::Dark), "Dark"),
    (None, "System"),
];

/// Sets the GUI color scheme to the entry at `index`.
#[cfg(feature = "gui")]
pub fn set_color_scheme(index: usize) {
    let (_, light, dark) = COLOR_SCHEMES[index];
    tuie::gui::config::update(|cfg| {
        cfg.light_theme = light;
        cfg.dark_theme = dark;
    });
    tuie::gui::reapply_theme();
}

/// Returns the index into [`COLOR_SCHEMES`] matching the active GUI themes.
#[cfg(feature = "gui")]
pub fn get_color_scheme_index() -> usize {
    let cfg = tuie::gui::config::get();
    COLOR_SCHEMES
        .iter()
        .position(|(_, l, d)| *l == cfg.light_theme && *d == cfg.dark_theme)
        .unwrap_or(0)
}

/// Sets the GUI appearance override.
#[cfg(feature = "gui")]
pub fn set_appearance(appearance: Option<ColorScheme>) {
    tuie::gui::config::update(|cfg| cfg.appearance = appearance);
    tuie::gui::reapply_theme();
}

/// Returns the index into [`APPEARANCES`] of the current GUI appearance.
#[cfg(feature = "gui")]
pub fn get_appearance_index() -> usize {
    let current = tuie::gui::config::get().appearance;
    APPEARANCES.iter().position(|(a, _)| *a == current).unwrap_or(2)
}
