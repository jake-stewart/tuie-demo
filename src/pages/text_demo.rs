//! Text widget demo page.

use tuie::prelude::*;

const SAMPLE: &str = "\
你 lorem ipsum dolor sit amet, 好 consectetur adipiscing elit, \
世 sed do eiusmod tempor incididunt, 界 ut labore et dolore magna aliqua.";

fn overflow_example(
    label: &'static str,
    desc: &'static str,
    overflow: &'static TextOverflow,
) -> Box<Pane> {
    Pane::new()
        .vertical()
        .children([
            Text::new()
                .content(
                    StyledString::new()
                        .span(label.fg(Color::YELLOW).bold())
                        .span("  ")
                        .span(desc.dim()),
                )
                .overflow(overflow),
            Text::new().content(SAMPLE).overflow(overflow),
        ])
}

fn scroll_example() -> Box<Pane> {
    Pane::new()
        .vertical()
        .children([
            Text::new()
                .content(
                    StyledString::new()
                        .span("Scroll".fg(Color::YELLOW).bold())
                        .span("  ")
                        .span("horizontally scrollable, no wrapping".dim()),
                )
                .overflow(TextOverflow::VISIBLE),
            Text::new()
                .content(SAMPLE)
                .overflow(TextOverflow::VISIBLE),
        ])
        .x_scroll(Scrollbar::Visible)
}

fn fade_text() -> Box<Text> {
    let content = "...Text can fade in and Text can fade out...";
    let count = content.chars().count();
    let mut text = Text::new();
    for (i, (byte, ch)) in content.char_indices().enumerate() {
        let progress = i as f32 / (count - 1) as f32;
        let triangle = 1.0 - (2.0 * progress - 1.0).abs();
        let shade = (1.0 + triangle * 22.0).round() as u8;
        let end = byte + ch.len_utf8();
        text.push(StyledStr::new(&content[byte..end]).fg(Color::grey256(shade)));
    }
    text.center().truncate()
}

fn alignment_and_style_pane() -> Box<Pane> {
    Pane::new()
        .vertical()
        .gap(1)
        .flex(1)
        .children([
            Text::new().content("Alignment".fg(Color::BLUE).bold()).center() as Box<dyn Widget>,
            Pane::new()
                .vertical()
                .bordered()
                .border_style(Style::new().fg(Color::grey256(8)))
                .padding(Spacing::balanced(1))
                .children([
                    Text::new().content("left".fg(Color::CYAN)).left(),
                    Text::new().content("center".fg(Color::YELLOW)).center(),
                    Text::new().content("right".fg(Color::GREEN)).right(),
                ]),
            Text::new().content("Style".fg(Color::BLUE).bold()).center().margin_top(1),
            Text::new()
                .content(
                    StyledString::new()
                        .span("You can make text ")
                        .span("bold".bold())
                        .span(", ")
                        .span("italic".italic())
                        .span(", ")
                        .span("dim".dim())
                        .span(", ")
                        .span("underlined".underline(UnderlineType::Single))
                        .span(" or ")
                        .span("strikethrough".strikethrough())
                        .span(", with different ")
                        .span("foreground".fg(Color::CYAN))
                        .span(" and ")
                        .span("background".bg(Color::grey256(6)))
                        .span(" colors, all of which can be ")
                        .span("combined".bold().italic().fg(Color::YELLOW).underline(UnderlineType::Curly))
                        .span("."),
                )
                .word_wrap()
                .center(),
            Text::new()
                .content(
                    StyledString::new()
                        .span(" red ".fg(Color::RED).reverse())
                        .span(" ")
                        .span(" grn ".fg(Color::GREEN).reverse())
                        .span(" ")
                        .span(" blu ".fg(Color::BLUE).reverse())
                        .span(" ")
                        .span(" yel ".fg(Color::YELLOW).reverse())
                        .span(" ")
                        .span(" cyn ".fg(Color::CYAN).reverse())
                        .span(" ")
                        .span(" mag ".fg(Color::MAGENTA).reverse()),
                )
                .truncate()
                .center(),
            Text::new().content("Opacity".fg(Color::BLUE).bold()).center().margin_top(1),
            fade_text(),
        ])
}

fn overflow_pane() -> Box<Pane> {
    Pane::new()
        .vertical()
        .gap(1)
        .flex(1)
        .children([
            Text::new().content("Overflow".fg(Color::BLUE).bold()).center() as Box<dyn Widget>,
            scroll_example(),
            overflow_example("Word Wrap", "breaks at word boundaries", TextOverflow::WORD_WRAP),
            overflow_example("Wrap", "breaks anywhere", TextOverflow::WRAP),
            overflow_example("Ellipsis", "truncates with a marker", TextOverflow::ELLIPSIS),
            overflow_example("Truncate", "clips silently", TextOverflow::TRUNCATE),
        ])
}

fn intro_text() -> Box<Pane> {
    Pane::new()
        .vertical()
        .children([
            Text::new()
                .content("Text renders styled content with alignment and overflow controls. Unicode graphemes and double-width characters render correctly on dumb terminals.")
                .word_wrap()
                .center(),
        ])
}

/// Returns the text demo page.
pub fn text_demo_page() -> Box<Pane> {
    Pane::new()
        .vertical()
        .flex(1)
        .gap(1)
        .children([
            intro_text() as Box<dyn Widget>,
            Text::new()
                .content("drag divider to resize".dim())
                .center(),
            Split::new(
                SplitPane::new().horizontal()
                    .children([
                        SplitPaneChild::from(alignment_and_style_pane().preferred_width(10)),
                        SplitPaneChild::from(overflow_pane().preferred_width(10)),
                    ]),
            ).bordered(),
        ])
}
