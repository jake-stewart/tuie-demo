//! Layout demo page.

use tuie::prelude::*;
use tuie::render::border;

use crate::counter::Counter;
use crate::segmented_control::SegmentedControl;

const A_COLOR: Color = Color::BLUE;
const B_COLOR: Color = Color::GREEN;
const C_COLOR: Color = Color::YELLOW;
const D_COLOR: Color = Color::MAGENTA;

const STAGE_BG: Color = Color::grey256(1);
const BLOCK_BG: Color = Color::grey256(3);

/// Interactive layout demo page.
pub(crate) struct LayoutDemo {
    root: Box<Pane>,
    a_block_id: WidgetId<Pane>,
    b_block_id: WidgetId<Pane>,
    c_block_id: WidgetId<Pane>,
    a_label_id: WidgetId<Text>,
    b_label_id: WidgetId<Text>,
    c_label_id: WidgetId<Text>,
    a_counter_id: WidgetId<Counter>,
    b_counter_id: WidgetId<Counter>,
    c_counter_id: WidgetId<Counter>,
    align_stage_id: WidgetId<Pane>,
    align_x_toggle_id: WidgetId<SegmentedControl>,
    align_y_toggle_id: WidgetId<SegmentedControl>,
    align_orient_toggle_id: WidgetId<SegmentedControl>,
    content_id: WidgetId<Pane>,
    padding_counter_id: WidgetId<Counter>,
    margin_counter_id: WidgetId<Counter>,
}

struct FlexIds {
    a_block: WidgetId<Pane>,
    b_block: WidgetId<Pane>,
    c_block: WidgetId<Pane>,
    a_label: WidgetId<Text>,
    b_label: WidgetId<Text>,
    c_label: WidgetId<Text>,
    a_counter: WidgetId<Counter>,
    b_counter: WidgetId<Counter>,
    c_counter: WidgetId<Counter>,
}

struct AlignIds {
    stage: WidgetId<Pane>,
    x_toggle: WidgetId<SegmentedControl>,
    y_toggle: WidgetId<SegmentedControl>,
    orient_toggle: WidgetId<SegmentedControl>,
}

impl LayoutDemo {
    fn heading(label: &str) -> Box<Text> {
        Text::new().content(label.fg(Color::BLUE).bold())
    }

    fn caption(text: &str) -> Box<Text> {
        Text::new()
            .content(text)
            .word_wrap()
    }

    fn section(heading_label: &str, caption_text: &str, body: Box<dyn Widget>) -> Box<Pane> {
        Pane::new()
            .vertical()
            .children([
                Self::heading(heading_label),
                Self::caption(caption_text).margin_bottom(1),
                body,
            ])
    }

    fn block_pane(letter: &str, color: Color) -> Box<Pane> {
        Pane::new()
            .vertical()
            .y_place(Place::Middle)
            .bordered()
            .border(Border::THICK)
            .border_style(Style::new().fg(color))
            .style(Style::new().bg(BLOCK_BG))
            .children([Text::new().content(letter.fg(color).bold()).center()])
    }

    fn build_hero() -> Box<Pane> {
        Pane::new()
            .vertical()
            .gap(1)
            .children([
                Text::new()
                    .content(
                        StyledString::new()
                            .span("The layout system handles ")
                            .span("flex".fg(Color::BLUE).bold())
                            .span(", ")
                            .span("fit".fg(Color::BLUE).bold())
                            .span(", ")
                            .span("alignment".fg(Color::BLUE).bold())
                            .span(", ")
                            .span("min, max, and preferred constraints".fg(Color::BLUE).bold())
                            .span(", ")
                            .span("padding".fg(Color::BLUE).bold())
                            .span(", ")
                            .span("margin".fg(Color::BLUE).bold())
                            .span(", ")
                            .span("wrapping".fg(Color::BLUE).bold())
                            .span(", and ")
                            .span("scrolling".fg(Color::BLUE).bold())
                            .span("."),
                    )
                    .word_wrap(),
            ])
    }

    fn build_flex() -> (Box<Pane>, FlexIds) {
        let mut a_block = WidgetId::EMPTY;
        let mut b_block = WidgetId::EMPTY;
        let mut c_block = WidgetId::EMPTY;
        let mut a_label = WidgetId::EMPTY;
        let mut b_label = WidgetId::EMPTY;
        let mut c_label = WidgetId::EMPTY;
        let mut a_counter = WidgetId::EMPTY;
        let mut b_counter = WidgetId::EMPTY;
        let mut c_counter = WidgetId::EMPTY;

        let make_block = |letter: &str, color: Color, weight: u8,
                          block_slot: &mut WidgetId<Pane>,
                          label_slot: &mut WidgetId<Text>| {
            Self::block_pane(letter, color)
                .flex(weight)
                .children([
                    Text::new()
                        .content(format!("flex {}", weight).fg(Color::grey256(11)))
                        .center()
                        .id(label_slot),
                ])
                .id(block_slot)
        };

        let body = Pane::new()
            .vertical()
            .gap(1)
            .children([
                Pane::new()
                    .horizontal()
                    .gap(1)
                    .height(5)
                    .style(Style::new().bg(STAGE_BG))
                    .children([
                        make_block("A", A_COLOR, 1, &mut a_block, &mut a_label),
                        make_block("B", B_COLOR, 2, &mut b_block, &mut b_label),
                        make_block("C", C_COLOR, 3, &mut c_block, &mut c_label),
                    ]) as Box<dyn Widget>,
                Pane::new()
                    .horizontal()
                    .gap(3)
                    .x_place(Place::Middle)
                    .children([
                        Counter::new("A").value(1).min(0).max(5).color(A_COLOR).id(&mut a_counter),
                        Counter::new("B").value(2).min(0).max(5).color(B_COLOR).id(&mut b_counter),
                        Counter::new("C").value(3).min(0).max(5).color(C_COLOR).id(&mut c_counter),
                    ]),
            ]);

        let pane = Self::section(
            "Flex",
            "Children divide leftover space in proportion to their flex weight.",
            body,
        );

        (
            pane,
            FlexIds {
                a_block,
                b_block,
                c_block,
                a_label,
                b_label,
                c_label,
                a_counter,
                b_counter,
                c_counter,
            },
        )
    }

    fn build_align() -> (Box<Pane>, AlignIds) {
        let mut stage = WidgetId::EMPTY;
        let mut x_toggle = WidgetId::EMPTY;
        let mut y_toggle = WidgetId::EMPTY;
        let mut orient_toggle = WidgetId::EMPTY;

        let chip = |letter: &str, color: Color| -> Box<dyn Widget> {
            Pane::new()
                .vertical()
                .y_place(Place::Middle)
                .horizontal_padding(2)
                .bordered()
                .border(Border::THICK)
                .border_style(Style::new().fg(color))
                .style(Style::new().bg(BLOCK_BG))
                .children([Text::new().content(letter.fg(color).bold()).center()])
                as Box<dyn Widget>
        };

        let stage_pane = Pane::new()
            .horizontal()
            .gap(1)
            .height(15)
            .x_place(Place::Middle)
            .y_place(Place::Middle)
            .style(Style::new().bg(STAGE_BG))
            .children([
                chip("A", A_COLOR),
                chip("B", B_COLOR),
                chip("C", C_COLOR),
            ])
            .id(&mut stage);

        let row = |label: &str, ctrl: Box<dyn Widget>| -> Box<Pane> {
            Pane::new()
                .horizontal()
                .gap(2)
                .children([
                    Pane::new()
                        .horizontal()
                        .width(13)
                        .children([Text::new().content(label.fg(Color::grey256(11)))])
                        as Box<dyn Widget>,
                    ctrl,
                ])
        };

        let body = Pane::new()
            .vertical()
            .gap(1)
            .children([
                stage_pane as Box<dyn Widget>,
                row(
                    "x_place",
                    SegmentedControl::new(&["Start", "Middle", "End", "Stretch", "Evenly", "Apart"])
                        .selected(1)
                        .disabled(3)
                        .id(&mut x_toggle),
                ),
                row(
                    "y_place",
                    SegmentedControl::new(&["Start", "Middle", "End", "Stretch", "Evenly", "Apart"])
                        .selected(1)
                        .disabled(4)
                        .disabled(5)
                        .id(&mut y_toggle),
                ),
                row(
                    "orientation",
                    SegmentedControl::new(&["Horizontal", "Vertical"])
                        .selected(0)
                        .id(&mut orient_toggle),
                ),
            ]);

        let pane = Self::section(
            "Place",
            "Each axis places its items independently: Start, Middle, End, Stretch, Evenly, or Apart. x_place always means horizontal position, regardless of orientation. Evenly/Apart are only valid on the main axis; Stretch is only valid on the cross axis.",
            body,
        );

        (
            pane,
            AlignIds {
                stage,
                x_toggle,
                y_toggle,
                orient_toggle,
            },
        )
    }

    fn build_constraints() -> Box<Pane> {
        let b = border::config::get().border;
        let h = b.get_edge(Axis2D::Y);
        let dashes = |n: usize| h.to_string().repeat(n);

        let t_right = b.get_arms(false, true, true, true).to_string();
        let t_left = b.get_arms(true, false, true, true).to_string();
        let push = |bar: &mut StyledString, text: &str, color: Color| {
            bar.push_span(StyledStr::new(text).fg(color).bold());
        };
        let mut bar = StyledString::new();
        push(&mut bar, &t_right, Color::BLUE);
        push(&mut bar, &format!("{} min {}", dashes(3), dashes(3)), Color::BLUE);
        push(&mut bar, &t_left, Color::BLUE);
        push(&mut bar, &format!("{} preferred {}", dashes(3), dashes(3)), Color::GREEN);
        push(&mut bar, &t_left, Color::GREEN);
        push(&mut bar, &format!("{} max {}", dashes(3), dashes(3)), Color::RED);
        push(&mut bar, &t_left, Color::RED);

        let left = Pane::new()
            .horizontal()
            .min_width(13)
            .preferred_width(31)
            .max_width(43)
            .x_place(Place::Start)
            .style(Style::new().bg(BLOCK_BG))
            .children([Text::new().content(bar).truncate().left()]);

        let right = Pane::new()
            .vertical()
            .y_place(Place::Middle)
            .flex(1)
            .children([
                Text::new()
                    .content(format!(" <{} drag to resize", dashes(4)).dim())
                    .left(),
            ]);

        let split = Split::new(
            SplitPane::horizontal()
                .children([
                    SplitPaneChild::from(left).border(Border::THICK_DASHED),
                    SplitPaneChild::from(right).border(Border::THICK_DASHED),
                ]),
        );

        Self::section(
            "Constraints",
            "Every element has a min, max, and preferred size.",
            split,
        )
    }

    fn build_padding_margin() -> (
        Box<Pane>,
        WidgetId<Pane>,
        WidgetId<Counter>,
        WidgetId<Counter>,
    ) {
        let mut content_id = WidgetId::EMPTY;
        let mut padding_id = WidgetId::EMPTY;
        let mut margin_id = WidgetId::EMPTY;

        let mut inner_content = Pane::new()
            .vertical()
            .y_place(Place::Middle)
            .bordered()
            .border(Border::THICK)
            .border_style(Style::new().fg(C_COLOR))
            .style(Style::new().bg(BLOCK_BG))
            .padding(Spacing::balanced(3))
            .margin(Spacing::balanced(3))
            .children([Text::new().content("Content".fg(C_COLOR).bold()).center()])
            .id(&mut content_id);
        inner_content.set_x_align(FlexAlign::Middle);

        let mut outer_bordered = Pane::new()
            .vertical()
            .y_place(Place::Middle)
            .bordered()
            .border(Border::THICK)
            .border_style(Style::new().fg(A_COLOR))
            .style(Style::new().bg(STAGE_BG))
            .children([inner_content]);
        outer_bordered.set_x_align(FlexAlign::Middle);

        let body = Pane::new()
            .vertical()
            .gap(1)
            .children([
                Pane::new()
                    .horizontal()
                    .gap(3)
                    .x_place(Place::Middle)
                    .children([
                        Counter::new("Padding").value(3).min(0).max(10).id(&mut padding_id),
                        Counter::new("Margin").value(3).min(0).max(10).id(&mut margin_id),
                    ]) as Box<dyn Widget>,
                outer_bordered,
            ]);

        let pane = Self::section(
            "Padding & Margin",
            "Padding adds space inside the border. Margin adds space around it.",
            body,
        );

        (pane, content_id, padding_id, margin_id)
    }

    fn build_wrap() -> Box<Pane> {
        let labels: &[&str] = &[
            "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];

        let make_chip = |label: &str| -> Box<dyn Widget> {
            Pane::new()
                .horizontal()
                .bordered()
                .border(Border::SINGLE)
                .border_style(Style::new().fg(Color::grey256(8)))
                .horizontal_padding(1)
                .children([Text::new().content(label.fg(Color::CYAN))]) as Box<dyn Widget>
        };

        let chips = |balanced: bool| -> Box<Pane> {
            let mut wrap = Pane::new()
                .horizontal()
                .wrap()
                .gap(1)
                .x_place(Place::Middle);
            if balanced {
                wrap = wrap.balanced();
            }
            for label in labels {
                wrap = wrap.child(make_chip(label));
            }
            let title = if balanced {
                "Balanced".fg(Color::YELLOW).bold()
            } else {
                "Greedy".fg(Color::YELLOW).bold()
            };
            Pane::new()
                .vertical()
                .flex(1)
                .gap(1)
                .horizontal_padding(1)
                .children([
                    Text::new().content(title).center() as Box<dyn Widget>,
                    wrap,
                ])
        };

        let split = Split::new(
            SplitPane::horizontal()
                .children([
                    SplitPaneChild::from(chips(false).preferred_width(10)).border(Border::THICK_DASHED),
                    SplitPaneChild::from(chips(true).preferred_width(10)).border(Border::THICK_DASHED),
                ]),
        ).bordered();

        Self::section(
            "Wrap",
            "Children flow along the main axis and break when they overflow. Greedy fills each line. Balanced evens out line widths.",
            split,
        )
    }

    fn build_nested_scroll() -> Box<Pane> {
        let mut list = Pane::new().vertical();
        for i in 1..=40 {
            let bg = if i % 2 == 0 {
                Style::new().bg(Color::grey256(1))
            } else {
                Style::new()
            };
            let row = Pane::new()
                .horizontal()
                .padding(Spacing::new().horizontal(2))
                .style(bg)
                .children([
                    Text::new().content(
                        StyledString::new()
                            .span(format!("{:>3}  ", i).fg(Color::grey256(8)))
                            .span(format!("entry {}", i).fg(Color::Foreground)),
                    ),
                ]);
            list = list.children([row]);
        }

        let inner_scroll = Pane::new()
            .vertical()
            .height(14)
            .bordered()
            .border(Border::SINGLE)
            .border_style(Style::new().fg(Color::CYAN))
            .y_scroll(Scrollbar::Visible)
            .children([list]);

        Self::section(
            "Nested scroll",
            "Scroll containers nest: the inner pane scrolls when hovered, the outer page scrolls otherwise.",
            inner_scroll,
        )
    }
}

impl DelegateWidget for LayoutDemo {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if let Some(&ChangeEvent(value)) = event.get_by::<ChangeEvent<i32>>(self.padding_counter_id) {
            if let Some(pane) = self.root.get_widget_mut(self.content_id) {
                pane.set_padding(Spacing::balanced(value.max(0) as u8));
            }
        } else if let Some(&ChangeEvent(value)) = event.get_by::<ChangeEvent<i32>>(self.margin_counter_id) {
            if let Some(pane) = self.root.get_widget_mut(self.content_id) {
                pane.set_margin(Spacing::balanced(value.max(0) as u8));
            }
        } else if event.get_by::<ChangeEvent<usize>>(self.align_x_toggle_id).is_some()
            || event.get_by::<ChangeEvent<usize>>(self.align_y_toggle_id).is_some()
        {
            self.reapply_alignment();
        } else if let Some(&ChangeEvent(idx)) = event.get_by::<ChangeEvent<usize>>(self.align_orient_toggle_id) {
            let axis = if idx == 0 { Axis2D::X } else { Axis2D::Y };
            if let Some(stage) = self.root.get_widget_mut(self.align_stage_id) {
                stage.set_orientation(axis);
            }
            self.refresh_align_constraints();
            self.reapply_alignment();
        } else if let Some(&ChangeEvent(value)) = event.get::<ChangeEvent<i32>>() {
            let source = event.source;
            let (block_id, label_id) = if source == self.a_counter_id {
                (self.a_block_id, self.a_label_id)
            } else if source == self.b_counter_id {
                (self.b_block_id, self.b_label_id)
            } else if source == self.c_counter_id {
                (self.c_block_id, self.c_label_id)
            } else {
                return;
            };

            let weight = value.max(0) as u8;
            if let Some(block) = self.root.get_widget_mut(block_id) {
                block.set_flex(weight);
                block.dirty_layout();
            }
            if let Some(label) = self.root.get_widget_mut(label_id) {
                label.set_content(format!("flex {}", weight).fg(Color::grey256(11)));
            }
        }
    }
}

impl LayoutDemo {
    fn refresh_align_constraints(&mut self) {
        let axis = self.root
            .get_widget(self.align_orient_toggle_id)
            .map(|t| if t.get_selected() == 0 { Axis2D::X } else { Axis2D::Y })
            .unwrap_or(Axis2D::X);
        let (main_id, cross_id) = match axis {
            Axis2D::X => (self.align_x_toggle_id, self.align_y_toggle_id),
            Axis2D::Y => (self.align_y_toggle_id, self.align_x_toggle_id),
        };

        let mut snap = false;
        if let Some(t) = self.root.get_widget_mut(main_id) {
            t.set_disabled(0, false);
            t.set_disabled(1, false);
            t.set_disabled(2, false);
            t.set_disabled(3, true);
            t.set_disabled(4, false);
            t.set_disabled(5, false);
            if t.get_selected() == 3 {
                t.set_selected(0);
                snap = true;
            }
        }
        if let Some(t) = self.root.get_widget_mut(cross_id) {
            t.set_disabled(0, false);
            t.set_disabled(1, false);
            t.set_disabled(2, false);
            t.set_disabled(3, false);
            t.set_disabled(4, true);
            t.set_disabled(5, true);
            let idx = t.get_selected();
            if idx == 4 || idx == 5 {
                t.set_selected(0);
                snap = true;
            }
        }
        if snap {
            self.reapply_alignment();
        }
    }

    fn reapply_alignment(&mut self) {
        let x_idx = self.root
            .get_widget(self.align_x_toggle_id)
            .map(|t| t.get_selected())
            .unwrap_or(0);
        let y_idx = self.root
            .get_widget(self.align_y_toggle_id)
            .map(|t| t.get_selected())
            .unwrap_or(0);
        let from_toggle = |idx| match idx {
            0 => Place::Start,
            1 => Place::Middle,
            2 => Place::End,
            3 => Place::Stretch,
            4 => Place::Evenly,
            _ => Place::Apart,
        };
        let x_place = from_toggle(x_idx);
        let y_place = from_toggle(y_idx);
        if let Some(stage) = self.root.get_widget_mut(self.align_stage_id) {
            stage.set_x_place(x_place);
            stage.set_y_place(y_place);
        }
    }

    /// Creates the layout demo.
    pub(crate) fn new() -> Box<Self> {
        let hero = Self::build_hero();
        let (flex_section, flex_ids) = Self::build_flex();
        let (align_section, align_ids) = Self::build_align();
        let constraints_section = Self::build_constraints();
        let (padding_margin_section, content_id, padding_counter_id, margin_counter_id) =
            Self::build_padding_margin();
        let nested_scroll_section = Self::build_nested_scroll();
        let wrap_section = Self::build_wrap();

        let root = Pane::new()
            .vertical()
            .gap(3)
            .flex(1)
            .children([
                hero,
                flex_section,
                align_section,
                constraints_section,
                padding_margin_section,
                nested_scroll_section,
                wrap_section,
            ]);

        Box::new(Self {
            root,
            a_block_id: flex_ids.a_block,
            b_block_id: flex_ids.b_block,
            c_block_id: flex_ids.c_block,
            a_label_id: flex_ids.a_label,
            b_label_id: flex_ids.b_label,
            c_label_id: flex_ids.c_label,
            a_counter_id: flex_ids.a_counter,
            b_counter_id: flex_ids.b_counter,
            c_counter_id: flex_ids.c_counter,
            align_stage_id: align_ids.stage,
            align_x_toggle_id: align_ids.x_toggle,
            align_y_toggle_id: align_ids.y_toggle,
            align_orient_toggle_id: align_ids.orient_toggle,
            content_id,
            padding_counter_id,
            margin_counter_id,
        })
    }
}

/// Returns a [`LayoutDemo`] page.
pub(crate) fn layout_demo_page() -> Box<LayoutDemo> {
    LayoutDemo::new()
}
