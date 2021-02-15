use sdl2::video::Window;


#[test]
pub fn main() ->Result<(),NorErr>{
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use std::time::Duration;
    let mut main = MainSystem::init().unwrap();
    let mut i = 0;
    let video=main.video()?;
    let window=NWindow::from_video(video, "Nor_lib")?;
    let mut canvas = window.to_canvas()?;
    let mut events=main.system.event_pump().unwrap();
    'running: loop {
        i = (i + 1) % 255;
        canvas.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.canvas.clear();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.canvas.present();
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
    #[must_use = "result should be checked"]
    pub fn init() -> Result<Self, NorErr> {
        if let Ok(g) = sdl2::init() {
            Ok(MainSystem { system: g })
        } else {
            return Err(NorErr::SysInitErr);
        }
    }
    pub fn video(&mut self) -> Result<NVideo, NorErr> {
        if let Ok(g) = self.system.video() {
            Ok(NVideo{
                video:g
            })
        } else {
            Err(NorErr::VideoInitErr)
        }
    }
}

pub struct NVideo{
    video:sdl2::VideoSubsystem
}
pub struct NWindow {
    window: sdl2::video::Window,
}

impl NWindow {
    pub fn from_video(video: NVideo, title: &str) -> Result<NWindow, NorErr> {
        if let Ok(g) = video.video
            .window(title, 800, 640)
            .maximized()
            .resizable()
            .build()
        {
            Ok(NWindow { window: g })
        } else {
            Err(NorErr::WindowInitErr)
        }
    }
    pub fn to_canvas(self)->Result<NCanvas,NorErr>{
        if let Ok(g)=self.window.into_canvas().build() {
            Ok(NCanvas{
                canvas:g
            })
        }else{
            Err(NorErr::CanvasInitErr)
        }
    }
}

pub struct NCanvas{
    canvas:sdl2::render::Canvas<sdl2::video::Window>
}
