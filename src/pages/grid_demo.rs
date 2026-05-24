//! [`Grid`] widget demo page.

use tuie::prelude::*;

use crate::checkbox::Checkbox;
use crate::counter::Counter;
use crate::radio_group::RadioGroup;

const BORDER_LABELS: [&str; 5] = ["None", "Single", "Thick", "Double", "Dashed"];

const BORDERS: [Option<&'static Border>; 5] = [
    None,
    Some(Border::SINGLE),
    Some(Border::THICK),
    Some(Border::DOUBLE),
    Some(Border::DASHED),
];

const VARIANTS: [&str; 5] = ["lorem", "ipsum", "dolor", "sit", "amet"];

const DATA: &[(&str, &str, u32, usize)] = &[
    (
        "Lorem",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        1420,
        2,
    ),
    (
        "Ipsum",
        "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        960,
        5,
    ),
    (
        "Dolor",
        "Ut enim ad minim veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam.",
        2300,
        3,
    ),
    (
        "Sit",
        "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore.",
        710,
        4,
    ),
];

const NUM_ROWS: usize = DATA.len();

fn text(s: impl Into<StyledString>) -> Box<Text> {
    Text::new().content(s)
}

fn rtext(s: impl Into<StyledString>) -> Box<Text> {
    Text::new().content(s).right()
}

fn wrap(s: &str) -> Box<Pane> {
    Pane::new().flex(1).children([Text::new().content(s).word_wrap()])
}

/// Interactive [`Grid`] demo page.
pub struct GridDemo {
    root: Box<Pane>,
    grid_id: WidgetId<Grid>,
    border_radio_id: WidgetId<RadioGroup>,
    external_border_id: WidgetId<Checkbox>,
    row_borders_id: WidgetId<Checkbox>,
    col_borders_id: WidgetId<Checkbox>,
    header_id: WidgetId<Checkbox>,
    footer_id: WidgetId<Checkbox>,
    col_gap_id: WidgetId<Counter>,
    row_gap_id: WidgetId<Counter>,
    col_padding_id: WidgetId<Counter>,
    row_padding_id: WidgetId<Counter>,
    row_striping_id: WidgetId<Checkbox>,
    col_striping_id: WidgetId<Checkbox>,
    border_idx: usize,
    external_border: bool,
    row_borders: bool,
    col_borders: bool,
    header: bool,
    footer: bool,
    col_gap: u8,
    row_gap: u8,
    col_padding: u8,
    row_padding: u8,
    row_striping: bool,
    col_striping: bool,
    radio_selections: [usize; NUM_ROWS],
    radio_ids: [WidgetId<RadioGroup>; NUM_ROWS],
}

impl DelegateWidget for GridDemo {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        let mut rebuild = false;
        if let Some(&ChangeEvent(i)) = event.get_by::<ChangeEvent<usize>>(self.border_radio_id) {
            self.border_idx = i;
            rebuild = true;
        } else if let Some(&ChangeEvent(b)) = event.get_by::<ChangeEvent<bool>>(self.external_border_id) {
            self.external_border = b;
            rebuild = true;
        } else if let Some(&ChangeEvent(b)) = event.get_by::<ChangeEvent<bool>>(self.row_borders_id) {
            self.row_borders = b;
            rebuild = true;
        } else if let Some(&ChangeEvent(b)) = event.get_by::<ChangeEvent<bool>>(self.col_borders_id) {
            self.col_borders = b;
            rebuild = true;
        } else if let Some(&ChangeEvent(b)) = event.get_by::<ChangeEvent<bool>>(self.header_id) {
            self.header = b;
            rebuild = true;
        } else if let Some(&ChangeEvent(b)) = event.get_by::<ChangeEvent<bool>>(self.footer_id) {
            self.footer = b;
            rebuild = true;
        } else if let Some(&ChangeEvent(v)) = event.get_by::<ChangeEvent<i32>>(self.col_gap_id) {
            self.col_gap = v.max(0) as u8;
            rebuild = true;
        } else if let Some(&ChangeEvent(v)) = event.get_by::<ChangeEvent<i32>>(self.row_gap_id) {
            self.row_gap = v.max(0) as u8;
            rebuild = true;
        } else if let Some(&ChangeEvent(v)) = event.get_by::<ChangeEvent<i32>>(self.col_padding_id) {
            self.col_padding = v.max(0) as u8;
            rebuild = true;
        } else if let Some(&ChangeEvent(v)) = event.get_by::<ChangeEvent<i32>>(self.row_padding_id) {
            self.row_padding = v.max(0) as u8;
            rebuild = true;
        } else if let Some(&ChangeEvent(b)) = event.get_by::<ChangeEvent<bool>>(self.row_striping_id) {
            self.row_striping = b;
            rebuild = true;
        } else if let Some(&ChangeEvent(b)) = event.get_by::<ChangeEvent<bool>>(self.col_striping_id) {
            self.col_striping = b;
            rebuild = true;
        } else if let Some(&ChangeEvent(idx)) = event.get::<ChangeEvent<usize>>() {
            for i in 0..NUM_ROWS {
                if event.source == self.radio_ids[i] {
                    self.radio_selections[i] = idx;
                    break;
                }
            }
        }
        if rebuild {
            self.rebuild_grid();
        }
    }
}

impl GridDemo {
    fn rebuild_grid(&mut self) {
        let border = BORDERS[self.border_idx];
        let row_striping = self.row_striping;
        let col_striping = self.col_striping;
        let radio_selections = self.radio_selections;
        let mut radio_ids = [WidgetId::EMPTY; NUM_ROWS];

        let Some(grid) = self.root.get_widget_mut(self.grid_id) else {
            return;
        };

        grid.clear();
        grid.set_border(if self.external_border { border } else { None });
        grid.set_row_borders(if self.row_borders { border } else { None });
        grid.set_col_borders(if self.col_borders { border } else { None });
        grid.set_col_gap(self.col_gap);
        grid.set_row_gap(self.row_gap);
        grid.set_padding(
            Spacing::new()
                .horizontal(self.col_padding)
                .vertical(self.row_padding),
        );
        let footer_row = 1 + NUM_ROWS as u16;
        let n_rows = footer_row + 1;

        grid.set_columns(vec![
            Track::auto(),
            Track::grow(1),
            Track::auto(),
            Track::auto(),
        ]);
        grid.set_rows(vec![Track::auto(); n_rows as usize]);

        grid.add_child(0, 0, text("Item".bold()));
        grid.add_child(0, 1, text("Description".bold()));
        grid.add_child(0, 2, text("Variant".bold()));
        grid.add_child(0, 3, rtext("Count".bold()));

        let mut total: u32 = 0;
        for (i, (name, desc, count, n_variants)) in DATA.iter().enumerate() {
            let row = 1 + i as u16;
            let radio = RadioGroup::new(&VARIANTS[..*n_variants])
                .selected(radio_selections[i])
                .id(&mut radio_ids[i]);
            grid.add_child(row, 0, text(*name));
            grid.add_child(row, 1, wrap(desc));
            grid.add_child(row, 2, radio);
            grid.add_child(row, 3, rtext(format!("{}", count)));
            total += *count;
        }

        grid.add_cell(Cell::new(footer_row, 0, text("Total".bold())).span(1, 3));
        grid.add_child(footer_row, 3, rtext(format!("{}", total).bold()));

        grid.set_row_bottom(0, self.header.then_some(Border::THICK));
        grid.set_row_top(footer_row, self.footer.then_some(Border::THICK));

        let stripe = Style::new().bg(Color::grey256(2));
        for r in 0..n_rows {
            let on = row_striping && r % 2 == 1;
            grid.set_row_style(r, if on { stripe } else { Style::new() });
        }
        for c in 0..4u16 {
            let on = col_striping && c % 2 == 1;
            grid.set_col_style(c, if on { stripe } else { Style::new() });
        }

        drop(grid);
        self.radio_ids = radio_ids;
    }

    /// Creates a `GridDemo` page.
    pub fn new() -> Box<Self> {
        let mut grid_id = WidgetId::EMPTY;
        let mut border_radio_id = WidgetId::EMPTY;
        let mut external_border_id = WidgetId::EMPTY;
        let mut row_borders_id = WidgetId::EMPTY;
        let mut col_borders_id = WidgetId::EMPTY;
        let mut header_id = WidgetId::EMPTY;
        let mut footer_id = WidgetId::EMPTY;
        let mut col_gap_id = WidgetId::EMPTY;
        let mut row_gap_id = WidgetId::EMPTY;
        let mut col_padding_id = WidgetId::EMPTY;
        let mut row_padding_id = WidgetId::EMPTY;
        let mut row_striping_id = WidgetId::EMPTY;
        let mut col_striping_id = WidgetId::EMPTY;

        let section = |heading: &str, body: Box<dyn Widget>| -> Box<dyn Widget> {
            Pane::new()
                .vertical()
                .children([
                    Text::new().content(heading.bold()) as Box<dyn Widget>,
                    body,
                ]) as Box<dyn Widget>
        };

        let border_section = section(
            "Border",
            RadioGroup::new(&BORDER_LABELS)
                .selected(1)
                .id(&mut border_radio_id),
        );
        let border_toggles_section = section(
            "Show borders",
            Pane::new()
                .vertical()
                .children([
                    Checkbox::new(Text::new().content("External"))
                        .checked()
                        .id(&mut external_border_id) as Box<dyn Widget>,
                    Checkbox::new(Text::new().content("Rows"))
                        .checked()
                        .id(&mut row_borders_id),
                    Checkbox::new(Text::new().content("Cols"))
                        .checked()
                        .id(&mut col_borders_id),
                    Checkbox::new(Text::new().content("Header"))
                        .checked()
                        .id(&mut header_id),
                    Checkbox::new(Text::new().content("Footer"))
                        .id(&mut footer_id),
                ]),
        );
        let gap_section = section(
            "Gaps",
            Pane::new()
                .vertical()
                .children([
                    Counter::new("Col gap")
                        .value(0)
                        .min(0)
                        .max(4)
                        .id(&mut col_gap_id) as Box<dyn Widget>,
                    Counter::new("Row gap")
                        .value(0)
                        .min(0)
                        .max(4)
                        .id(&mut row_gap_id),
                    Counter::new("Col padding")
                        .value(1)
                        .min(0)
                        .max(8)
                        .id(&mut col_padding_id),
                    Counter::new("Row padding")
                        .value(0)
                        .min(0)
                        .max(4)
                        .id(&mut row_padding_id),
                ]),
        );
        let striping_section = section(
            "Striping",
            Pane::new()
                .vertical()
                .children([
                    Checkbox::new(Text::new().content("Rows")).id(&mut row_striping_id)
                        as Box<dyn Widget>,
                    Checkbox::new(Text::new().content("Cols")).id(&mut col_striping_id),
                ]),
        );
        let controls = Pane::new()
            .horizontal()
            .wrap()
            .gap(4)
            .child(border_section)
            .child(border_toggles_section)
            .child(gap_section)
            .child(striping_section);

        let intro = Text::new()
            .content("Grid places children at explicit (row, col) positions and aligns cells across both axes. This is useful for tables and other grid-based layouts.")
            .word_wrap();

        let root = Pane::new()
            .vertical()
            .flex(1)
            .gap(1)
            .children([
                intro as Box<dyn Widget>,
                controls,
                Grid::new().flex(1).resizable().id(&mut grid_id),
            ]);

        let mut this = Box::new(Self {
            root,
            grid_id,
            border_radio_id,
            external_border_id,
            row_borders_id,
            col_borders_id,
            header_id,
            footer_id,
            col_gap_id,
            row_gap_id,
            col_padding_id,
            row_padding_id,
            row_striping_id,
            col_striping_id,
            border_idx: 1,
            external_border: true,
            row_borders: true,
            col_borders: true,
            header: true,
            footer: false,
            col_gap: 0,
            row_gap: 0,
            col_padding: 1,
            row_padding: 0,
            row_striping: false,
            col_striping: false,
            radio_selections: [0; NUM_ROWS],
            radio_ids: [WidgetId::EMPTY; NUM_ROWS],
        });

        this.rebuild_grid();
        this
    }
}

/// Returns a [`GridDemo`] page.
pub fn grid_demo_page() -> Box<GridDemo> {
    GridDemo::new()
}
