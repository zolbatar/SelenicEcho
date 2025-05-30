use crate::app_state::GFXState;
use crate::dialogue::logic::{DialogueNodeID, DialoguePersonID};
use crate::game::game_state::GameState;
use crate::location::locations::LocationID;
use crate::skia::Skia;
use skia_safe::paint::Style;
use skia_safe::utils::text_utils::Align;
use skia_safe::{Color, Font, Paint, Point, Rect};
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

pub struct PrinterStyle {
    pub paint: Paint,
    pub font: Font,
    pub font_bold: Font,
}

struct OnScreenWord {
    pos: Point,
    c: String,
    style: Arc<PrinterStyle>,
    is_bold: bool,
}

struct QueueItem {
    text: String,
    style: PrintStyle,
}

pub struct Printer {
    pub cursor: Point,
    queue: VecDeque<QueueItem>,
    onscreen: Vec<OnScreenWord>,
    pub(crate) padding: f32,
    v_adjust: f32,
    next_time: Instant,
    pub style: HashMap<PrintStyle, Arc<PrinterStyle>>,
    bold_mode: bool,
    ai_mode: bool,
}

const TEXT_SPEED: u64 = 1; //25;

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
                font_bold: skia.font_main_bold.clone(),
            }),
        );
        paint_white.set_color(Color::MAGENTA);
        map.insert(
            PrintStyle::Echo,
            Arc::new(PrinterStyle {
                paint: paint_white.clone(),
                font: skia.font_echo.clone(),
                font_bold: skia.font_echo.clone(),
            }),
        );
        paint_white.set_color(Color::MAGENTA);
        map.insert(
            PrintStyle::AI,
            Arc::new(PrinterStyle {
                paint: paint_white,
                font: skia.font_ai.clone(),
                font_bold: skia.font_ai_bold.clone(),
            }),
        );

        Printer {
            queue: VecDeque::new(),
            onscreen: Vec::new(),
            padding,
            v_adjust: 120.0,
            cursor: Point::new(padding, padding),
            next_time: Instant::now(),
            style: map,
            bold_mode: false,
            ai_mode: false,
        }
    }

    pub fn is_writing(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn print_location(&mut self, id: LocationID, game_state: &GameState) {
        let location = game_state.locations.get(&id).unwrap();
        let narration = game_state.narrations.get(&location.narration_id).unwrap();
        self.print(&narration.text, PrintStyle::Normal);
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

        let mut command = false;
        for c in text.chars() {
            if command {
                command = false;
                if c == '#' {
                    result.push("<NL>".to_string());
                } else if c == 'A' {
                    result.push("<ai>".to_string());
                } else if c == 'a' {
                    result.push("</ai>".to_string());
                } else if c == 'B' {
                    result.push("<bold>".to_string());
                } else if c == 'b' {
                    result.push("</bold>".to_string());
                }
            } else if c == '\n' {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                result.push("\n".to_string()); // Add newline as its own token
            } else if c == '#' {
                command = true;
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
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
    }

    fn check_for_vertical_scroll(&mut self, gfx: &GFXState) {
        if self.cursor.y > (gfx.height as f32 - self.padding) {
            self.onscreen.iter_mut().for_each(|screen| {
                screen.pos.y -= self.v_adjust;
            });

            // And remove any that are offscreen now.
            self.onscreen.retain(|screen| screen.pos.y >= self.padding);

            // Adjust cursor
            self.cursor.y -= self.v_adjust;
        }
    }

    fn single_linefeed(&mut self, style: Arc<PrinterStyle>, gfx: &GFXState) {
        self.cursor.y += style.font.size() * 1.25;
        self.check_for_vertical_scroll(gfx);
    }

    fn double_linefeed(&mut self, style: Arc<PrinterStyle>, gfx: &GFXState) {
        self.cursor.y += style.font.size() * 2.5;
        self.check_for_vertical_scroll(gfx);
    }

    pub fn print_render(&mut self, skia: &mut Skia, gfx: &GFXState, phase: f32) {
        let ai_style = self.style.get(&PrintStyle::AI).unwrap().clone();

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
                    self.double_linefeed(style, gfx);
                    self.next_time = Instant::now().add(Duration::from_millis(TEXT_SPEED * 32));
                } else if c.text == "<NL>" {
                    self.cursor.x = self.padding;
                    self.single_linefeed(style, gfx);
                    self.next_time = Instant::now().add(Duration::from_millis(TEXT_SPEED * 32));
                } else if c.text == "<bold>" {
                    self.bold_mode = true;
                } else if c.text == "</bold>" {
                    self.bold_mode = false;
                } else if c.text == "<ai>" {
                    self.ai_mode = true;
                } else if c.text == "</ai>" {
                    self.ai_mode = false;
                } else {
                    let c_with_space = c.text + " ";

                    // Size text
                    let p = style.font.measure_text(&c_with_space, Some(&style.paint));

                    // Move cursor down?
                    if (self.cursor.x + p.0) > (gfx.width as f32 - self.padding) {
                        self.cursor.x = self.padding;
                        self.single_linefeed(style.clone(), gfx);
                    }

                    let length = c_with_space.len();
                    let osw = OnScreenWord {
                        pos: self.cursor,
                        c: c_with_space,
                        style: if !self.ai_mode {
                            style.clone()
                        } else {
                            ai_style
                        },
                        is_bold: self.bold_mode,
                    };
                    self.onscreen.push(osw);

                    // Move cursor along?
                    self.cursor.x += p.0;

                    // Delay for next word
                    self.next_time = Instant::now().add(Duration::from_millis(TEXT_SPEED * length as u64));

                    // Last one
                    if self.queue.is_empty() {
                        self.cursor.x = self.padding;
                        self.double_linefeed(style.clone(), gfx);
                    }
                }
            }
        }

        // Draw all existing
        let canvas = skia.surface.canvas();
        self.onscreen.iter().for_each(|osw| {
            canvas.draw_text_align(
                osw.c.as_str(),
                osw.pos,
                if !osw.is_bold {
                    &osw.style.font
                } else {
                    &osw.style.font_bold
                },
                &osw.style.paint,
                Align::Left,
            );
        });

        // Cursor
        if phase >= 1.0 {
            let (_, fm) = skia.font_main.metrics();
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            let x = self.cursor.x;
            let y = self.cursor.y + fm.descent;
            if self.is_writing() {
                paint.set_color(Color::RED);
                paint.set_style(Style::Fill);
                let rect = Rect::from_xywh(x, y, fm.avg_char_width / 6.0, fm.ascent);
                canvas.draw_rect(rect, &paint);
            } else {
                paint.set_color(Color::GREEN);
                paint.set_style(Style::Stroke);
                paint.set_stroke_width(1.0);
                canvas.draw_line(Point::new(x, y - 2.0), Point::new(x, y + fm.ascent + 2.0), &paint);
            }
        }
    }
}
