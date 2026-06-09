//! Virtualized list demo page.

use tuie::prelude::*;

const IPS: &[&str] = &[
    "10.0.0.42", "10.0.0.17", "10.0.0.88", "192.168.1.5", "192.168.1.23",
    "172.16.4.100", "172.16.4.201", "203.0.113.5", "203.0.113.77",
    "198.51.100.12", "198.51.100.34", "10.0.1.15", "10.0.1.200",
    "192.168.0.7", "192.168.0.132", "10.10.10.10", "10.10.10.200",
    "172.20.0.1", "172.20.0.99", "127.0.0.1",
];

const METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "PATCH"];

const PATHS: &[&str] = &[
    "/api/users", "/api/users/4821", "/api/users/4821/posts",
    "/api/login", "/api/logout", "/api/health", "/api/metrics",
    "/api/sessions", "/api/orders/1293", "/api/orders/1293/items",
    "/api/products", "/api/products/search", "/api/cart",
    "/api/checkout", "/api/notifications", "/api/feed",
    "/api/messages/42", "/api/auth/refresh", "/api/upload",
    "/api/webhooks/github", "/api/analytics/events",
    "/api/reports/daily", "/api/v2/graph", "/api/v2/subscribe",
];

fn hash64(x: u64) -> u64 {
    let mut x = x.wrapping_mul(0x9E3779B97F4A7C15);
    x ^= x >> 32;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 32;
    x = x.wrapping_mul(0x94D049BB133111EB);
    x ^= x >> 32;
    x
}

fn number_cell(index: usize) -> Box<dyn Widget> {
    let num = format!("{}", index);
    let bg = if index % 10 == 0 {
        Color::grey256(4)
    } else {
        Color::grey256(2)
    };
    let styled = if index % 100 == 0 {
        num.fg(Color::YELLOW).bold().bg(bg)
    } else if index % 10 == 0 {
        num.fg(Color::Foreground).bold().bg(bg)
    } else {
        num.fg(Color::grey256(12)).bg(bg)
    };

    Pane::new()
        .width(10)
        .style(Style::new().bg(bg))
        .y_place(Place::Center)
        .children([Text::new().content(styled).center()])
}

fn session_widget(index: usize) -> Box<dyn Widget> {
    let session_hash = hash64(index as u64 + 1);
    let ip = IPS[(session_hash as usize) % IPS.len()];
    let action_count = ((session_hash >> 8) % 6) as usize;

    let seconds_of_day = index as u64 * 5 + (session_hash % 11);
    let hours = (seconds_of_day / 3600) % 24;
    let minutes = (seconds_of_day / 60) % 60;
    let seconds = seconds_of_day % 60;

    let duration_ms = 80 + ((session_hash >> 24) % 4800) as u64;

    let timestamp = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    let ip_column = format!("{:<16}", ip);
    let session_id = format!("#{:04x}", (session_hash >> 16) as u16);
    let duration = if duration_ms >= 1000 {
        format!("{}.{}s", duration_ms / 1000, (duration_ms % 1000) / 100)
    } else {
        format!("{}ms", duration_ms)
    };
    let duration_column = format!("  {}", duration);

    let header = Pane::new()
        .horizontal()
        .margin(Spacing::balanced(1))
        .children([
            Text::new().content(
                StyledString::new()
                    .span(timestamp.fg(Color::grey256(9)))
                    .span("   ")
                    .span("connect   ".fg(Color::GREEN))
                    .span(ip_column.as_str()),
            ) as Box<dyn Widget>,
            Pane::new().flex(1),
            Text::new().content(session_id.fg(Color::YELLOW)),
        ]);

    let mut pane = Pane::new().vertical().children([header]);

    for i in 0..action_count {
        let action_hash = hash64((index as u64).wrapping_mul(997).wrapping_add(i as u64 + 1));
        let method = METHODS[(action_hash as usize) % METHODS.len()];
        let path = PATHS[((action_hash >> 16) as usize) % PATHS.len()];
        let method_column = format!("{:<7}", method);

        pane = pane.children([
            Text::new().content(
                StyledString::new()
                    .span("            ")
                    .span(method_column.fg(Color::YELLOW).bold())
                    .span(path),
            ),
        ]);
    }

    pane.children([
        Text::new().content(
            StyledString::new()
                .span("            ")
                .span("disconnect".fg(Color::grey256(8)))
                .span(duration_column.fg(Color::grey256(9))),
        ),
    ])
}

fn number_scroll() -> Box<List> {
    let mut list = List::new()
        .horizontal()
        .min_height(5)
        .y_place(Place::Center)
        .scroll(Scrollbar::Visible)
        .bordered()
        .padding_top(1)
        .border_style(Style::new().fg(Color::grey256(8)));
    list.set_renderer((), |_: &mut (), idx: usize| -> Option<Box<dyn Widget>> {
        Some(number_cell(idx))
    });
    list.set_item_count(100_000_000);
    list
}

fn log_list() -> Box<List> {
    let mut list = List::new()
        .vertical()
        .flex(1)
        .min_height(5)
        .gap(1)
        .gap_border(&Border::DASHED)
        .gap_border_style(Style::new().fg(Color::grey256(6)))
        .scroll(Scrollbar::AutoHide)
        .bordered()
        .border_style(Style::new().fg(Color::grey256(8)));
    list.set_renderer((), |_: &mut (), idx: usize| -> Option<Box<dyn Widget>> {
        Some(session_widget(idx))
    });
    list.set_item_count(1000);
    list
}

/// Returns the list demo page.
pub fn list_demo_page() -> Box<Pane> {
    let prose = Pane::new()
        .max_width(68)
        .x_align(FlexAlign::Center)
        .children([
            Text::new()
                .content("List is a virtualised scroll container that only builds the visible window of items.")
                .center()
                .word_wrap(),
        ]);

    Pane::new()
        .vertical()
        .flex(1)
        .gap(2)
        .children([prose as Box<dyn Widget>, number_scroll(), log_list()])
}
