//! Demo page for the harmonious 256-color palette.

use tuie::prelude::*;

use crate::link::Link;

const GIST_URL: &str =
    "https://gist.github.com/jake-stewart/0a8ea46159a7da2c808e5be2177e1783";

type Hue = (bool, bool, bool);

const DEPTH: u8 = 3;

const HUES: [Hue; 6] = [
    (true, false, false),
    (true, true, false),
    (false, true, false),
    (false, true, true),
    (false, false, true),
    (true, false, true),
];

fn hex_digit(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + n - 10) as char,
        _ => '?',
    }
}

fn palette_cell(index: u8, label: char, background: bool) -> Box<Text> {
    let swatch = Color::Base256(index);
    let dark = if (16..=231).contains(&index) {
        let idx = index - 16;
        let r = (idx / 36) % 6;
        let g = (idx / 6) % 6;
        let b = idx % 6;
        r < 4 && g < 4 && b < 4
    } else if index >= 232 {
        (index - 232) < 11
    } else {
        index % 8 == 0
    };

    let content = format!(" {} ", label);
    if background {
        let fg = if dark {
            Color::Foreground
        } else {
            Color::Background
        };
        Text::new().content(content.fg(fg).bg(swatch))
    } else {
        Text::new().content(content.fg(swatch).bg(Color::Background))
    }
}

fn grey_strip(from: i32, to: i32, vertical: bool) -> Box<Pane> {
    let step: i32 = if from < to {
        1
    } else {
        -1
    };
    let mut pane = if vertical {
        Pane::new().vertical()
    } else {
        Pane::new().horizontal()
    };
    let mut i = from;
    loop {
        let idx = if i == 24 {
            231
        } else {
            232 + i as u8
        };
        let brightness = ((i * 16 / 24) as u8).min(15);
        pane = pane.children([palette_cell(idx, hex_digit(brightness), true)]);
        if i == to {
            break;
        }
        i += step;
    }
    pane
}

fn mix_cell(
    cur: Hue,
    next: Hue,
    cur_count: u8,
    next_count: u8,
    intensity: u8,
    fallback: u8,
    background: bool,
) -> Box<Text> {
    let channel = |(r, g, b): Hue, enabled_val: u8, disabled_val: u8| -> (u8, u8, u8) {
        let pick = |on: bool| -> u8 {
            if on {
                enabled_val
            } else {
                disabled_val
            }
        };
        (pick(r), pick(g), pick(b))
    };
    let cur_rgb = channel(cur, intensity, fallback);
    let next_rgb = channel(next, intensity, fallback);
    let sum = (
        cur_rgb.0 * cur_count + next_rgb.0 * next_count,
        cur_rgb.1 * cur_count + next_rgb.1 * next_count,
        cur_rgb.2 * cur_count + next_rgb.2 * next_count,
    );
    let total = cur_count + next_count;
    let r = sum.0 / total;
    let g = sum.1 / total;
    let b = sum.2 / total;
    let index = 16 + r * 36 + g * 6 + b;
    let brightness_divisor = if fallback == 0 {
        15
    } else {
        16
    };
    let brightness = ((r as u32 + g as u32 + b as u32) * 16 / brightness_divisor) as u8;
    palette_cell(index, hex_digit(brightness.min(15)), background)
}

fn color_slices(background: bool) -> Box<Pane> {
    let mut pane = Pane::new().horizontal();
    for (i, hue) in HUES.iter().enumerate() {
        let next = HUES[(i + 1) % HUES.len()];
        for step in 0..DEPTH {
            let cur_count = DEPTH - step;
            let next_count = step;
            let mut cells: Vec<Box<Text>> = Vec::new();
            for intensity in 1..=5u8 {
                cells.push(mix_cell(*hue, next, cur_count, next_count, intensity, 0, background));
            }
            let second_end = if background {
                6
            } else {
                5
            };
            for fallback in 1..second_end {
                cells.push(mix_cell(*hue, next, cur_count, next_count, 5, fallback, background));
            }
            if background {
                cells.reverse();
            }
            let mut slice = Pane::new().vertical();
            for cell in cells {
                slice = slice.children([cell]);
            }
            pane = pane.children([slice]);
        }
    }
    pane
}

fn palette_preview() -> Box<Pane> {
    Pane::new()
        .vertical()
        .children([
            Pane::new()
                .horizontal()
                .children([
                    grey_strip(24, 6, true),
                    Pane::new()
                        .vertical()
                        .children([
                            color_slices(true),
                            color_slices(false),
                        ]),
                    grey_strip(24, 6, true),
                ]),
            Pane::new()
                .horizontal()
                .children([
                    grey_strip(5, 0, false) as Box<dyn Widget>,
                    Text::new().content(format!("{:^24}", "Harmonious")),
                    grey_strip(0, 5, false),
                ]),
        ])
        .vertical_padding(1)
        .x_place(Place::Middle)
        .x_scroll(Scrollbar::AutoHide)
}

/// Returns the harmonious demo page.
pub fn harmonious_demo_page() -> Box<dyn Widget> {
    let prose = Text::new()
        .content(
            "The 256-color palette is harmonized with the terminal's theme. \
             Colors render consistently across terminals without configuration."
                .fg(Color::Foreground),
        )
        .word_wrap()
        .center();

    Pane::new()
        .vertical()
        .gap(1)
        .flex(1)
        .children([
            palette_preview() as Box<dyn Widget>,
            prose,
            Link::new("Read more", GIST_URL).x_align(FlexAlign::Middle),
        ])
}
