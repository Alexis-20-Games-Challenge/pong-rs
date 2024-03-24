use anyhow;
use sdl2::{
    self, 
    event::Event, 
    keyboard::Keycode, 
    pixels::Color, 
    rect::{Point, Rect}, 
    render::{Canvas, TextureCreator, TextureQuery}, 
    ttf::*, 
    video::{Window, WindowContext}, 
    EventPump, 
    Sdl, 
    VideoSubsystem
};   
use std::{
    fs,
    path::PathBuf,
    time::Duration
};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const CENTRE: u32 = WIDTH / 2;
const BALL_SIZE: u32 = 8;
const RECT_WIDTH: u32 = 16;
const RECT_HEIGHT: u32 = 64;

/// This is the gamestate struct 
struct State {
    score: (u8, u8),
    p1_centre: Point,
    p2_centre: Point,
    ball_centre: Point,
    sdl: Sdl,
    vid: VideoSubsystem,
    ttf: Sdl2TtfContext,
    canvas: Canvas<Window>,
    tex: TextureCreator<WindowContext>,
    events: EventPump,
}

impl State {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let vid = sdl.video().unwrap();
        let win = vid
            .window("Pong", 512, 512)
            .position_centered()
            .build()
            .unwrap();
        let ttf = sdl2::ttf::init().unwrap();
        let canvas = win.into_canvas().build().unwrap();
        let tex = canvas.texture_creator();
        let events = sdl.event_pump().unwrap();

        State {
            score: (0u8, 0u8),
            p1_centre: Point::new(32, (HEIGHT / 2).try_into().unwrap()),
            p2_centre: Point::new(
                (WIDTH - 32).try_into().unwrap(), 
                (HEIGHT / 2).try_into().unwrap()
            ),
            ball_centre: Point::new(
                (WIDTH / 2).try_into().unwrap(), 
                (HEIGHT / 2).try_into().unwrap()
            ),
            sdl,
            vid,
            ttf,
            canvas,
            tex,
            events,
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Welcome message to all you lovely lot
    println!("Welcome to Alexis' Pong!");

    // Initialise gamestate
    let mut state = State::new();

    // Initialises the window to start with a black background before any rendering
    state.canvas.set_draw_color(Color::BLACK);
    state.canvas.clear();
    state.canvas.present();

    // Cool and concise way of doing a "while flag" loop. Me like.
    'running: loop {
        for event in state.events.poll_iter() {
            match event {
                #[rustfmt::skip]
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Draw and then sleep for 1 60th of a second i.e. locks to 60fps.
        // No high refresh rate thrilling pong gameplay for you.
        draw(&mut state);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    Ok(())
}

fn draw(state: &mut State) {
    state.canvas.clear();

    // Font rendering
    let font_path = fs::canonicalize("./assets/RobotoMono-Bold.ttf").unwrap();
    let mut font = state.ttf.load_font(font_path, 72).unwrap();
    let surface = font
        .render("Test")
        .solid(Color::WHITE)
        .unwrap();
    let texture = state.tex
        .create_texture_from_surface(&surface)
        .unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(CENTRE as i32, CENTRE as i32, width, height);
    state.canvas.copy(&texture, None, Some(target));

    state.canvas.present();
}