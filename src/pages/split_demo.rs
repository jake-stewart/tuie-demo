//! Drag-resizable split layout demo.

use tuie::prelude::*;

use crate::flat_button::FlatButton;
use crate::theme;

struct Card {
    pane_id: WidgetId<Pane>,
    up_id: WidgetId<FlatButton>,
    down_id: WidgetId<FlatButton>,
    left_id: WidgetId<FlatButton>,
    right_id: WidgetId<FlatButton>,
    close_id: Option<WidgetId<FlatButton>>,
}

fn arrow_button(label: &str) -> Box<FlatButton> {
    FlatButton::new().child(Text::new().content(format!(" {} ", label)))
}

fn spacer() -> Box<Pane> {
    Pane::new().width(3).height(1)
}

fn build_dpad() -> (
    Box<Pane>,
    WidgetId<FlatButton>,
    WidgetId<FlatButton>,
    WidgetId<FlatButton>,
    WidgetId<FlatButton>,
    WidgetId<Pane>,
) {
    let mut up_id = WidgetId::EMPTY;
    let mut down_id = WidgetId::EMPTY;
    let mut left_id = WidgetId::EMPTY;
    let mut right_id = WidgetId::EMPTY;
    let mut center_cell_id = WidgetId::EMPTY;

    let dpad = Pane::new().vertical().children([
        Pane::new().horizontal().children([
            spacer() as Box<dyn Widget>,
            arrow_button("^").id(&mut up_id),
            spacer(),
        ]),
        Pane::new().horizontal().children([
            arrow_button("<").id(&mut left_id) as Box<dyn Widget>,
            Pane::new().width(3).height(1).id(&mut center_cell_id),
            arrow_button(">").id(&mut right_id),
        ]),
        Pane::new().horizontal().children([
            spacer() as Box<dyn Widget>,
            arrow_button("v").id(&mut down_id),
            spacer(),
        ]),
    ]);

    (dpad, up_id, down_id, left_id, right_id, center_cell_id)
}

fn make_intro_card() -> (Box<Pane>, Card) {
    let (dpad, up_id, down_id, left_id, right_id, _) = build_dpad();
    let mut pane_id = WidgetId::EMPTY;

    let card = Pane::new()
        .vertical()
        .flex(1)
        .min_width(18)
        .padding(Spacing::balanced(1))
        .x_place(Place::Middle)
        .children([
            Pane::new().flex(2) as Box<dyn Widget>,
            Pane::new().max_width(60).children([
                Text::new()
                    .content(
                        StyledString::new()
                            .span("Partition space into drag-resizable regions. Each pane respects its own ")
                            .span("min".fg(Color::BLUE))
                            .span(" and ")
                            .span("max".fg(Color::BLUE))
                            .span(" size constraints, distributes leftover space via ")
                            .span("flex".fg(Color::BLUE))
                            .span(" layout, and reflows wrapped text as it resizes. Borders are optional."),
                    )
                    .center()
                    .word_wrap(),
            ]),
            Pane::new().flex(1).min_height(1),
            dpad,
            Pane::new().flex(2),
        ])
        .id(&mut pane_id);

    (
        card,
        Card {
            pane_id,
            up_id,
            down_id,
            left_id,
            right_id,
            close_id: None,
        },
    )
}

fn make_card() -> (Box<Pane>, Card) {
    let (dpad, up_id, down_id, left_id, right_id, center_cell_id) = build_dpad();
    let mut close_id = WidgetId::EMPTY;
    let mut pane_id = WidgetId::EMPTY;

    let mut card = Pane::new()
        .vertical()
        .flex(1)
        .x_place(Place::Middle)
        .y_place(Place::Middle)
        .children([dpad]);

    if let Some(cell) = card.get_widget_mut(center_cell_id) {
        cell.add_child(
            FlatButton::new()
                .child(Text::new().content(" x ".fg(Color::RED).bold()))
                .id(&mut close_id),
        );
    }

    let card = card.id(&mut pane_id);

    (
        card,
        Card {
            pane_id,
            up_id,
            down_id,
            left_id,
            right_id,
            close_id: Some(close_id),
        },
    )
}

/// Drag-resizable split demo with focusable cards.
pub struct SplitDemo {
    split: Box<Split>,
    cards: Vec<Card>,
}

impl SplitDemo {
    fn add_card(&mut self, target: WidgetId, axis: Axis2D, direction: Sign) {
        let (card_pane, card) = make_card();
        self.split
            .split(target, SplitPaneChild::from(card_pane), axis, direction);
        self.cards.push(card);
    }

    fn remove_card(&mut self, target: WidgetId<Pane>) {
        let deletable = self
            .cards
            .iter()
            .any(|c| c.pane_id == target && c.close_id.is_some());
        if !deletable {
            return;
        }
        self.split.remove(target);
        self.cards.retain(|c| c.pane_id != target);

        if let Some(card) = self.cards.first() {
            tuie::focus_widget(card.pane_id);
            tuie::reveal(card.pane_id, tuie::prelude::Vec2 { x: None, y: None });
        }
    }
}

impl DelegateWidget for SplitDemo {
    tuie::delegate_widget!(split);

    fn after_on_state_change(&mut self, _state: WidgetState) {
        for card in &self.cards {
            let focused = tuie::runtime::is_focus_chain(card.pane_id);
            if let Some(leaf) = self.split.get_leaf_mut(card.pane_id) {
                leaf.set_border_style(if focused {
                    theme::get_accent()
                } else {
                    Style::new()
                });
                leaf.set_border(focused.then_some(Border::THICK));
            }
        }
    }

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if !event.of::<ClickEvent>() {
            return;
        }
        let source = event.source;

        let mut to_add: Option<(WidgetId, Axis2D, Sign)> = None;
        let mut to_remove: Option<WidgetId<Pane>> = None;

        for card in &self.cards {
            if source == card.up_id {
                to_add = Some((card.pane_id.untyped(), Axis2D::Y, Sign::Negative));
                break;
            }
            if source == card.down_id {
                to_add = Some((card.pane_id.untyped(), Axis2D::Y, Sign::Positive));
                break;
            }
            if source == card.left_id {
                to_add = Some((card.pane_id.untyped(), Axis2D::X, Sign::Negative));
                break;
            }
            if source == card.right_id {
                to_add = Some((card.pane_id.untyped(), Axis2D::X, Sign::Positive));
                break;
            }
            if let Some(close_id) = card.close_id {
                if source == close_id {
                    to_remove = Some(card.pane_id);
                    break;
                }
            }
        }

        if let Some((target, axis, direction)) = to_add {
            self.add_card(target, axis, direction);
        } else if let Some(target) = to_remove {
            self.remove_card(target);
        }
    }
}

impl SplitDemo {
    /// Creates a [`SplitDemo`] with a single intro card.
    pub fn new() -> Box<Self> {
        let (card_pane, card) = make_intro_card();

        let split = Split::new(
            SplitPane::vertical().children([SplitPaneChild::from(card_pane).title("intro")]),
        )
        .bordered()
        .flex(1);

        Box::new(Self {
            split,
            cards: vec![card],
        })
    }
}

/// Returns the split demo page.
pub fn split_demo_page() -> Box<SplitDemo> {
    SplitDemo::new()
}
