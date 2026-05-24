//! Demo page for the [`Input`] widget.

use tuie::prelude::*;
use crate::checkbox::Checkbox;
use crate::counter::Counter;
use crate::focus_pane::FocusPane;
use crate::radio_group::RadioGroup;

const BINDINGS: [InputBindingsFactory; 4] = [
    DefaultBindings::new,
    ModernBindings::new,
    ViBindings::new,
    EmacsBindings::new,
];

const OVERFLOWS: [&TextOverflow; 3] = [
    TextOverflow::VISIBLE,
    TextOverflow::WRAP,
    TextOverflow::WORD_WRAP,
];

/// Demo page for the [`Input`] widget.
pub struct InputDemo {
    root: Box<Pane>,
    bindings_toggle_id: WidgetId<RadioGroup>,
    overflow_toggle_id: WidgetId<RadioGroup>,
    scrollbars_checkbox_id: WidgetId<Checkbox>,
    cursor_blink_checkbox_id: WidgetId<Checkbox>,
    expandtabs_checkbox_id: WidgetId<Checkbox>,
    ml_input_id: WidgetId<Input>,
    sl_input_id: WidgetId<Input>,
    inline_input_id: WidgetId<Input>,
    ml_scroll_id: WidgetId<Pane>,
    sl_scroll_id: WidgetId<Pane>,
    scrolloff_counter_id: WidgetId<Counter>,
    tabstop_counter_id: WidgetId<Counter>,
}

impl DelegateWidget for InputDemo {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if let Some(ChangeEvent(index)) = event.get_by::<ChangeEvent<usize>>(self.bindings_toggle_id) {
            let factory = BINDINGS[*index];
            for id in [self.ml_input_id, self.sl_input_id, self.inline_input_id] {
                if let Some(input) = self.root.get_widget_mut(id) {
                    input.set_bindings(factory);
                }
            }
        } else if let Some(ChangeEvent(index)) = event.get_by::<ChangeEvent<usize>>(self.overflow_toggle_id) {
            let overflow = OVERFLOWS[*index];
            for id in [self.ml_input_id, self.sl_input_id] {
                if let Some(input) = self.root.get_widget_mut(id) {
                    input.set_overflow(overflow);
                }
            }
        } else if let Some(ChangeEvent(checked)) = event.get_by::<ChangeEvent<bool>>(self.scrollbars_checkbox_id) {
            let mode = if *checked { Scrollbar::AutoHide } else { Scrollbar::Hidden };
            for id in [self.ml_scroll_id, self.sl_scroll_id] {
                if let Some(pane) = self.root.get_widget_mut(id) {
                    pane.set_y_scroll(Some(mode));
                    pane.set_x_scroll(Some(mode));
                }
            }
        } else if let Some(ChangeEvent(checked)) = event.get_by::<ChangeEvent<bool>>(self.cursor_blink_checkbox_id) {
            tuie::config::update(|c| c.cursor_blink = *checked);
        } else if let Some(ChangeEvent(checked)) = event.get_by::<ChangeEvent<bool>>(self.expandtabs_checkbox_id) {
            tuie::config::update(|c| c.expandtabs = *checked);
        } else if let Some(ChangeEvent(value)) = event.get_by::<ChangeEvent<i32>>(self.scrolloff_counter_id) {
            tuie::config::update(|c| c.scrolloff = *value as u16);
        } else if let Some(ChangeEvent(value)) = event.get_by::<ChangeEvent<i32>>(self.tabstop_counter_id) {
            tuie::config::update(|c| c.tabstop = *value as u8);
        }
    }
}

impl InputDemo {
    /// Creates the [`InputDemo`] page.
    pub fn new() -> Box<Self> {
        let config = tuie::config::get();

        let mut bindings_toggle_id = WidgetId::EMPTY;
        let mut overflow_toggle_id = WidgetId::EMPTY;
        let mut scrollbars_checkbox_id = WidgetId::EMPTY;
        let mut cursor_blink_checkbox_id = WidgetId::EMPTY;
        let mut expandtabs_checkbox_id = WidgetId::EMPTY;
        let mut ml_input_id = WidgetId::EMPTY;
        let mut sl_input_id = WidgetId::EMPTY;
        let mut inline_input_id = WidgetId::EMPTY;
        let mut ml_scroll_id = WidgetId::EMPTY;
        let mut sl_scroll_id = WidgetId::EMPTY;
        let mut scrolloff_counter_id = WidgetId::EMPTY;
        let mut tabstop_counter_id = WidgetId::EMPTY;

        let ml_pane = FocusPane::new().children([
            Pane::new()
                .min_height(5)
                .max_height(10)
                .children([Input::new().word_wrap().multiline().flex(1).id(&mut ml_input_id)])
                .y_scroll(Scrollbar::AutoHide)
                .x_scroll(Scrollbar::AutoHide)
                .id(&mut ml_scroll_id),
        ]);

        let sl_pane = FocusPane::new().children([
            Pane::new()
                .horizontal()
                .max_height(3)
                .children([Input::new().word_wrap().flex(1).id(&mut sl_input_id)])
                .x_scroll(Scrollbar::AutoHide)
                .y_scroll(Scrollbar::AutoHide)
                .id(&mut sl_scroll_id),
        ]);

        let inline_row = Pane::new()
            .horizontal()
            .children([
                Text::new().content("Hello ") as Box<dyn Widget>,
                Input::new()
                    .margin_right(0)
                    .overflow(TextOverflow::VISIBLE)
                    .style(Style::new().underline(UnderlineType::Single))
                    .placeholder(Text::new().content("Name".dim()).style(Style::new().underline(UnderlineType::Single)))
                    .id(&mut inline_input_id),
                Text::new().content(", how are you?"),
            ])
            .x_scroll(Scrollbar::AutoHide);

        let left = Pane::new()
            .vertical()
            .flex(2)
            .preferred_width(30)
            .gap(1)
            .children([
                Text::new().content("Multiline".bold()) as Box<dyn Widget>,
                ml_pane,
                Text::new().content("Single Line".bold()),
                sl_pane,
                Text::new().content("Inline".bold()),
                inline_row,
            ]);

        let right = Pane::new()
            .vertical()
            .flex(1)
            .bordered()
            .horizontal_padding(1)
            .y_align(FlexAlign::Start)
            .max_width(30)
            .children([
                Text::new().content("Bindings".bold()) as Box<dyn Widget>,
                RadioGroup::new(&["Default", "Modern", "Vi", "Emacs"])
                    .id(&mut bindings_toggle_id),
                Text::new().content("Overflow".bold()).margin_top(1),
                RadioGroup::new(&["Scroll", "Wrap", "Word Wrap"])
                    .selected(2)
                    .id(&mut overflow_toggle_id),
                Text::new().content("Tabs".bold()).margin_top(1),
                Counter::new("Tabstop")
                    .value(config.tabstop as i32)
                    .min(1)
                    .max(16)
                    .id(&mut tabstop_counter_id),
                Checkbox::new(Text::new().content("Expand tabs"))
                    .checked_if(config.expandtabs)
                    .id(&mut expandtabs_checkbox_id),
                Text::new().content("Appearance".bold()).margin_top(1),
                Checkbox::new(Text::new().content("Scrollbars"))
                    .checked()
                    .id(&mut scrollbars_checkbox_id),
                Checkbox::new(Text::new().content("Cursor Blink"))
                    .checked_if(config.cursor_blink)
                    .id(&mut cursor_blink_checkbox_id),
                Counter::new("Scrolloff")
                    .value(config.scrolloff as i32)
                    .min(0)
                    .max(999)
                    .id(&mut scrolloff_counter_id),
            ]);

        let intro = Text::new()
            .content(
                "Input is a text editing widget with cursor, selection, undo/redo, and word navigation.",
            )
            .word_wrap();

        let root = Pane::new()
            .vertical()
            .flex(1)
            .children([
                intro.margin_bottom(1) as Box<dyn Widget>,
                Pane::new()
                    .horizontal()
                    .flex(1)
                    .gap(2)
                    .children([left, right]),
            ]);

        Box::new(Self {
            root,
            bindings_toggle_id,
            overflow_toggle_id,
            scrollbars_checkbox_id,
            cursor_blink_checkbox_id,
            expandtabs_checkbox_id,
            scrolloff_counter_id,
            tabstop_counter_id,
            ml_input_id,
            sl_input_id,
            inline_input_id,
            ml_scroll_id,
            sl_scroll_id,
        })
    }
}
