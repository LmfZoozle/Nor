use nor::{libc::system, render::Canvas, video::Window, EventPump};
use sdl2 as nor;

pub fn origin() {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use std::time::Duration;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    //    let window = window::new_window(& sdl_context,"Nor_lib").unwrap();
    let window = video_subsystem
        .window("Nor_lib", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[test]
pub fn main() {
    use nor::event::Event;
    use nor::keyboard::Keycode;
    use nor::pixels::Color;
    use std::time::Duration;
    let mut main = MainSystem::init("Nor_lib").unwrap();
    let mut i = 0;
    let canvas = main.canvas();
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in main.event_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub struct MainSystem {
    system: sdl2::Sdl,
    event: sdl2::EventPump,
    video: sdl2::VideoSubsystem,
    //window: sdl2::video::Window,
    canvas: Canvas<Window>,
}

#[derive(Debug)]
pub enum NorErr {
    SysInitErr,
    EventInitErr,
    VideoInitErr,
    WindowInitErr,
    CanvasInitErr,
}

impl MainSystem {
    #[must_use = "result should be checked"]
    pub fn init(title: &str) -> Result<Self, NorErr> {
        let system;
        let event;
        let video;
        let window;
        let canvas;

        if let Ok(g) = nor::init() {
            system = g;
        } else {
            return Err(NorErr::SysInitErr);
        }

        if let Ok(g) = system.event_pump() {
            event = g;
        } else {
            return Err(NorErr::EventInitErr);
        }

        if let Ok(g) = system.video() {
            video = g;
        } else {
            return Err(NorErr::VideoInitErr);
        }

        if let Ok(g) = video
            .window(title, 160, 90)
            .position_centered()
            .fullscreen()
            .build()
        {
            window = g;
        } else {
            return Err(NorErr::WindowInitErr);
        }

        if let Ok(g) = window.into_canvas().build() {
            canvas = g;
        } else {
            return Err(NorErr::CanvasInitErr);
        }

        Ok(MainSystem {
            system,
            event,
            video,
            //      window,
            canvas,
        })
    }

    pub fn event_iter(&mut self) -> nor::event::EventPollIterator {
        self.event.poll_iter()
    }
    pub fn canvas(&mut self) -> &mut nor::render::Canvas<Window> {
        &mut self.canvas
    }
}
