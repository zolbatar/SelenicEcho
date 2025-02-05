use crate::app_state::GFXState;
use crate::skia::Skia;
use skia_safe::paint::Style;
use skia_safe::utils::text_utils::Align;
use skia_safe::{Color, Font, Paint, Point};
use std::collections::{HashMap, VecDeque};
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Eq, PartialEq, Hash)]
pub enum PrintStyle {
    NORMAL,
}

struct PrinterStyle {
    paint: Paint,
    font: Font,
}

struct OnScreenWord {
    pos: Point,
    c: String,
    style: Arc<PrinterStyle>,
}

pub struct Printer {
    cursor: Point,
    queue: VecDeque<String>,
    onscreen: Vec<OnScreenWord>,
    padding: f32,
    next_time: Instant,
    style: HashMap<PrintStyle, Arc<PrinterStyle>>,
}

const TEXT_SPEED: u64 = 10;

impl Printer {
    pub fn new(skia: &Skia) -> Printer {
        let mut paint_white = Paint::default();
        paint_white.set_anti_alias(true);
        paint_white.set_style(Style::StrokeAndFill);
        paint_white.set_color(Color::WHITE);
        let padding = 80.0f32;

        // Styles
        let mut map = HashMap::new();
        map.insert(
            PrintStyle::NORMAL,
            Arc::new(PrinterStyle {
                paint: paint_white,
                font: skia.font.clone(),
            }),
        );

        Printer {
            queue: VecDeque::new(),
            onscreen: Vec::new(),
            padding,
            cursor: Point::new(padding, padding),
            next_time: Instant::now(),
            style: map,
        }
    }

    pub fn print(&mut self, text: String) {
        for c in text.split_whitespace() {
            self.queue.push_back(c.trim().to_string());
        }
    }

    pub fn print_render(&mut self, skia: &mut Skia, gfx: &GFXState, style: PrintStyle) {
        // Too early?
        let diff = Instant::now().duration_since(self.next_time).as_millis();
        if diff > 0 {
            // Move new one?
            let c = self.queue.pop_front();
            if let Some(c) = c {
                let c_with_space = c + " ";

                // Get style
                let style = self.style.get(&style).unwrap().clone();

                // Size text
                let p = style.font.measure_text(&c_with_space, Some(&style.paint));

                // Move cursor down?
                if (self.cursor.x + p.0) > (gfx.width as f32 - self.padding) {
                    self.cursor.x = self.padding;
                    self.cursor.y += style.font.size() * 1.25;
                }

                let length = c_with_space.len();
                let osw = OnScreenWord {
                    pos: self.cursor,
                    c: c_with_space,
                    style,
                };
                self.onscreen.push(osw);

                // Move cursor along?
                self.cursor.x += p.0;

                // Delay for next word
                self.next_time = Instant::now().add(Duration::from_millis(TEXT_SPEED * length as u64));
            }
        }

        // Draw all existing
        let canvas = skia.surface.canvas();
        self.onscreen.iter().for_each(|osw| {
            canvas.draw_text_align(osw.c.as_str(), osw.pos, &osw.style.font, &osw.style.paint, Align::Left);
        });
    }
}
