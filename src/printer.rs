use crate::app_state::GFXState;
use crate::dialogue::logic::{DialogueNodeID, DialoguePersonID};
use crate::game::game_state::GameState;
use crate::location::locations::LocationID;
use crate::skia::Skia;
use skia_safe::paint::Style;
use skia_safe::utils::text_utils::Align;
use skia_safe::{Color, Font, Paint, Point};
use std::collections::{HashMap, VecDeque};
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum PrintStyle {
    Normal,
    AI,
    Echo,
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

struct QueueItem {
    text: String,
    style: PrintStyle,
}

pub struct Printer {
    cursor: Point,
    queue: VecDeque<QueueItem>,
    onscreen: Vec<OnScreenWord>,
    padding: f32,
    next_time: Instant,
    style: HashMap<PrintStyle, Arc<PrinterStyle>>,
}

const TEXT_SPEED: u64 = 25;

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
            PrintStyle::Normal,
            Arc::new(PrinterStyle {
                paint: paint_white.clone(),
                font: skia.font_main.clone(),
            }),
        );
        paint_white.set_color(Color::MAGENTA);
        map.insert(
            PrintStyle::Echo,
            Arc::new(PrinterStyle {
                paint: paint_white.clone(),
                font: skia.font_echo.clone(),
            }),
        );
        paint_white.set_color(Color::YELLOW);
        map.insert(
            PrintStyle::AI,
            Arc::new(PrinterStyle {
                paint: paint_white,
                font: skia.font_ai.clone(),
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

    pub fn print_location(&mut self, id: LocationID, game_state: &GameState) {
        let location = game_state.locations.get(&id).unwrap();
        self.print(&location.text, PrintStyle::Normal);
    }

    pub fn print_dialogue(&mut self, id: DialogueNodeID, game_state: &GameState) {
        let dialogue = game_state.dialogues.get(&id).unwrap();
        let style = match dialogue.speaker {
            DialoguePersonID::Player => PrintStyle::Normal,
            DialoguePersonID::Central => PrintStyle::AI,
            DialoguePersonID::Fixer => PrintStyle::AI,
            DialoguePersonID::Watcher => PrintStyle::AI,
            DialoguePersonID::Echo => PrintStyle::Echo,
        };
        self.print(&dialogue.text, style)
    }

    fn split_keep_newlines(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();

        for c in text.chars() {
            if c == '\n' {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                result.push("\n".to_string()); // Add newline as its own token
            } else if c.is_whitespace() {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }

        if !current.is_empty() {
            result.push(current);
        }

        result
    }
    
    pub fn print(&mut self, text: &str, style: PrintStyle) {
        let words = self.split_keep_newlines(text);
        for c in words.iter() {
            self.queue.push_back(QueueItem {
                text: c.clone(),
                style,
            });
        }
        self.queue.push_back(QueueItem {
            text: "\n".to_string(),
            style,
        });
    }

    pub fn print_render(&mut self, skia: &mut Skia, gfx: &GFXState) {
        // Too early?
        let diff = Instant::now().duration_since(self.next_time).as_millis();
        if diff > 0 {
            // Move new one?
            let c = self.queue.pop_front();
            if let Some(c) = c {
                // Get style
                let style = self.style.get(&c.style).unwrap().clone();

                if c.text == "\n" {
                    self.cursor.x = self.padding;
                    self.cursor.y += style.font.size() * 2.5;

                    // Delay for next word
                    self.next_time = Instant::now().add(Duration::from_millis(TEXT_SPEED * 32));
                } else {
                    let c_with_space = c.text + " ";

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
        }

        // Draw all existing
        let canvas = skia.surface.canvas();
        self.onscreen.iter().for_each(|osw| {
            canvas.draw_text_align(osw.c.as_str(), osw.pos, &osw.style.font, &osw.style.paint, Align::Left);
        });
    }
}
