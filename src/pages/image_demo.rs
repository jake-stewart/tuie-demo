//! Image rendering demo page.

use tuie::prelude::*;

use crate::checkbox::Checkbox;

/// Image demo page.
pub(crate) struct ImageDemo {
    root: Box<Pane>,
    force_render_id: Option<WidgetId<Checkbox>>,
}

impl ImageDemo {
    fn build_tmux_pane(force_render: Box<Checkbox>) -> Box<Pane> {
        Pane::new()
            .vertical()
            .gap(1)
            .bordered()
            .border_style(Style::new().fg(Color::grey256(8)))
            .title("tmux")
            .padding(Spacing::balanced(1))
            .children([
                Text::new()
                    .content("Tuie has detected that you are using tmux.")
                    .word_wrap() as Box<dyn Widget>,
                Text::new()
                    .content(
                        "Image support cannot be auto-detected for tmux. Enable passthrough by running the following command:",
                    )
                    .word_wrap(),
                Pane::new()
                    .style(Style::new().bg(Color::grey256(2)))
                    .padding(Spacing::balanced(2))
                    .children([
                        Text::new()
                            .content("tmux set -g allow-passthrough on")
                            .style(Style::new().fg(Color::GREEN))
                    ]),
                Text::new()
                    .content(
                        "Once enabled, you can tell tuie to skip terminal detection and use the kitty graphics protocol via tmux passthrough:",
                    )
                    .word_wrap(),
                force_render,
            ])
    }
}

impl DelegateWidget for ImageDemo {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        let Some(id) = self.force_render_id else {
            return;
        };
        if let Some(ChangeEvent(checked)) = event.get_by::<ChangeEvent<bool>>(id) {
            tuie::render::image::config::update(|cfg| {
                cfg.protocol = if *checked {
                    Some(ImageProtocol::Kitty)
                } else {
                    None
                };
            });
        }
    }
}

impl ImageDemo {
    /// Creates the [`ImageDemo`] page.
    pub(crate) fn new() -> Box<Self> {
        let bytes = include_bytes!("../../../lena.png");
        let source =
            ImageSource::from_encoded(bytes.to_vec()).expect("failed to decode image header");
        let is_tmux = tuie::get_terminal_info()
            .and_then(|info| info.xtversion)
            .is_some_and(|v| v.starts_with("tmux "));

        let mut root = Pane::new()
            .vertical()
            .gap(1)
            .flex(1)
            .children([
                Text::new()
                    .content(
                        "Image rendering uses the kitty graphics protocol, with sixel and half-block fallbacks. Works over SSH and tmux via passthrough.",
                    )
                    .center()
                    .word_wrap() as Box<dyn Widget>,
                Image::new(source)
                    .height(16)
                    .max_width(80)
                    .margin_top(1)
                    .x_align(FlexAlign::Middle),
            ]);

        let force_render_id = if is_tmux {
            let mut id = WidgetId::EMPTY;
            let force_render = Checkbox::new(
                Text::new()
                    .content("This terminal supports the full kitty graphics protocol and passthrough is enabled")
                    .word_wrap()
                    .flex(1),
            )
            .id(&mut id);
            root = root.children([Self::build_tmux_pane(force_render)]);
            Some(id)
        } else {
            None
        };

        Box::new(Self { root, force_render_id })
    }
}

/// Returns the image demo page widget.
pub(crate) fn image_demo_page() -> Box<ImageDemo> {
    ImageDemo::new()
}
