use crate::app_state::{AppState, GFXState};
use rand::Rng;
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::{ContextOptions, DirectContext};
use skia_safe::image_filters::drop_shadow_only;
use skia_safe::runtime_effect::RuntimeShaderBuilder;
use skia_safe::{
    gpu, Canvas, Color, Color4f, Data, Font, FontMgr, ImageFilter, Matrix, Paint, PaintStyle, Point, Rect,
    RuntimeEffect, Shader, Surface, Vector,
};

static AI_FONT: &[u8] = include_bytes!("../assets/TX-02-Medium.ttf");
static ECHO_FONT: &[u8] = include_bytes!("../assets/Marcellus-Regular.ttf");
static MAIN_FONT: &[u8] = include_bytes!("../assets/NotoSans-VariableFont_wdth,wght.ttf");
const NOISE_SKSL: &str = include_str!("../assets/noise.sksl");
const PLASMA_SKSL: &str = include_str!("../assets/plasma.sksl");
pub const _NOISE_MIX: f32 = 0.075;

pub struct Skia {
    context: DirectContext,
    pub _drop_shadow: Option<ImageFilter>,
    pub _drop_shadow_white: Option<ImageFilter>,
    noise_shader: RuntimeEffect,
    plasma_shader: RuntimeEffect,
    pub surface: Surface,
    pub colour_background: Color,
    pub font_main: Font,
    pub font_ai: Font,
    pub font_echo: Font,
}

pub const FONT_SIZE: f32 = 20.0;

impl Skia {
    fn make_surface(context: &mut DirectContext, width: i32, height: i32) -> Surface {
        // Get window size and create a Skia surface from the OpenGL framebuffer
        let fb_info = FramebufferInfo {
            fboid: 0,
            format: gl::RGBA8,
            ..Default::default()
        };
        let backend_render_target = gpu::backend_render_targets::make_gl(
            (width, height),
            0, // Sample count
            8, // Stencil bits
            fb_info,
        );

        // Create the Skia surface for rendering
        wrap_backend_render_target(
            context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Could not create Skia surface")
    }

    pub fn new(app_state: &AppState) -> Self {
        let interface = Interface::new_native().expect("Can't get GL interface");
        let options = ContextOptions::new();
        let mut context = make_gl(&interface, &options).expect("Can't create Skia context");

        // Fonts
        let font_mgr = FontMgr::new();

        // Shaders
        let noise_shader = RuntimeEffect::make_for_shader(NOISE_SKSL, None).expect("Failed to make runtime effect");
        let plasma_shader = RuntimeEffect::make_for_shader(PLASMA_SKSL, None).expect("Failed to make runtime effect");

        // Filters
        let drop_shadow =
            drop_shadow_only(Vector::new(1.5, 1.5), (0.5, 0.5), Color::from_argb(64, 0, 0, 0), None, None, None);
        let drop_shadow_white =
            drop_shadow_only(Vector::new(1.5, 1.5), (2.0, 2.0), Color::from_argb(64, 255, 255, 255), None, None, None);

        // Surface
        let surface = Skia::make_surface(
            &mut context,
            app_state.gfx.width * app_state.gfx.dpi as i32,
            app_state.gfx.height * app_state.gfx.dpi as i32,
        );

        Skia {
            context,
            surface,
            font_main: Font::from_typeface(font_mgr.new_from_data(MAIN_FONT, None).unwrap(), FONT_SIZE),
            font_ai: Font::from_typeface(font_mgr.new_from_data(AI_FONT, None).unwrap(), FONT_SIZE),
            font_echo: Font::from_typeface(font_mgr.new_from_data(ECHO_FONT, None).unwrap(), FONT_SIZE),
            _drop_shadow: drop_shadow,
            _drop_shadow_white: drop_shadow_white,
            noise_shader,
            plasma_shader,
            colour_background: Color::from_argb(255, 53, 53, 53),
        }
    }

    pub fn _test(&mut self, width: i32, height: i32) {
        let canvas = self.get_canvas();
        let mut rng = rand::rng();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        for _ in 1..=10000 {
            canvas.draw_line(
                Point {
                    x: rng.random_range(0..=width) as f32,
                    y: rng.random_range(0..=height) as f32,
                },
                Point {
                    x: rng.random_range(0..=width) as f32,
                    y: rng.random_range(0..=height) as f32,
                },
                &paint,
            );
        }
    }

    pub fn get_canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }

    pub unsafe fn flush(&mut self, dpi: f32, millis: f32) {
        self.surface.image_snapshot();
        self.context.flush_and_submit();

        // Clear
        let w = self.surface.width();
        let h = self.surface.height();
        self.get_canvas().clear(skia_safe::Color::TRANSPARENT);
        let mut paint_background = Paint::default();
        paint_background.set_color(self.colour_background);
        paint_background.set_style(PaintStyle::Fill);
        //paint_background.set_shader(self.create_parchment_shader(w as f32, h as f32, 0f32));
        paint_background.set_shader(self.create_plasma_shader(w as f32, h as f32, dpi, millis));
        self.get_canvas().draw_rect(Rect::from_xywh(0.0, 0.0, w as f32, h as f32), &paint_background);

        // Now overlay with darker
        paint_background.set_argb(180, 0, 0, 0);
        paint_background.set_shader(None);
        self.get_canvas().draw_rect(Rect::from_xywh(0.0, 0.0, w as f32, h as f32), &paint_background);

        // And add noise
        paint_background.set_argb(180, 0, 0, 0);
        paint_background.set_shader(self.create_noise_shader(Color::BLACK, 0.1));
        self.get_canvas().draw_rect(Rect::from_xywh(0.0, 0.0, w as f32, h as f32), &paint_background);
    }

    pub fn set_matrix(&mut self, gfx: &GFXState) {
        let canvas = self.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((gfx.dpi, gfx.dpi));
    }

    pub fn _set_matrix_centre(&mut self, gfx: &GFXState) {
        let canvas = self.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((gfx.dpi, gfx.dpi));
        canvas.translate((gfx._half_width, gfx._half_height));
    }

    pub fn set_matrix_camera(&mut self, app_state: &AppState) {
        let canvas = self.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((app_state.gfx.dpi, app_state.gfx.dpi));
        canvas.translate((app_state.gfx._half_width, app_state.gfx._half_height));
        canvas.scale((app_state._zoom, app_state._zoom));
        canvas.translate((-app_state._target.x, -app_state._target.y));
    }

    pub fn clear_matrix(&mut self) {
        let canvas = self.get_canvas();
        canvas.restore();
    }

    pub fn reset_context(&mut self) {
        self.context.reset(None);
    }

    pub fn create_plasma_shader(&mut self, width: f32, height: f32, dpi: f32, time: f32) -> Shader {
        let mut builder = RuntimeShaderBuilder::new(self.plasma_shader.clone());
        builder.set_uniform_float("u_time", &[time]).unwrap();
        builder.set_uniform_float("u_resolution", &[width, height]).unwrap();
        builder.set_uniform_float("u_dpi_scale", &[dpi]).unwrap();

        let m = Matrix::i();
        builder.make_shader(m).expect("Failed to create shader")
    }

    pub fn create_noise_shader(&mut self, base_color: Color, mix: f32) -> Shader {
        let uniforms = {
            let mut data = vec![];

            // Mix
            data.extend_from_slice(&mix.to_ne_bytes());

            // Colour
            let d = Color4f::from(base_color).as_array().iter().map(|&f| f.to_ne_bytes()).flatten().collect::<Vec<_>>();
            data.extend_from_slice(&d);

            Data::new_copy(&data)
        };
        self.noise_shader.clone().make_shader(uniforms, &[], None).expect("Make shader failed")
    }

    /*pub fn button(&mut self, text: &str, app_state: &AppState, xy: Vector) {
        let width = 328.0 * 0.4 / 2.0;
        let height = 196.0 * 0.4 / 2.0;

        {
            let canvas = self.get_canvas();
            canvas.save();
            canvas.reset_matrix();
            canvas.scale((app_state.gfx.dpi, app_state.gfx.dpi));
            canvas.translate(Vector::new(xy.x - width, xy.y));
            canvas.scale((0.4, 0.4));
            app_state.res.button_path.render(canvas);
            canvas.restore();
        }

        // Background Colour
        let mut paint_click = Paint::default();
        paint_click.set_style(Style::Fill);
        let rect = skia_safe::Rect::from_xywh(
            xy.x - width + 1.0,
            xy.y + 21.4,
            width * 2.0 - 1.0,
            height - 3.0,
        );
        if rect.contains(app_state.hover) {
            paint_click.set_color(self.button_hover);
        } else {
            paint_click.set_color(self.button);
        }
        self.get_canvas().draw_rect(rect, &paint_click);

        // Label
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_color(Color::WHITE);
        paint.set_image_filter(self.drop_shadow_white.clone());
        self.write_text_centre(
            30.0,
            &paint,
            text,
            Point::new(xy.x - width, xy.y + (196.0 * 0.4 / 2.0 / 2.0)),
            width * 2.0,
            &FontFamily::EbGaramondBold,
        );
        paint.set_image_filter(None);
        self.write_text_centre(
            30.0,
            &paint,
            text,
            Point::new(xy.x - width, xy.y + (height / 2.0)),
            width * 2.0,
            &FontFamily::EbGaramondBold,
        );
    }*/
}
