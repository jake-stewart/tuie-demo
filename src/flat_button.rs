//! Borderless focusable button widget.

use tuie::prelude::*;
use chord_macro::chord;
use tuie::render::border;

use crate::theme;

/// Borderless focusable button that emits [`ClickEvent`].
pub(crate) struct FlatButton {
    pane: Box<Pane>,
    bg: Color,
    state: WidgetState,
    base_style: Option<Style>,
}

impl FlatButton {
    fn apply_state_bg(&mut self) {
        let base = *self.base_style.get_or_insert_with(|| self.pane.get_style());
        let mut style = base;
        match self.state {
            WidgetState::None | WidgetState::Hover => {
                if style.bg.is_none() {
                    style.bg = Some(self.bg);
                }
            }
            WidgetState::Focused | WidgetState::FocusedHover => {
                style.bg = None;
                style.fg = Some(theme::get_accent_color());
                style.set_reverse(true);
            }
            WidgetState::Active => {
                style.bg = None;
                style.fg = Some(theme::get_accent_color());
                style.set_reverse(true);
                style.set_blend(Some(75));
            }
        }
        self.pane.set_style(style);
    }
}

impl DelegateWidget for FlatButton {
    tuie::delegate_widget!(pane);

    fn override_on_input(&mut self, queue: &mut InputQueue) -> InputResult {
        let Some(event) = queue.next() else {
            return InputResult::Rejected;
        };
        match &event.chord {
            chord!(Enter) => {
                tuie::emit(self.get_id(), ClickEvent);
            }
            chord!(LeftClick) => {
                tuie::focus_widget(self.get_id());
            }
            chord!(LeftRelease) => {
                let size = self.get_rect_size();
                let inside = Axis2D::all(|a| {
                    event.mouse_pos[a] >= 0
                        && event.mouse_pos[a] < size[a] as i32
                });
                if inside {
                    tuie::emit(self.get_id(), ClickEvent);
                }
            }
            _ => return InputResult::Rejected,
        }
        InputResult::Handled
    }

    fn override_measure_constraints(&mut self) -> Constraints {
        self.apply_state_bg();
        self.pane.measure_constraints()
    }

    fn after_on_state_change(&mut self, widget_state: WidgetState) {
        self.state = widget_state;
        self.apply_state_bg();
    }

    fn override_is_focusable(&self) -> bool {
        true
    }
}

impl FlatButton {
    /// Creates an empty flat button with no children.
    pub(crate) fn new() -> Box<Self> {
        Box::new(Self {
            pane: Pane::new(),
            bg: border::config::get().style.bg.unwrap_or(Color::grey256(5)),
            state: WidgetState::None,
            base_style: None,
        })
    }

    /// Appends `child` to the button.
    pub(crate) fn child<T: Widget + 'static>(
        mut self: Box<Self>,
        child: Box<T>,
    ) -> Box<Self> {
        self.pane.add_child(child);
        self
    }
}
