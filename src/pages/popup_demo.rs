//! Popup demo page.

use std::cell::Cell;
use std::rc::Rc;

use tuie::prelude::*;

use crate::button::Button;
use crate::checkbox::Checkbox;
use crate::flat_button::FlatButton;
use crate::point_picker::PointPicker;

struct PopupHost {
    root: Box<Pane>,
    on_event: Box<dyn Fn(&WidgetEvent)>,
}

impl PopupHost {
    fn new(root: Box<Pane>, on_event: impl Fn(&WidgetEvent) + 'static) -> Box<Self> {
        Box::new(Self {
            root,
            on_event: Box::new(on_event),
        })
    }
}

impl DelegateWidget for PopupHost {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        (self.on_event)(event);
    }
}

fn heading(label: &str) -> Box<Text> {
    Text::new().content(label.fg(Color::BLUE).bold()).center()
}

fn picker_cell(label: &str, picker: Box<PointPicker>) -> Box<Pane> {
    Pane::new()
        .vertical()
        .flex(1)
        .gap(1)
        .children([
            Pane::new().horizontal().x_place(Place::Center).children([picker]),
            Text::new()
                .content(label.fg(Color::grey256(10)))
                .x_align(FlexAlign::Center),
        ])
}

fn popup_body(content: Box<dyn Widget>, max_width: u16) -> Box<Pane> {
    Pane::new()
        .style(Style::new().bg(Color::grey256(3)).blend(95))
        .padding(Spacing::new().horizontal(2).vertical(1))
        .max_width(max_width)
        .vertical()
        .children([content])
}

fn point_label(p: Vec2<u16>) -> &'static str {
    match (p.x, p.y) {
        (0, 0) => "top left",
        (1, 0) => "top",
        (2, 0) => "top right",
        (0, 1) => "left",
        (1, 1) => "center",
        (2, 1) => "right",
        (0, 2) => "bottom left",
        (1, 2) => "bottom",
        (2, 2) => "bottom right",
        _ => "?",
    }
}

fn placement_for(anchor: Vec2<u16>, popup: Vec2<u16>) -> Placement {
    let to_align = |v: u16| match v {
        0 => Align::Start,
        1 => Align::Center,
        _ => Align::End,
    };
    Placement::center()
        .anchor_point(anchor.map(to_align))
        .popup_point(popup.map(to_align))
}

fn write_tooltip_body(text: &mut Text, anchor: Vec2<u16>, popup: Vec2<u16>) {
    text.clear_content();
    text.push("The ");
    text.push(format!("{} of this popup", point_label(popup)).fg(Color::YELLOW).bold());
    text.push(" is attached to the ");
    text.push(format!("{} of the anchor", point_label(anchor)).fg(Color::MAGENTA).bold());
}

fn open_modal() {
    let mut ok_id = WidgetId::EMPTY;
    let body = Pane::new()
        .style(Style::new().bg(Color::grey256(3)).blend(95))
        .width(52)
        .vertical()
        .padding(Spacing::balanced(2))
        .gap(1)
        .children([
            Text::new().content("Modal Popup".fg(Color::Foreground).bold()) as Box<dyn Widget>,
            Text::new()
                .content("Input, clicks, and focus stay inside this modal.")
                .word_wrap(),
            Pane::new().min_height(1),
            Pane::new().horizontal().x_place(Place::End).children([
                Button::new().children([Text::new().content(" Ok ")]).id(&mut ok_id),
            ]),
        ]);

    let popup_id: Rc<Cell<Option<WidgetId>>> = Rc::new(Cell::new(None));
    let popup_id_ref = popup_id.clone();
    let host = PopupHost::new(body, move |event| {
        if event.of_by::<ClickEvent>(ok_id) {
            if let Some(id) = popup_id_ref.get() {
                tuie::close_popup(id);
            }
        }
    });
    popup_id.set(Some(host.get_id().untyped()));
    tuie::open_popup(Popup::new(host));
}

/// Interactive popup demo page.
pub(crate) struct PopupDemo {
    root: Box<Pane>,
    tooltip_button_id: WidgetId<Button>,
    tooltip_id: WidgetId<Tooltip>,
    tooltip_body_text_id: WidgetId<Text>,
    dialog_button_id: WidgetId<Button>,
    help_button_id: WidgetId<FlatButton>,
    help_tooltip_id: WidgetId<Tooltip>,
    anchor_picker_id: WidgetId<PointPicker>,
    popup_picker_id: WidgetId<PointPicker>,
    top_checkbox_id: WidgetId<Checkbox>,
    anchor_point: Vec2<u16>,
    popup_point: Vec2<u16>,
}

impl PopupDemo {
    fn sync_tooltip_placement(&mut self) {
        let placement = placement_for(self.anchor_point, self.popup_point);
        if let Some(tooltip) = self.root.get_widget_mut(self.tooltip_id) {
            tooltip.set_placement(placement);
        }
        if let Some(text) = self.root.get_widget_mut(self.tooltip_body_text_id) {
            write_tooltip_body(text, self.anchor_point, self.popup_point);
        }
    }
}

impl DelegateWidget for PopupDemo {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if let Some(ChangeEvent(point)) = event.get_by::<ChangeEvent<Vec2<u16>>>(self.anchor_picker_id) {
            self.anchor_point = *point;
            self.sync_tooltip_placement();
        } else if let Some(ChangeEvent(point)) = event.get_by::<ChangeEvent<Vec2<u16>>>(self.popup_picker_id) {
            self.popup_point = *point;
            self.sync_tooltip_placement();
        } else if let Some(ChangeEvent(on)) = event.get_by::<ChangeEvent<bool>>(self.top_checkbox_id) {
            if let Some(tooltip) = self.root.get_widget_mut(self.tooltip_id) {
                tooltip.set_on_top(*on);
            }
        } else if event.of_by::<ClickEvent>(self.tooltip_button_id) {
            let placement = placement_for(self.anchor_point, self.popup_point);
            if let Some(tooltip) = self.root.get_widget_mut(self.tooltip_id) {
                let visible = !tooltip.is_visible();
                tooltip.set_placement(placement);
                tooltip.set_visible(visible);
            }
            if let Some(text) = self.root.get_widget_mut(self.tooltip_body_text_id) {
                write_tooltip_body(text, self.anchor_point, self.popup_point);
            }
        } else if event.of_by::<ClickEvent>(self.dialog_button_id) {
            open_modal();
        } else if event.of_by::<ClickEvent>(self.help_button_id) {
            if let Some(tooltip) = self.root.get_widget_mut(self.help_tooltip_id) {
                let visible = !tooltip.is_visible();
                tooltip.set_visible(visible);
            }
        }
    }
}

impl PopupDemo {
    /// Creates a [`PopupDemo`] page.
    pub(crate) fn new() -> Box<Self> {
        let anchor_point = Vec2::new(1u16, 0u16);
        let popup_point = Vec2::new(1u16, 2u16);

        let mut tooltip_button_id = WidgetId::EMPTY;
        let mut tooltip_id = WidgetId::EMPTY;
        let mut tooltip_body_text_id = WidgetId::EMPTY;
        let mut dialog_button_id = WidgetId::EMPTY;
        let mut help_button_id = WidgetId::EMPTY;
        let mut help_tooltip_id = WidgetId::EMPTY;
        let mut anchor_picker_id = WidgetId::EMPTY;
        let mut popup_picker_id = WidgetId::EMPTY;
        let mut top_checkbox_id = WidgetId::EMPTY;

        let mut tooltip_text = Text::new()
            .word_wrap()
            .center()
            .id(&mut tooltip_body_text_id);
        write_tooltip_body(&mut tooltip_text, anchor_point, popup_point);

        let tooltip_section = Pane::new()
            .vertical()
            .gap(1)
            .children([
                heading("Anchored tooltip") as Box<dyn Widget>,
                Text::new()
                    .content(
                        StyledString::new()
                            .span("The latch point on the ")
                            .span("anchor".fg(Color::MAGENTA).bold())
                            .span(" attaches to the latch point on the ")
                            .span("popup".fg(Color::YELLOW).bold())
                            .span("."),
                    )
                    .center()
                    .word_wrap()
                    .max_width(50).x_align(FlexAlign::Center),
                Pane::new()
                    .horizontal()
                    .padding_top(1)
                    .x_place(Place::Center)
                    .children([
                        Tooltip::new(
                            Button::new()
                                .children([Text::new().content(" Show tooltip ".fg(Color::MAGENTA))])
                                .selected_border_style(Style::new().fg(Color::MAGENTA))
                                .border_style(Style::new().fg(Color::MAGENTA))
                                .id(&mut tooltip_button_id),
                        )
                        .content(popup_body(tooltip_text, 40))
                        .placement(placement_for(anchor_point, popup_point))
                        .autohide()
                        .id(&mut tooltip_id),
                    ]),
                Pane::new()
                    .horizontal()
                    .gap(2)
                    .padding_top(1)
                    .children([
                        picker_cell(
                            "Popup latch point",
                            PointPicker::new().point(popup_point).id(&mut popup_picker_id),
                        ),
                        picker_cell(
                            "Anchor latch point",
                            PointPicker::new().point(anchor_point).id(&mut anchor_picker_id),
                        ),
                    ]),
            ]);

        let tracking_section = Pane::new()
            .vertical()
            .gap(1)
            .children([
                Text::new()
                    .content(
                        StyledString::new()
                            .span("The tooltip tracks its anchor when the pane scrolls and ")
                            .span("escapes parent bounds".fg(Color::YELLOW).bold())
                            .span("."),
                    )
                    .word_wrap() as Box<dyn Widget>,
                Checkbox::new(Text::new().content("Render tooltip on Top layer"))
                    .id(&mut top_checkbox_id),
            ]);

        let modal_section = Pane::new()
            .vertical()
            .gap(1)
            .children([
                heading("Modal dialog") as Box<dyn Widget>,
                Text::new()
                    .content(
                        StyledString::new()
                            .span("Anchorless".fg(Color::YELLOW).bold())
                            .span(" popups. They center and keep input, clicks, and focus inside."),
                    )
                    .word_wrap(),
                Pane::new().horizontal().padding_top(1).children([
                    Button::new()
                        .children([Text::new().content(" Open dialog ")])
                        .id(&mut dialog_button_id),
                ]),
            ]);

        let bordered_scroll = Pane::new()
            .vertical()
            .flex(1)
            .bordered()
            .border_style(Style::new().fg(Color::grey256(8)))
            .children([
                Pane::new()
                    .vertical()
                    .flex(1)
                    .y_scroll(Scrollbar::AutoHide)
                    .children([
                        Pane::new()
                            .vertical()
                            .gap(2)
                            .max_width(68)
                            .x_align(FlexAlign::Center)
                            .margin(Spacing::balanced(2))
                            .children([tooltip_section, tracking_section, modal_section]),
                    ]),
            ]);

        let help_layer = Pane::new()
            .flex(1)
            .vertical()
            .y_place(Place::End)
            .x_place(Place::End)
            .margin(Spacing::new().bottom(2).right(5))
            .children([
                Pane::new().horizontal().children([
                    Tooltip::new(
                        FlatButton::new()
                            .child(Pane::new().padding(Spacing::balanced(3)).children([Text::new().content("?")]))
                            .style(Style::new().blend(95))
                            .id(&mut help_button_id),
                    )
                    .content(popup_body(
                        Text::new()
                            .content(
                                StyledString::new()
                                    .span("A Stack ")
                                    .span("layer".fg(Color::YELLOW).bold())
                                    .span(" pinned above the content. It stays fixed during scroll."),
                            )
                            .word_wrap(),
                        54,
                    ))
                    .placement(Placement::side(Direction2D::Up, Sign::Positive, Align::End))
                    .autohide()
                    .id(&mut help_tooltip_id),
                ]),
            ]);

        let root = Pane::new().vertical().flex(1).children([
            Stack::new(bordered_scroll)
                .flex(1)
                .max_height(30)
                .children([help_layer]),
        ]);

        Box::new(Self {
            root,
            tooltip_button_id,
            tooltip_id,
            tooltip_body_text_id,
            dialog_button_id,
            help_button_id,
            help_tooltip_id,
            anchor_picker_id,
            popup_picker_id,
            top_checkbox_id,
            anchor_point,
            popup_point,
        })
    }
}

/// Returns a `PopupDemo` page.
pub(crate) fn popup_demo_page() -> Box<PopupDemo> {
    PopupDemo::new()
}
