extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate whack;

use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use whack::colours;
use whack::game;
use whack::gobs;

pub struct App {
    gl: GlGraphics,
    started: bool,
    board: gobs::Board,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let board = &self.board;
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(colours::BLUE, gl);
            for otile in board.tiles.iter() {
                if otile.is_some() {
                    let tile = otile.unwrap();
                    let transform = c.transform
                        .trans(tile.pos.x, tile.pos.y)
                        .trans(-(tile.rect[2] / 2.0), -(tile.rect[3] / 2.0));
                    graphics::rectangle(tile.colour, tile.rect, transform, gl);
                }
            }
        });
    }
}

fn main() {
    const WINDOW_XY: f64 = 300.0;
    let mut window: Window = WindowSettings::new("WHACK!", [WINDOW_XY as u32, WINDOW_XY as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(OpenGL::V3_2),
        started: false,
        board: game::initialise_board(WINDOW_XY),
    };

    println!("PRESS SPACE TO START!");

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                if !app.started {
                    println!("START!");
                    game::add_tile(&mut app.board);
                }
            }
        }
    }
}