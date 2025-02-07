use crate::printer::{PrintStyle, Printer};
use crate::skia::Skia;
use skia_safe::utils::text_utils::Align;
use skia_safe::{Color, Paint, PaintStyle, Point};

pub struct Parser {
    current_line: String,
    paint: Paint,
    line_start_text: String,
}

impl Parser {
    pub fn new() -> Self {
        let mut paint = Paint::default();
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_anti_alias(true);
        paint.set_color(Color::YELLOW);
        Parser {
            current_line: String::new(),
            paint,
            line_start_text: "# ".to_string(),
        }
    }
    
    fn get_full_text(&self) -> String {
        let t = self.line_start_text.as_str().to_owned() + self.current_line.as_str();
        t
    }

    pub fn print(&self, skia: &mut Skia, printer: &mut Printer) {
        self.calc_cursor(skia, printer);
        let canvas = skia.surface.canvas();
        canvas.draw_text_align(
            self.get_full_text(),
            Point::new(printer.padding, printer.cursor.y),
            &skia.font_main_bold,
            &self.paint,
            Align::Left,
        );
    }

    fn calc_cursor(&self, skia: &mut Skia, printer: &mut Printer) {
        let p = skia.font_main_bold.measure_text(self.get_full_text(), Some(&self.paint));
        printer.cursor.x = p.0 + printer.padding;
    }
    
    pub fn process_key(&mut self, text: String, skia: &mut Skia, printer: &mut Printer) {
        self.current_line.push_str(&text);
    }

    pub fn process_backspace(&mut self, skia: &mut Skia, printer: &mut Printer) {
        if self.current_line.is_empty() {
            return;
        }
        self.current_line.pop();
    }

    pub fn process_enter(&mut self, skia: &mut Skia, printer: &mut Printer) {}
}
