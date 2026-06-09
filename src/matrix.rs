//! Falling-glyph matrix animation widget.

use std::cell::Cell as StdCell;
use std::time::Duration;

use tuie::prelude::*;

struct Rng(u64);

impl Rng {
    fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E3779B97F4A7C15)
            | 1;
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / ((1u32 << 24) as f32)
    }

    fn range_u16(&mut self, lo: u16, hi_inclusive: u16) -> u16 {
        if hi_inclusive <= lo {
            return lo;
        }
        let span = (hi_inclusive - lo) as u64 + 1;
        lo + (self.next_u64() % span) as u16
    }

    fn range_f32(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.next_f32() * (hi - lo)
    }

    fn pick_char(&mut self, s: &str) -> char {
        let count = s.chars().count();
        if count == 0 {
            return ' ';
        }
        let idx = (self.next_u64() as usize) % count;
        s.chars().nth(idx).unwrap_or(' ')
    }
}

struct Snake {
    x: u16,
    y: i32,
    body_length: u16,
    speed: f32,
    speed_counter: f32,
    step_count: i32,
}

#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    step_number: i32,
}

/// Falling-glyph matrix animation widget.
pub struct Matrix {
    layout: Layout,
    snakes: Vec<Snake>,
    grid: Vec<Vec<Option<Cell>>>,
    grid_size: Vec2<u16>,
    seen_size: StdCell<Vec2<u16>>,
    rng: Rng,
    task: TaskHandle,
}

impl Matrix {
    const TICK_INTERVAL: Duration = Duration::from_millis(33);
    const SNAKE_BODY_MIN_LENGTH: u16 = 0;
    const SNAKE_BODY_MAX_LENGTH: u16 = 16;
    const SNAKE_TAIL_LENGTH: u16 = 6;
    const SNAKE_SPAWN_CHANCE: f32 = 0.2;
    const FALL_SPEED_MIN: f32 = 1.0;
    const FALL_SPEED_MAX: f32 = 4.0;
    const HEAD_COLOR_INDEX: u8 = 231;
    const JAPANESE_CHARS: &'static str =
        "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワン";

    fn ensure_grid(&mut self, size: Vec2<u16>) {
        if self.grid_size == size {
            return;
        }
        self.grid_size = size;
        let w = size.x as usize;
        let h = size.y as usize;
        self.grid.resize(h, Vec::new());
        for row in &mut self.grid {
            row.resize(w, None);
        }
    }

    fn spawn_snakes(&mut self) {
        let w = self.grid_size.x;
        let h = self.grid_size.y;
        if w == 0 || h == 0 {
            return;
        }
        if self.rng.next_f32() >= Self::SNAKE_SPAWN_CHANCE {
            return;
        }
        let max_x = w.saturating_sub(2);
        let mut x = self.rng.range_u16(0, max_x);
        x &= !1;
        let new_hi = x + 2;
        let occupied = self.snakes.iter().any(|s| {
            let s_hi = s.x + 2;
            x < s_hi && s.x < new_hi
        });
        if occupied {
            return;
        }
        let body_length = self.rng.range_u16(Self::SNAKE_BODY_MIN_LENGTH, Self::SNAKE_BODY_MAX_LENGTH);
        let speed = self.rng.range_f32(Self::FALL_SPEED_MIN, Self::FALL_SPEED_MAX);
        self.snakes.push(Snake {
            x,
            y: -1,
            body_length,
            speed,
            speed_counter: 0.0,
            step_count: 0,
        });
    }

    fn advance_snakes(&mut self) {
        let w = self.grid_size.x as usize;
        let h = self.grid_size.y as i32;
        if w == 0 || h <= 0 {
            return;
        }
        for i in 0..self.snakes.len() {
            self.snakes[i].speed_counter += 1.0;
            while self.snakes[i].speed_counter >= self.snakes[i].speed {
                self.snakes[i].speed_counter -= self.snakes[i].speed;
                self.snakes[i].y += 1;
                self.snakes[i].step_count += 1;
                let y = self.snakes[i].y;
                let x = self.snakes[i].x as usize;
                let step = self.snakes[i].step_count;
                if y >= 0 && y < h && x < w {
                    let glyph = self.rng.pick_char(Self::JAPANESE_CHARS);
                    self.grid[y as usize][x] = Some(Cell { ch: glyph, step_number: step });
                    if x + 1 < w {
                        self.grid[y as usize][x + 1] = Some(Cell { ch: '\0', step_number: step });
                    }
                }
            }
        }
        let h_u16 = self.grid_size.y;
        self.snakes.retain(|s| {
            let limit = h_u16 as i32 + s.body_length as i32 + Self::SNAKE_TAIL_LENGTH as i32 + 1;
            s.y <= limit
        });
    }

    fn color_for(&self, snake: &Snake, cell_step: i32) -> Option<Color> {
        let offset = snake.step_count - cell_step;
        if offset < 0 {
            return None;
        }
        if offset == 0 {
            return Some(Color::Indexed(Self::HEAD_COLOR_INDEX));
        }
        let body_length = snake.body_length as i32;
        if offset <= body_length {
            return Some(Color::cube256(0, 5, 0));
        }
        let tail_position = offset - body_length;
        let fade_amount = std::cmp::min(5, tail_position - 1);
        if fade_amount >= 5 {
            return None;
        }
        let g = (5 - fade_amount).max(0) as u8;
        Some(Color::cube256(0, g, 0))
    }

    fn find_active_snake(&self, x: u16, cell_step: i32) -> Option<&Snake> {
        let max_offset = Self::SNAKE_BODY_MAX_LENGTH as i32 + Self::SNAKE_TAIL_LENGTH as i32;
        for s in &self.snakes {
            let owns_x = s.x == x || s.x + 1 == x;
            if !owns_x {
                continue;
            }
            let offset = s.step_count - cell_step;
            if offset < 0 {
                continue;
            }
            let limit = s.body_length as i32 + Self::SNAKE_TAIL_LENGTH as i32;
            if offset <= limit && offset <= max_offset {
                return Some(s);
            }
        }
        None
    }

    fn cull_dead_cells(&mut self) {
        let h = self.grid_size.y as usize;
        let w = self.grid_size.x as usize;
        for y in 0..h {
            for x in 0..w {
                let Some(cell) = self.grid[y][x] else {
                    continue;
                };
                if self.find_active_snake(x as u16, cell.step_number).is_none() {
                    self.grid[y][x] = None;
                }
            }
        }
    }

    fn tick(&mut self) {
        let size = self.seen_size.get();
        if size.x > 0 && size.y > 0 {
            self.ensure_grid(size);
            self.spawn_snakes();
            self.advance_snakes();
            self.cull_dead_cells();
        }
        self.dirty_paint();
        self.task = tuie::schedule(self.get_id(), Self::TICK_INTERVAL, Self::tick);
    }
}

impl Widget for Matrix {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }

    fn get_name(&self) -> &'static str {
        "Matrix"
    }

    fn measure_constraints(&mut self) -> Constraints {
        Constraints {
            min_size: Vec2::new(0, 0),
            max_size: Vec2::new(u16::MAX, u16::MAX),
            preferred_size: Vec2::new(0, 0),
        }
    }

    fn render(&self, mut ctx: RenderContext) {
        self.seen_size.set(ctx.size);
        let h = self.grid.len();
        for y in 0..h {
            let row = &self.grid[y];
            for x in 0..row.len() {
                let Some(cell) = row[x] else {
                    continue;
                };
                if cell.ch == '\0' {
                    continue;
                }
                let Some(snake) = self.find_active_snake(x as u16, cell.step_number) else {
                    continue;
                };
                let Some(color) = self.color_for(snake, cell.step_number) else {
                    continue;
                };
                ctx.move_to((x as i32, y as i32).into());
                ctx.set_style(Style::new().fg(color));
                ctx.write(&cell.ch.to_string());
            }
        }
    }
}

impl Matrix {
    /// Creates a new matrix animation widget.
    pub fn new() -> Box<Self> {
        let mut this = Box::new(Self {
            layout: Layout::new(),
            snakes: Vec::new(),
            grid: Vec::new(),
            grid_size: Vec2::of(0u16),
            seen_size: StdCell::new(Vec2::of(0u16)),
            rng: Rng::new(),
            task: TaskHandle::EMPTY,
        });
        this.task = tuie::schedule(this.get_id(), Self::TICK_INTERVAL, Self::tick);
        this
    }
}
