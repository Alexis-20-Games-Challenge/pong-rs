use anyhow;
use sdl2::{
    self,
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, TextureCreator, TextureQuery},
    ttf::*,
    video::{Window, WindowContext},
    EventPump, Sdl, VideoSubsystem,
};
use std::{fs, path::PathBuf, str::Matches, time::Duration};
use vec2;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const CENTRE: u32 = WIDTH / 2;
const BALL_SIZE: u32 = 8;
const RECT_WIDTH: u32 = 16;
const RECT_HEIGHT: u32 = 64;
const FONT_PATH: &str = "./src/assets/RobotoMono-Bold.ttf";
const PADDLE_V: i32 = 8;

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
    ball_d: [i32; 2],
    ball_s: i32,
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
                (HEIGHT / 2).try_into().unwrap(),
            ),
            ball_centre: Point::new(
                (WIDTH / 2).try_into().unwrap(),
                (HEIGHT / 2).try_into().unwrap(),
            ),
            sdl,
            vid,
            ttf,
            canvas,
            tex,
            events,
            ball_d: vec2::new(1, 0),
            ball_s: 4,
        }
    }
}

/// Macro because I am lazy
macro_rules! key_event {
    ($key:pat) => {
        Event::KeyDown {
            keycode: Some($key),
            ..
        }
    };
    () => {};
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
                Event::Quit { .. } | key_event!(Keycode::Escape) => break 'running,
                _ => {}
            }
        }

        let mut is_reset_queued = false;
        for scancode in state.events.keyboard_state().pressed_scancodes() {
            if PADDLE_V + RECT_HEIGHT as i32 / 2 <= state.p1_centre.y {
                if scancode == Scancode::W {
                    state.p1_centre.y = state.p1_centre.y - PADDLE_V;
                }
            }

            if state.p1_centre.y < (HEIGHT - RECT_HEIGHT / 2) as i32 {
                if scancode == Scancode::S {
                    state.p1_centre.y = state.p1_centre.y + PADDLE_V;
                }
            }

            if PADDLE_V + RECT_HEIGHT as i32 / 2 <= state.p2_centre.y {
                if scancode == Scancode::Up {
                    state.p2_centre.y = state.p2_centre.y - PADDLE_V;
                }
            }

            if state.p2_centre.y < (HEIGHT - RECT_HEIGHT / 2) as i32 {
                if scancode == Scancode::Down {
                    state.p2_centre.y = state.p2_centre.y + PADDLE_V;
                }
            }

            if scancode == Scancode::R {
                is_reset_queued = true;
            }
        }

        if is_reset_queued {
            ball_reset(&mut state);
        }

        ball_handle(&mut state);

        // Draw and then sleep for 1 60th of a second i.e. locks to 60fps.
        // No high refresh rate thrilling pong gameplay for you.
        draw(&mut state);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    Ok(())
}

fn draw(state: &mut State) {
    state.canvas.set_draw_color(Color::BLACK);
    state.canvas.clear();
    draw_centre_line(state);
    draw_score(state);
    draw_players(state);
    draw_ball(state);
    state.canvas.present();
}

fn get_centred_rect(cx: u32, cy: u32, w: u32, h: u32) -> Rect {
    Rect::new((cx - w / 2) as i32, (cy - h / 2) as i32, w, h)
}

fn draw_score(state: &mut State) {
    // Font rendering
    let mut font = state.ttf.load_font(FONT_PATH, 72).unwrap();

    let surface = font
        .render(&format!("{}   {}", state.score.0, state.score.1)[..])
        .solid(Color::WHITE)
        .unwrap();
    let texture = state.tex.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let target = get_centred_rect(CENTRE, height / 2, width, height);
    state.canvas.copy(&texture, None, Some(target)).unwrap();
}

fn draw_centre_line(state: &mut State) {
    state.canvas.set_draw_color(Color::WHITE);
    state
        .canvas
        .draw_line(
            Point::new(CENTRE as i32, 0i32),
            Point::new(CENTRE as i32, HEIGHT as i32),
        )
        .unwrap();
    state.canvas.set_draw_color(Color::BLACK)
}

fn draw_players(state: &mut State) {
    state.canvas.set_draw_color(Color::WHITE);
    state
        .canvas
        .fill_rect(get_centred_rect(
            state.p1_centre.x as u32,
            state.p1_centre.y as u32,
            RECT_WIDTH,
            RECT_HEIGHT,
        ))
        .unwrap();
    state
        .canvas
        .fill_rect(get_centred_rect(
            state.p2_centre.x as u32,
            state.p2_centre.y as u32,
            RECT_WIDTH,
            RECT_HEIGHT,
        ))
        .unwrap();
    state.canvas.set_draw_color(Color::WHITE);
}

fn draw_ball(state: &mut State) {
    state.canvas.set_draw_color(Color::WHITE);
    state
        .canvas
        .fill_rect(get_centred_rect(
            state.ball_centre.x as u32,
            state.ball_centre.y as u32,
            BALL_SIZE,
            BALL_SIZE,
        ))
        .unwrap();
    state.canvas.set_draw_color(Color::BLACK);
}

fn ball_handle(state: &mut State) {
    // Scoring
    if state.ball_centre.x <= 8 {
        state.score.1 += 1;
        ball_reset(state);
    } else if state.ball_centre.x >= WIDTH as i32 - 8 {
        state.score.0 += 1;
        ball_reset(state);
    }

    // top/bottom collisions
    if state.ball_centre.y <= BALL_SIZE as i32 + state.ball_s {
        state.ball_d[1] = -state.ball_d[1];
    } else if state.ball_centre.y >= (HEIGHT - (BALL_SIZE + state.ball_s as u32)) as i32 {
        state.ball_d[1] = -state.ball_d[1];
    }

    // Player 1 ball bounce
    if get_centred_rect(
        state.ball_centre.x as u32,
        state.ball_centre.y as u32,
        BALL_SIZE,
        BALL_SIZE,
    )
    .has_intersection(get_centred_rect(
        state.p1_centre.x as u32,
        state.p1_centre.y as u32,
        RECT_WIDTH,
        RECT_HEIGHT,
    )) {
        state.ball_d = [
            (state.ball_centre.x - state.p1_centre.x) / 8 + 1,
            (state.ball_centre.y - state.p1_centre.y) / 8,
        ];
        state.ball_s += 1;
    }

    // Player 2 ball bounce
    if get_centred_rect(
        state.ball_centre.x as u32,
        state.ball_centre.y as u32,
        BALL_SIZE,
        BALL_SIZE,
    )
    .has_intersection(get_centred_rect(
        state.p2_centre.x as u32,
        state.p2_centre.y as u32,
        RECT_WIDTH,
        RECT_HEIGHT,
    )) {
        state.ball_d = [
            (state.ball_centre.x - state.p2_centre.x) / 8 - 1,
            (state.ball_centre.y - state.p2_centre.y) / 8,
        ];
        state.ball_s += 1;
    }

    // Move
    state.ball_centre.x = state.ball_centre.x + state.ball_d[0] * (state.ball_s / 4);
    state.ball_centre.y = state.ball_centre.y + state.ball_d[1] * (state.ball_s / 4);
}

fn ball_reset(state: &mut State) {
    match (state.score.0 + state.score.1) % 2 == 0 {
        true => state.ball_d = vec2::new(1, 0),
        false => state.ball_d = vec2::new(-1, 0),
    };
    state.ball_s = 4;
    state.ball_centre = Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2);
}
