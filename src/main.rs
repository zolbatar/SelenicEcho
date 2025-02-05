use crate::app_state::AppState;
use crate::printer::{PrintStyle, Printer};
use crate::skia::Skia;
use sdl2::event::Event;
use sdl2::video::GLProfile;
use std::process::exit;
use std::time::{Duration, Instant};

mod app_state;
mod printer;
mod skia;

fn main() {
    setup_sdl_skia()
}

fn setup_sdl_skia() {
    // Initialize SDL2
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // Set OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3); // OpenGL 3.3

    // Create an SDL2 window
    let window = video_subsystem
        .window("Simulation", 1500, 900)
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    // Create an OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();

    // Load OpenGL functions
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Get display index (typically 0 is the default display)
    let display_index = 0;

    // Get DPI information
    let mut dpi = 0.0;
    match video_subsystem.display_dpi(display_index) {
        Ok((d_dpi, hdpi, v_dpi)) => {
            println!("Diagonal DPI: {}", d_dpi);
            println!("Horizontal DPI: {}", hdpi);
            println!("Vertical DPI: {}", v_dpi);

            // Calculate scaling factor
            dpi = hdpi / 96.0; // 96 DPI is considered the default "normal" DPI
            println!("Scaling factor: {}", dpi);
        }
        Err(e) => {
            eprintln!("Could not get DPI information: {}", e);
        }
    }
    let dpi = dpi.floor();

    // Event pump for SDL2 events
    let mut event_pump = sdl.event_pump().unwrap();

    // Store the time of the previous frame and the last time we measured FPS
    let mut frame_count = 0;
    let mut last_fps_check = Instant::now();
    let fps_check_interval = Duration::from_secs(1); // Check FPS every second

    let mut app_state = AppState::new(window, dpi);
    let mut skia = Skia::new(&app_state);
    unsafe {
        skia.flush(app_state.gfx.dpi, 0.0);
    }
    let start = Instant::now();
    let mut printer = Printer::new(&skia);
    printer.print(String::from("No one shall be subjected to arbitrary arrest, detention or exile. Everyone is entitled in full equality to a fair and public hearing by an independent and impartial tribunal, in the determination of his rights and obligations and of any criminal charge against him. No one shall be subjected to arbitrary interference with his privacy, family, home or correspondence, nor to attacks upon his honour and reputation. Everyone has the right to the protection of the law against such interference or attacks."));
    loop {
        // Measure the time it took to render the previous frame
        let current_time = Instant::now();
        app_state.phase = (current_time.duration_since(start).as_millis() as f32 / 250.0) % 2.0;

        // Render!
        skia.set_matrix(&app_state.gfx);
        printer.print_render(&mut skia, &app_state.gfx, PrintStyle::NORMAL);
        // skia.test(app_state.gfx.width, app_state.gfx.height);
        unsafe {
            skia.flush(app_state.gfx.dpi, start.elapsed().as_secs_f32());
        }

        // Increment the frame count
        frame_count += 1;

        // Calculate FPS every second
        if current_time - last_fps_check >= fps_check_interval {
            app_state.fps = frame_count as f64 / (fps_check_interval.as_secs_f64());

            // Reset frame count and last FPS check time
            frame_count = 0;
            last_fps_check = current_time;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => exit(0),
                _ => {}
            }
        }

        app_state.gfx.window.gl_swap_window();
    }
}
