use sdl2::video::Window;
use skia_safe::Point;

pub const MIN_ZOOM: f32 = 32.0;

pub struct GFXState {
    pub window: Window,
    pub width: i32,
    pub height: i32,
    pub _half_width: i32,
    pub _half_height: i32,
    pub dpi: f32,
}

pub struct AppState {
    pub gfx: GFXState,
    pub fps: f64,
    pub phase: f32,
    pub _zoom: f32,
    pub _target: Point,
}

impl AppState {
    pub fn new(window: Window, dpi: f32) -> Self {
        let width = window.size().0 as i32;
        let height = window.size().1 as i32;
        println!("Screen resolution: {}x{}", width, height);
        let half_width = width / 2;
        let half_height = height / 2;

        AppState {
            gfx: GFXState {
                window,
                width,
                height,
                _half_width: half_width,
                _half_height: half_height,
                dpi,
            },
            fps: 0.0,
            phase: 0.0,
            _zoom: MIN_ZOOM,
            _target: Point::new(0.0, 50.0),
        }
    }
}
