//! Demo page for async, timer, and animation features.

use std::time::Duration;

use tuie::prelude::*;

use crate::accordion::Accordion;
use crate::button::Button;
use crate::matrix::Matrix;
use crate::progress_bar::ProgressBar;
use crate::title::Title;

const TICK: Duration = Duration::from_secs(1);

struct TimersPanel {
    root: Box<Pane>,
    title_id: WidgetId<Title>,
    counter: u32,
    task: TaskHandle,
}

impl TimersPanel {
    fn new() -> Box<Self> {
        let mut title_id = WidgetId::EMPTY;
        let root = Pane::new()
            .horizontal()
            .y_place(Place::Center)
            .gap(2)
            .children([
                Text::new()
                    .content("The counter ticks once every second. Each tick wakes the widget and updates the label.")
                    .word_wrap()
                    .center()
                    .margin(Spacing::balanced(3))
                    .flex(1) as Box<dyn Widget>,
                Pane::new()
                    .horizontal()
                    .y_place(Place::Center)
                    .children([
                        Pane::new()
                            .bordered()
                            .children([
                                Title::new("0")
                                    .compact()
                                    .margin(Spacing::new().horizontal(3))
                                    .id(&mut title_id),
                            ]),
                    ]),
            ]);
        let mut this = Box::new(Self {
            root,
            title_id,
            counter: 0,
            task: TaskHandle::EMPTY,
        });
        this.task = tuie::schedule(this.get_id(), TICK, Self::tick);
        this
    }

    fn tick(&mut self) {
        self.counter = (self.counter + 1) % 10;
        if let Some(t) = self.root.get_widget_mut(self.title_id) {
            t.set_text(&self.counter.to_string());
        }
        self.task = tuie::schedule(self.get_id(), TICK, Self::tick);
    }
}

impl DelegateWidget for TimersPanel {
    tuie::delegate_widget!(root);
}

fn animations_panel() -> Box<Pane> {
    Pane::new()
        .vertical()
        .height(16)
        .children([
            Text::new()
                .content("Falling glyphs inspired by cmatrix. The widget runs its own animation, no external driver needed.")
                .word_wrap()
                .center()
                .margin(Spacing::balanced(3)),
            Pane::new()
                .bordered()
                .flex(1)
                .children([Matrix::new().flex(1)]),
        ])
}

struct AsyncPanel {
    root: Box<Pane>,
    button_id: WidgetId<Button>,
    bar_id: WidgetId<ProgressBar>,
    task: TaskHandle,
}

impl AsyncPanel {
    fn new() -> Box<Self> {
        let mut button_id = WidgetId::EMPTY;
        let mut bar_id = WidgetId::EMPTY;
        let root = Pane::new()
            .horizontal()
            .y_place(Place::Center)
            .gap(2)
            .children([
                Text::new()
                    .content("Click Load to start a background task. As it runs, it streams progress back to update the bar.")
                    .word_wrap()
                    .center()
                    .margin(Spacing::balanced(3))
                    .flex(1) as Box<dyn Widget>,
                Pane::new()
                    .vertical()
                    .flex(1)
                    .children([
                        Button::new()
                            .children([Text::new().content("Load").margin(Spacing::balanced(1))])
                            .id(&mut button_id) as Box<dyn Widget>,
                        Pane::new()
                            .bordered()
                            .children([ProgressBar::new().margin(Spacing::balanced(1)).id(&mut bar_id)]),
                    ]),
            ]);
        Box::new(Self {
            root,
            button_id,
            bar_id,
            task: TaskHandle::EMPTY,
        })
    }
}

impl DelegateWidget for AsyncPanel {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if !event.of_by::<ClickEvent>(self.button_id) {
            return;
        }
        self.task.cancel();
        if let Some(bar) = self.root.get_widget_mut(self.bar_id) {
            bar.set_progress(0.0);
        }
        let bar_id = self.bar_id;
        self.task = tuie::spawn_stream(
            self.get_id(),
            move |send: Box<dyn Fn(f32) + Send>| async move {
                for i in 1..=100u32 {
                    std::thread::sleep(Duration::from_millis(30));
                    send(i as f32 / 100.0);
                }
            },
            move |me: &mut AsyncPanel, value| {
                if let Some(v) = value {
                    if let Some(bar) = me.root.get_widget_mut(bar_id) {
                        bar.set_progress(v);
                    }
                }
            },
        );
    }
}

struct AsyncDemo {
    root: Box<Pane>,
}

impl DelegateWidget for AsyncDemo {
    tuie::delegate_widget!(root);
}

/// Returns the async demo page.
pub fn async_demo_page() -> Box<dyn Widget> {
    let root = Pane::new()
        .vertical()
        .gap(1)
        .flex(1)
        .children([
            Accordion::new("Timers", TimersPanel::new()),
            Accordion::new("Animations", animations_panel()),
            Accordion::new("Async", AsyncPanel::new()),
        ]);
    Box::new(AsyncDemo { root })
}
