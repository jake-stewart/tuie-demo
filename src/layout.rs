//! Demo shell layout and page-stack scaffolding.

use tuie::prelude::*;

use crate::button::Button;
use crate::page_layout::PageLayout;

/// Wraps `content` in a scrollable, padded pane.
pub fn wrap_scrollable(content: Box<dyn Widget>) -> Box<Pane> {
    let inner = Pane::new()
        .vertical()
        .children([content])
        .preferred_width(80)
        .flex(1);
    Pane::new()
        .vertical()
        .children([inner])
        .y_scroll(Scrollbar::AutoHide)
        .x_scroll(Scrollbar::AutoHide)
        .insets(Spacing::new().top(3))
        .padding(Spacing::balanced(2).top(4))
        .x_place(Place::Middle)
        .flex(1)
}

/// Demo shell with a title header and page stack.
pub struct DemoLayout {
    root: Box<Stack>,
    page_layout_id: WidgetId<PageLayout>,
    left_column_id: WidgetId<Pane>,
    title_text_id: WidgetId<Text>,
    titles: Vec<String>,
    back_button_id: Option<WidgetId<Button>>,
}

impl DemoLayout {
    fn pop(&mut self) {
        if self.titles.len() <= 1 {
            return;
        }
        self.titles.pop();
        let title = self.titles.last().unwrap().clone();
        let at_root = self.titles.len() <= 1;

        if let Some(text) = self.root.get_widget_mut(self.title_text_id) {
            text.set_content(title.bold());
        }

        if at_root {
            if let Some(left_column) = self.root.get_widget_mut(self.left_column_id) {
                left_column.clear();
            }
            self.back_button_id = None;
        }

        if let Some(page_layout) = self.root.get_widget_mut(self.page_layout_id) {
            page_layout.pop();
        }
    }
}

impl DelegateWidget for DemoLayout {
    tuie::delegate_widget!(root);

    fn after_on_event(&mut self, event: &mut WidgetEvent) {
        if let Some(back_id) = self.back_button_id {
            if event.of_by::<ClickEvent>(back_id.untyped()) {
                self.pop();
            }
        }
    }
}

impl DemoLayout {
    /// Creates the shell with `title` in the header and `home` as the root page.
    pub fn new(title: &str, home: Box<dyn Widget>) -> Box<Self> {
        let mut title_text_id = WidgetId::EMPTY;
        let mut left_column_id = WidgetId::EMPTY;
        let mut page_layout_id = WidgetId::EMPTY;

        let header = Pane::new()
            .flex(1)
            .height(3)
            .horizontal()
            .x_place(Place::Middle)
            .style(Style::new().bg(Color::grey256(5)).blend(95))
            .children([
                Pane::new()
                    .horizontal()
                    .max_width(84)
                    .flex(1)
                    .padding(Spacing::new().horizontal(2))
                    .children([
                        Pane::new().flex(1).horizontal().id(&mut left_column_id),
                        Pane::new()
                            .vertical()
                            .flex(1)
                            .y_place(Place::Middle)
                            .children([
                                Text::new()
                                    .content(title.bold())
                                    .center()
                                    .id(&mut title_text_id),
                            ]),
                        Pane::new().flex(1),
                    ]),
            ]);

        let root = Stack::new(PageLayout::new(wrap_scrollable(home)).id(&mut page_layout_id))
            .flex(1)
            .min_height(0)
            .children([TopLayer::new(header)]);

        Box::new(Self {
            root,
            page_layout_id,
            left_column_id,
            title_text_id,
            titles: vec![title.to_string()],
            back_button_id: None,
        })
    }

    /// Pushes `content` onto the page stack with `title` in the header.
    pub fn push(&mut self, title: &str, content: Box<dyn Widget>) {
        self.titles.push(title.to_string());

        if let Some(text) = self.root.get_widget_mut(self.title_text_id) {
            text.set_content(title.bold());
        }

        let mut back_id = WidgetId::EMPTY;
        let back_button = Button::new()
            .children([Text::new().content(" Back ")])
            .id(&mut back_id);
        self.back_button_id = Some(back_id);
        if let Some(left_column) = self.root.get_widget_mut(self.left_column_id) {
            left_column.clear();
            left_column.add_child(back_button);
        }

        if let Some(page_layout) = self.root.get_widget_mut(self.page_layout_id) {
            page_layout.push(content);
        }

        tuie::focus_widget(back_id);
    }
}

struct TopLayer {
    inner: Box<dyn Widget>,
}

impl TopLayer {
    fn new(inner: Box<dyn Widget>) -> Box<Self> {
        Box::new(Self { inner })
    }
}

impl DelegateWidget for TopLayer {
    tuie::delegate_widget!(inner);

    fn override_get_layer(&self) -> Layer {
        Layer::Top
    }
}
