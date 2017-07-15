extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::error::Error;
use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct GameManager {
    /// Represents the state of the game
    gl: GlGraphics,
    started: bool,
    board: gobs::Board,
}

impl GameManager {
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

pub fn run() -> Result<(), Box<Error>> {
    const WINDOW_XY: f64 = 300.0;
    let mut window: Window = WindowSettings::new("WHACK!", [WINDOW_XY as u32, WINDOW_XY as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = GameManager {
        gl: GlGraphics::new(OpenGL::V3_2),
        started: false,
        board: gobs::Board::from_length(WINDOW_XY),
    };

    println!("PRESS SPACE TO START!");

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                if !game.started {
                    println!("START!");
                    game::add_tile(&mut game.board);
                }
            }
        }
    }

    Ok(())
}

pub mod colours {
    pub type Colour = [f32; 4];
    pub const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
    pub const RED: Colour = [1.0, 0.0, 0.0, 1.0];
    pub const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
    pub const YELLOW: Colour = [1.0, 1.0, 0.0, 1.0];
    pub const MAGENTA: Colour = [1.0, 0.0, 1.0, 1.0];
    pub const CYAN: Colour = [0.0, 1.0, 1.0, 1.0];
    pub const WHITE: Colour = [1.0, 1.0, 1.0, 1.0];
    pub const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
}

pub mod gobs {
    extern crate graphics;

    use colours::Colour;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Tile {
        pub rect: [f64; 4],
        pub pos: Point,
        pub colour: Colour,
    }

    impl Tile {
        /// Returns a tile struct
        ///
        /// ```
        /// use whack::colours;
        /// use whack::gobs::Tile;
        /// let tile = Tile::new(100.0, 100.0, 50.0, colours::BLUE);
        /// ```
        pub fn new(x: f64, y: f64, wh: f64, colour: Colour) -> Tile {
            Tile {
                rect: graphics::rectangle::square(0.0, 0.0, wh),
                pos: Point { x: x, y: y },
                colour: colour,
            }
        }
    }

    #[derive(Debug)]
    pub struct Board {
        pub tiles: Tiles,
        pub length: f64,
    }

    impl Board {
        /// Returns a Board struct with an empty Tiles array
        ///
        /// ```
        /// use whack::gobs::Board;
        /// let board = Board::from_length(300.0);
        /// ```
        pub fn from_length(length: f64) -> Board {
            Board {
                tiles: [None; 9],
                length: length,
            }
        }
    }

    pub type MaybeTile = Option<Tile>;
    pub type Tiles = [MaybeTile; 9];
}

pub mod game {
    extern crate rand;

    use rand::Rng;
    use colours::RED;
    use gobs::{Board, Tile};

    pub fn add_tile(board: &mut Board) {
        let new_pos = random_position();
        let new_tile = Tile::new(x_from_index(new_pos, board.length),
                                 y_from_index(new_pos, board.length),
                                 board.length / 3.0,
                                 RED);
        println!("{:?}", new_pos);
        board.tiles[new_pos] = Some(new_tile);
    }

    fn random_position() -> usize {
        let pos = rand::thread_rng().gen_range(0, 9);
        pos
    }

    fn x_from_index(i: usize, board_length: f64) -> f64 {
        let tile_length = board_length / 3.0;
        println!("{:?} {:?}", i, (i as f64 % 3.0));

        ((i as f64 % 3.0) * tile_length) + (0.5 * tile_length)
    }

    fn y_from_index(i: usize, board_length: f64) -> f64 {
        let tile_length = board_length / 3.0;

        ((i as f64 / 3.0).floor() * tile_length) + (0.5 * tile_length)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use gobs;

        #[test]
        fn gen_random_index() {
            for _ in 1..10 {
                let i = random_position();
                assert!(i <= 8);
            }
        }

        #[test]
        fn check_x_from_i() {
            assert_eq!(x_from_index(0, 300.0), 50.0);
            assert_eq!(x_from_index(1, 300.0), 150.0);
            assert_eq!(x_from_index(2, 300.0), 250.0);
            assert_eq!(x_from_index(8, 300.0), 250.0);
        }

        #[test]
        fn check_y_from_i() {
            assert_eq!(y_from_index(0, 300.0), 50.0);
            assert_eq!(y_from_index(1, 300.0), 50.0);
            assert_eq!(y_from_index(2, 300.0), 50.0);
            assert_eq!(y_from_index(8, 300.0), 250.0);
        }

        #[test]
        fn check_tile_addtion() {
            let mut board = gobs::Board::from_length(300.0);
            add_tile(&mut board);
            let is_some_array: Vec<bool> = board.tiles.iter().map(|x| x.is_some()).collect();
            assert!(is_some_array.contains(&true));
        }
    }
}