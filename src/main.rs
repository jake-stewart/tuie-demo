//! tuie demo entry point.

use tuie::prelude::*;

mod accordion;
mod button;
mod checkbox;
mod counter;
mod flat_button;
mod focus_pane;
mod global_chords;
mod layout;
mod link;
mod matrix;
mod page_layout;
mod pages;
mod point_picker;
mod progress_bar;
mod radio_group;
mod segmented_control;
mod theme;
mod title;

use button::Button;
use layout::{DemoLayout, wrap_scrollable};
use title::Title;
use pages::async_demo::async_demo_page;
use pages::grid_demo::grid_demo_page;
use pages::harmonious_demo::harmonious_demo_page;
use pages::image_demo::image_demo_page;
use pages::input_demo::InputDemo;
use pages::layout_demo::layout_demo_page;
use pages::list_demo::list_demo_page;
use pages::popup_demo::popup_demo_page;
use pages::split_demo::split_demo_page;
use pages::text_demo::text_demo_page;
use pages::theme_demo::theme_demo_page;

type PageBuilder = fn() -> Box<dyn Widget>;

struct DemoPage {
    label: &'static str,
    build: PageBuilder,
    button_id: WidgetId<Button>,
}

struct DemoApp {
    layout: Box<DemoLayout>,
    pages: Vec<DemoPage>,
}

impl DemoApp {
    fn new() -> Box<Self> {
        let entries: [(&'static str, PageBuilder); 11] = [
            ("Text", || text_demo_page()),
            ("Grid", || grid_demo_page()),
            ("Input", || InputDemo::new()),
            ("Harmonious", || harmonious_demo_page()),
            ("Layout", || layout_demo_page()),
            ("Split", || split_demo_page()),
            ("List", || list_demo_page()),
            ("Popup", || popup_demo_page()),
            ("Images", || image_demo_page()),
            ("Theming", || theme_demo_page()),
            ("Async", || async_demo_page()),
        ];

        let mut buttons = Pane::new().vertical().gap(1);
        let mut pages = Vec::with_capacity(entries.len());
        for (label, build) in entries {
            let mut button_id = WidgetId::EMPTY;
            buttons.add_child(
                Button::new()
                    .children([Text::new().content(format!(" {label} "))])
                    .id(&mut button_id),
            );
            pages.push(DemoPage { label, build, button_id });
        }

        let home = Pane::new()
            .vertical()
            .gap(1)
            .flex(1)
            .padding(Spacing::balanced(1))
            .children([
                Pane::new().x_align(FlexAlign::Center).children([Title::new("Tuie")]) as Box<dyn Widget>,
                buttons,
            ]);

        let layout = DemoLayout::new("tuie demo", home);

        Box::new(Self { layout, pages })
    }
}

impl DelegateWidget for DemoApp {
    tuie::delegate_widget!(layout);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if !event.of::<ClickEvent>() {
            return;
        }
        for page in &self.pages {
            if event.source == page.button_id {
                let built = (page.build)();
                let wrapped = wrap_scrollable(built);
                self.layout.push(page.label, wrapped);
                return;
            }
        }
    }
}

fn main() -> std::io::Result<std::process::ExitCode> {
    tuie::config::update(|cfg| cfg.hover_events = false);

    #[cfg(feature = "gui")]
    tuie::gui::config::update(|cfg| {
        cfg.font_family = Some(String::from("Iosevka Custom"));
        cfg.title_bar = tuie::gui::TitleBar::Padding;
        cfg.extend_sides = true;
        cfg.extend_header = true;
    });

    tuie::set_spawner(|fut| {
        std::thread::spawn(move || {
            futures::executor::block_on(fut);
        });
    });

    let app = DemoApp::new();

    let root = Pane::new().children([app]);
    let root = global_chords::GlobalChords::new(root);
    #[cfg(feature = "gui")]
    return tuie::start_gui(root);
    #[cfg(not(feature = "gui"))]
    tuie::start_tui(root)
}
