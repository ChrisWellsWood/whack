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
    board: gobs::Board,
    cursor: gobs::Sprite,
    started: bool,
    max_time: f64,
    tile_timer: f64,
}

impl GameManager {
    /// Returns the game manager.
    pub fn new(window_size: f64, max_time: f64) -> GameManager {
        GameManager {
            gl: GlGraphics::new(OpenGL::V3_2),
            board: gobs::Board::from_length(window_size),
            cursor: gobs::Sprite::new(window_size / 2.0,
                                      window_size / 2.0,
                                      window_size / 16.0,
                                      colours::YELLOW),
            started: false,
            max_time: max_time,
            tile_timer: 0.0,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let sprites = self.get_sprites();
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(colours::BLUE, gl);
            for sprite in sprites {
                let transform = c.transform
                    .trans(sprite.pos.x, sprite.pos.y)
                    .trans(-(sprite.rect[2] / 2.0), -(sprite.rect[3] / 2.0));
                graphics::rectangle(sprite.colour, sprite.rect, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.started {
            self.tile_timer -= args.dt;
            if self.tile_timer < 0.0 {
                self.tile_timer = self.max_time;
                self.board.add_tile();
            }
        }
    }

    fn get_sprites(&self) -> Vec<gobs::Sprite> {
        // Could add tags to sprites and filter them later on
        // Add field for layer to sprite
        let mut sprites: Vec<gobs::Sprite> = self.board
            .tiles
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        sprites.push(self.cursor);
        sprites
    }
}

pub fn run() -> Result<(), Box<Error>> {
    const WINDOW_XY: f64 = 300.0;
    let mut window: Window = WindowSettings::new("WHACK!", [WINDOW_XY as u32, WINDOW_XY as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = GameManager::new(WINDOW_XY, 1.0);

    println!("PRESS SPACE TO START!");

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            handle_key_press(&mut game, key);
        }
    }

    Ok(())
}

fn handle_key_press(game: &mut GameManager, key: piston::input::Key) {
    if key == Key::Space {
        if !game.started {
            game.started = true;
        }
    }
    if key == Key::Backspace {
        game.board.clear_board();
    }
}

#[cfg(test)]
mod tests {
    extern crate piston;
    extern crate glutin_window;

    use super::*;

    fn make_manager() -> GameManager {
        const WINDOW_XY: f64 = 300.0;
        let window: glutin_window::GlutinWindow =
            piston::window::WindowSettings::new("WHACK!", [WINDOW_XY as u32, WINDOW_XY as u32])
                .exit_on_esc(true)
                .build()
                .unwrap();
        GameManager::new(WINDOW_XY, 3.0)
    }

    #[test]
    fn get_sprites() {
        let mut game = make_manager();
        let sprites = game.get_sprites();
        assert_eq!(sprites.len(), 1);
        game.board.add_tile();
        let sprites = game.get_sprites();
        assert_eq!(sprites.len(), 2);
    }

    #[test]
    fn move_cursor() {
        let mut game = make_manager();
        game.cursor.translate(-100.0, 0.0);
        assert_eq!(game.cursor.pos.x, 50.0);
        assert_eq!(game.cursor.pos.y, 150.0);
        game.cursor.translate(100.0, 100.0);
        assert_eq!(game.cursor.pos.x, 150.0);
        assert_eq!(game.cursor.pos.y, 250.0);
    }
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
    extern crate rand;

    use rand::sample;
    use colours::{Colour, RED};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Sprite {
        pub rect: [f64; 4],
        pub pos: Point,
        pub colour: Colour,
    }

    impl Sprite {
        /// Returns a tile struct
        ///
        /// ```
        /// use whack::colours;
        /// use whack::gobs::Sprite;
        /// let tile = Sprite::new(100.0, 100.0, 50.0, colours::BLUE);
        /// ```
        pub fn new(x: f64, y: f64, wh: f64, colour: Colour) -> Sprite {
            Sprite {
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

        pub fn free_positions(&self) -> Vec<usize> {
            let positions: Vec<usize> = self.tiles
                .iter()
                .enumerate()
                .filter(|t| t.1.is_none())
                .map(|t| t.0)
                .collect();
            positions
        }

        pub fn is_full(&self) -> bool {
            if self.free_positions().is_empty() {
                true
            } else {
                false
            }
        }

        pub fn add_tile(&mut self) {
            let new_pos = self.random_position();
            if let Some(i) = new_pos {
                let new_tile = Sprite::new(self.x_from_index(i, self.length),
                                           self.y_from_index(i, self.length),
                                           self.length / 3.0,
                                           RED);
                self.tiles[i] = Some(new_tile);
            }
        }

        fn random_position(&self) -> Option<usize> {
            let free_positions = self.free_positions();
            if free_positions.is_empty() {
                return None;
            }
            let mut rng = rand::thread_rng();
            let sample = sample(&mut rng, free_positions.into_iter(), 1);
            Some(sample[0])
        }

        fn x_from_index(&self, i: usize, board_length: f64) -> f64 {
            let tile_length = board_length / 3.0;
            ((i as f64 % 3.0) * tile_length) + (0.5 * tile_length)
        }

        fn y_from_index(&self, i: usize, board_length: f64) -> f64 {
            let tile_length = board_length / 3.0;
            ((i as f64 / 3.0).floor() * tile_length) + (0.5 * tile_length)
        }

        pub fn clear_board(&mut self) {
            self.tiles = [None; 9];
        }
    }

    pub type MaybeSprite = Option<Sprite>;
    pub type Tiles = [MaybeSprite; 9];

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_tile() {
            let mut board = Board::from_length(300.0);
            board.add_tile();
            let is_some_array: Vec<bool> = board.tiles.iter().map(|x| x.is_some()).collect();
            assert!(is_some_array.contains(&true));
        }

        #[test]
        fn free_positions() {
            let mut board = Board::from_length(300.0);
            board.add_tile();
            assert_eq!(board.free_positions().len(), 8);
        }

        #[test]
        fn clear_board() {
            let mut board = Board::from_length(300.0);
            for _ in 0..8 {
                board.add_tile();
            }
            assert!(!board.is_full());
            board.add_tile();
            assert!(board.is_full());
            board.clear_board();
            assert!(!board.is_full());
        }

        #[test]
        fn gen_random_index() {
            let board = Board::from_length(300.0);
            for _ in 1..10 {
                if let Some(i) = board.random_position() {
                    assert!(i <= 8);
                }
            }
        }

        #[test]
        fn check_x_from_i() {
            let board = Board::from_length(300.0);
            assert_eq!(board.x_from_index(0, 300.0), 50.0);
            assert_eq!(board.x_from_index(1, 300.0), 150.0);
            assert_eq!(board.x_from_index(2, 300.0), 250.0);
            assert_eq!(board.x_from_index(8, 300.0), 250.0);
        }

        #[test]
        fn check_y_from_i() {
            let board = Board::from_length(300.0);
            assert_eq!(board.y_from_index(0, 300.0), 50.0);
            assert_eq!(board.y_from_index(1, 300.0), 50.0);
            assert_eq!(board.y_from_index(2, 300.0), 50.0);
            assert_eq!(board.y_from_index(8, 300.0), 250.0);
        }
    }
}