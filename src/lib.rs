#[test]
pub fn main() -> Result<(), NorErr> {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use std::time::Duration;
    let mut main = MainSystem::init().unwrap();
    let mut i = 0;
    let window = main.video()?.window("Nor_lib")?;
    let mut canvas = window.into_canvas()?;
    let mut event = main.event()?;

    'running: loop {
        i = (i + 1) % 255;
        canvas.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for eve in event.poll_iter() {
            match eve {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.display();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

pub struct MainSystem {
    system: sdl2::Sdl,
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
    pub fn raw(&mut self) -> &mut sdl2::Sdl {
        &mut self.system
    }

    #[must_use = "result should be checked"]
    pub fn init() -> Result<Self, NorErr> {
        if let Ok(g) = sdl2::init() {
            Ok(MainSystem { system: g })
        } else {
            return Err(NorErr::SysInitErr);
        }
    }

    #[must_use = "result should be checked"]
    pub fn video(&mut self) -> Result<Nvideo, NorErr> {
        if let Ok(g) = self.system.video() {
            Ok(Nvideo { video: g })
        } else {
            Err(NorErr::VideoInitErr)
        }
    }

    #[must_use = "result should be checked"]
    pub fn event(&mut self) -> Result<Nevent, NorErr> {
        if let Ok(event) = self.system.event_pump() {
            Ok(Nevent { event })
        } else {
            Err(NorErr::EventInitErr)
        }
    }
}

pub struct Nevent {
    event: sdl2::EventPump,
}

impl Nevent {
    pub fn raw(&mut self) -> &mut sdl2::EventPump {
        &mut self.event
    }
    pub fn poll_iter(&mut self) -> sdl2::event::EventPollIterator {
        self.event.poll_iter()
    }
}

pub struct Nvideo {
    video: sdl2::VideoSubsystem,
}
impl Nvideo {
    pub fn raw(&mut self) -> &mut sdl2::VideoSubsystem {
        &mut self.video
    }

    pub fn window(&self, title: &str) -> Result<Nwindow, NorErr> {
        if let Ok(window) = self
            .video
            .window(title, 800, 640)
            .maximized()
            .resizable()
            .position_centered()
            .build()
        {
            Ok(Nwindow { window })
        } else {
            Err(NorErr::WindowInitErr)
        }
    }
}

pub struct Nwindow {
    window: sdl2::video::Window,
}

impl Nwindow {
    #[must_use = "result should be checked"]
    pub fn from_video(video: Nvideo, title: &str) -> Result<Nwindow, NorErr> {
        if let Ok(g) = video
            .video
            .window(title, 800, 640)
            .maximized()
            .resizable()
            .build()
        {
            Ok(Nwindow { window: g })
        } else {
            Err(NorErr::WindowInitErr)
        }
    }
    pub fn into_canvas(self) -> Result<Ncanvas, NorErr> {
        if let Ok(g) = self.window.into_canvas().build() {
            Ok(Ncanvas { canvas: g })
        } else {
            Err(NorErr::CanvasInitErr)
        }
    }

    pub fn raw(&mut self) -> &mut sdl2::video::Window {
        &mut self.window
    }
}

pub struct Ncanvas {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Ncanvas{
/*     pub fn set_color(){

    }
*/

    pub fn clear(&mut self){
        self.canvas.clear()
    }

    pub fn display(&mut self){
        self.canvas.present()
    }
}
