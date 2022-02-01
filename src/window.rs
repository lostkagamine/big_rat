use std::time::Duration;
use sdl2::{pixels::Color, event::Event, image::InitFlag, surface::Surface, image::LoadSurface};

const FRAME_RATE: u32 = 60;

pub fn make_window() {
    let sdl = sdl2::init().unwrap();
    let videosus = sdl.video().unwrap();

    let _sdlimg = sdl2::image::init(InitFlag::all()).unwrap();

    trace!("creating big rat window");

    let window = videosus
        .window("big rat", 971, 629)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let creator = canvas.texture_creator();

    trace!("loading big rat");

    let big_rat = Surface::from_file("rat.png").unwrap();
    let big_rat_tex = big_rat.as_texture(&creator).unwrap();

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    trace!("now entering main loop");

    let mut pump = sdl.event_pump().unwrap();
    'yeah: loop {
        canvas.clear();
        for evt in pump.poll_iter() {
            match evt {
                Event::Quit {..} => break 'yeah,
                _ => {}
            }
        }

        match canvas.copy(&big_rat_tex, None, None) {
            Ok(_) => {},
            Err(x) => panic!("canvas.copy: {}", x)
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAME_RATE));
    }

    trace!("exiting");

    info!("thank you for using big rat");
    info!("(c) 2022~ big rat laboratories - github.com/ry00001");
    info!("goodbye");
}