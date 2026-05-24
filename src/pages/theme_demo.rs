//! Theme demo page.

use tuie::prelude::*;

use crate::segmented_control::SegmentedControl;
use crate::theme;

/// Theme demo page.
pub(crate) struct ThemeDemo {
    root: Box<Pane>,
    accent_toggle_id: WidgetId<SegmentedControl>,
    thumb_toggle_id: WidgetId<SegmentedControl>,
    border_toggle_id: WidgetId<SegmentedControl>,
    #[cfg(feature = "gui")]
    scheme_toggle_id: WidgetId<SegmentedControl>,
    #[cfg(feature = "gui")]
    appearance_toggle_id: WidgetId<SegmentedControl>,
}

impl DelegateWidget for ThemeDemo {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if let Some(&ChangeEvent(index)) = event.get_by::<ChangeEvent<usize>>(self.accent_toggle_id) {
            let (color, _) = theme::ACCENT_COLORS[index];
            theme::set_accent_color(color);
        } else if let Some(&ChangeEvent(index)) = event.get_by::<ChangeEvent<usize>>(self.thumb_toggle_id) {
            let (thumb, _) = theme::SCROLLBAR_THUMBS[index];
            theme::set_scrollbar_thumb(thumb);
        } else if let Some(&ChangeEvent(index)) = event.get_by::<ChangeEvent<usize>>(self.border_toggle_id) {
            let (border, _) = theme::BORDERS[index];
            theme::set_border(border);
        }
        #[cfg(feature = "gui")]
        if let Some(&ChangeEvent(index)) = event.get_by::<ChangeEvent<usize>>(self.scheme_toggle_id) {
            theme::set_color_scheme(index);
        } else if let Some(&ChangeEvent(index)) = event.get_by::<ChangeEvent<usize>>(self.appearance_toggle_id) {
            let (appearance, _) = theme::APPEARANCES[index];
            theme::set_appearance(appearance);
        }
    }
}

impl ThemeDemo {
    /// Creates the [`ThemeDemo`] page.
    pub(crate) fn new() -> Box<Self> {
        let mut accent_toggle_id = WidgetId::EMPTY;
        let mut thumb_toggle_id = WidgetId::EMPTY;
        let mut border_toggle_id = WidgetId::EMPTY;

        let accent_labels: Vec<&str> = theme::ACCENT_COLORS.iter().map(|(_, l)| *l).collect();
        let accent_toggle = SegmentedControl::new(&accent_labels)
            .selected(theme::get_accent_index())
            .id(&mut accent_toggle_id);

        let thumb_labels: Vec<&str> = theme::SCROLLBAR_THUMBS.iter().map(|(_, l)| *l).collect();
        let thumb_toggle = SegmentedControl::new(&thumb_labels)
            .selected(theme::get_thumb_index())
            .id(&mut thumb_toggle_id);

        let border_labels: Vec<&str> = theme::BORDERS.iter().map(|(_, l)| *l).collect();
        let border_toggle = SegmentedControl::new(&border_labels)
            .selected(theme::get_border_index())
            .id(&mut border_toggle_id);

        let intro = Pane::new().vertical().gap(1).children([
            Text::new()
                .content(
                    StyledString::new()
                        .span("Borders, styles, and scrollbar glyphs are driven by per-module ")
                        .span("config".fg(Color::YELLOW).bold())
                        .span(". Tweak a config and the app repaints."),
                )
                .word_wrap(),
        ]);

        let accent_section = Pane::new().vertical().gap(1).children([
            heading("Accent color") as Box<dyn Widget>,
            caption("Drives selected borders, pressed segments, and the selected-text highlight in Input."),
            accent_toggle,
        ]);

        let thumb_section = Pane::new().vertical().gap(1).children([
            heading("Scrollbar thumb") as Box<dyn Widget>,
            caption("The character (or block) used to render scrollbar thumbs across pages."),
            thumb_toggle,
        ]);

        let border_section = Pane::new().vertical().gap(1).children([
            heading("Border style") as Box<dyn Widget>,
            caption("Default border drawn by bordered() panes."),
            border_toggle,
        ]);

        #[cfg(feature = "gui")]
        let mut scheme_toggle_id = WidgetId::EMPTY;
        #[cfg(feature = "gui")]
        let mut appearance_toggle_id = WidgetId::EMPTY;

        #[cfg(feature = "gui")]
        let gui_section = {
            let scheme_labels: Vec<&str> =
                theme::COLOR_SCHEMES.iter().map(|(l, _, _)| *l).collect();
            let scheme_toggle = SegmentedControl::new(&scheme_labels)
                .selected(theme::get_color_scheme_index())
                .id(&mut scheme_toggle_id);

            let appearance_labels: Vec<&str> = theme::APPEARANCES.iter().map(|(_, l)| *l).collect();
            let appearance_toggle = SegmentedControl::new(&appearance_labels)
                .selected(theme::get_appearance_index())
                .id(&mut appearance_toggle_id);

            Pane::new().vertical().gap(1).children([
                heading("Color scheme (GUI)") as Box<dyn Widget>,
                caption("Picks the (light, dark) Theme pair installed in tuie::gui::GuiConfig."),
                scheme_toggle,
                heading("Appearance (GUI)"),
                caption("Forces light or dark, or follows the OS appearance."),
                appearance_toggle,
            ])
        };

        let root = Pane::new().vertical().gap(2).flex(1).children([
            intro,
            accent_section,
            thumb_section,
            border_section,
        ]);
        #[cfg(feature = "gui")]
        let root = root.children([gui_section as Box<dyn Widget>]);

        Box::new(Self {
            root,
            accent_toggle_id,
            thumb_toggle_id,
            border_toggle_id,
            #[cfg(feature = "gui")]
            scheme_toggle_id,
            #[cfg(feature = "gui")]
            appearance_toggle_id,
        })
    }
}

fn heading(label: &str) -> Box<Text> {
    Text::new().content(label.fg(Color::BLUE).bold())
}

fn caption(text: &str) -> Box<Text> {
    Text::new()
        .content(text)
        .word_wrap()
}

/// Returns the theme demo page widget.
pub(crate) fn theme_demo_page() -> Box<ThemeDemo> {
    ThemeDemo::new()
}
