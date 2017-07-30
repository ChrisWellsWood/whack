//! Contains the data structures and functions used to run an instance of **Whack!**

pub mod colours;
pub mod gobs;

extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::error::Error;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

/// Represents the state of the game.
#[derive(Debug, PartialEq)]
pub enum GameState {
    Ready,
    Playing,
    Win,
    Lose,
}

/// Initialises an instance of **Whack!**
pub fn run() -> Result<(), Box<Error>> {
    const WINDOW_XY: f64 = 300.0;
    let window: Window = WindowSettings::new("WHACK!", [WINDOW_XY as u32, WINDOW_XY as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = GameManager::new(WINDOW_XY, 1.0, 0.1);
    game.start(window)
}

/// The `GameManager` struct contains data and methods to run an instance of **Whack!**
pub struct GameManager {
    pub gl: GlGraphics,
    pub board: gobs::Board,
    pub cursor: gobs::Sprite,
    pub state: GameState,
    pub score: u32,
    pub max_time: f64,
    pub min_time: f64,
    pub tile_timer: f64,
}

impl PartialEq for GameManager {
    fn eq(&self, other: &GameManager) -> bool {
        (self.board == other.board) && (self.cursor == other.cursor) &&
        (self.state == other.state) && (self.score == other.score) &&
        (self.max_time == other.max_time) && (self.tile_timer == other.tile_timer)
    }
}

impl GameManager {
    /// Returns a new game manager struct.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate whack;
    /// extern crate piston;
    /// extern crate glutin_window;
    ///
    /// const WINDOW_XY: f64 = 300.0;
    /// let window: glutin_window::GlutinWindow =
    ///     piston::window::WindowSettings::new("WHACK!", [WINDOW_XY as u32, WINDOW_XY as u32])
    ///         .exit_on_esc(true)
    ///         .build()
    ///         .unwrap();
    /// whack::GameManager::new(WINDOW_XY, 3.0, 1.0);
    /// ```
    pub fn new(window_size: f64, max_time: f64, min_time: f64) -> GameManager {
        let cursor_width = window_size / 16.0;
        let cursor_height = window_size / 16.0;
        GameManager {
            gl: GlGraphics::new(OpenGL::V3_2),
            board: gobs::Board::from_length(window_size),
            cursor: gobs::Sprite::new((window_size / 2.0) - (0.5 * cursor_width),
                                      (window_size / 2.0) - (0.5 * cursor_height),
                                      cursor_width,
                                      cursor_height,
                                      colours::YELLOW),
            state: GameState::Ready,
            score: 0,
            max_time: max_time,
            min_time: min_time,
            tile_timer: 0.0,
        }
    }

    /// Resets the state of the `GameManager`.
    pub fn reset(&mut self) {
        self.board.clear_board();
        self.cursor.pos = gobs::Vec2D {
            x: (self.board.length / 2.0) - (0.5 * self.cursor.width),
            y: (self.board.length / 2.0) - (0.5 * self.cursor.height),
        };
        self.state = GameState::Ready;
        self.score = 0;
        self.tile_timer = 0.0;
    }

    /// Initialises the event loop for the game instance.
    pub fn start(&mut self, mut window: Window) -> Result<(), Box<Error>> {
        println!("PRESS SPACE TO START!");
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.input(key);
            }
        }

        Ok(())
    }

    /// Called by the event loop when a `Render` event is recieved.
    fn render(&mut self, args: &RenderArgs) {
        let sprites = self.get_sprites();
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(colours::BLUE, gl);
            for sprite in sprites {
                graphics::rectangle(sprite.colour, sprite.get_rect(), c.transform, gl);
            }
        });
    }

    /// Called by the event loop when an `Update` event is recieved.
    fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            GameState::Playing => self.playing_update(args),
            _ => (),
        }
    }

    /// Called by `update` when the `GameState` is `Playing`.
    fn playing_update(&mut self, args: &UpdateArgs) {
        self.tile_timer -= args.dt;
        if self.tile_timer < 0.0 {
            if self.score < 100 {
                let score_delta = (self.max_time - self.min_time) * (self.score as f64 / 100.0);
                self.tile_timer = self.max_time - score_delta;
            } else {
                self.tile_timer = self.min_time;
            }
            println!("{}", self.tile_timer);
            self.board.add_tile();
        }
        if self.board.is_full() {
            self.state = GameState::Lose;
            println!("You lose!");
        }
    }

    /// Called by the event loop when an `Input` event is recieved.
    fn input(&mut self, key: piston::input::Key) {
        match self.state {
            GameState::Ready => self.ready_key_press(key),
            GameState::Playing => self.playing_key_press(key),
            GameState::Lose => self.lose_key_press(key),
            _ => (),
        }
    }

    /// Called by `input` when the `GameState` is `Ready`.
    fn ready_key_press(&mut self, key: piston::input::Key) {
        if key == Key::Space {
            self.state = GameState::Playing;
        }
    }

    /// Called by `input` when the `GameState` is `Playing`.
    fn playing_key_press(&mut self, key: piston::input::Key) {
        self.handle_movement(key);
        self.whack(key);
    }

    /// Called by `input` when the `GameState` is `Lose`.
    fn lose_key_press(&mut self, key: piston::input::Key) {
        if key == Key::Space {
            self.reset();
            self.state = GameState::Ready;
        }
    }

    /// Handles movement input when the
    fn handle_movement(&mut self, key: piston::input::Key) {
        const MOVEMENT_KEYS: [piston::input::Key; 4] = [Key::Up, Key::Down, Key::Left, Key::Right];
        if MOVEMENT_KEYS.contains(&key) {
            let move_dist: f64 = self.board.length / 3.0;
            let move_vec = match key {
                Key::Up => {
                    gobs::Vec2D {
                        x: 0.0,
                        y: -move_dist,
                    }
                }
                Key::Down => {
                    gobs::Vec2D {
                        x: 0.0,
                        y: move_dist,
                    }
                }
                Key::Right => {
                    gobs::Vec2D {
                        x: move_dist,
                        y: 0.0,
                    }
                }
                Key::Left => {
                    gobs::Vec2D {
                        x: -move_dist,
                        y: 0.0,
                    }
                }
                _ => gobs::Vec2D { x: 0.0, y: 0.0 },
            };
            self.cursor.pos.add(move_vec);
        }
    }

    /// Checks if user has whacked a valid tile.
    fn whack(&mut self, key: piston::input::Key) {
        if key == Key::Space {
            let overlapping: Vec<usize> = self.board
                .tiles
                .iter()
                .map(|x| x.map_or(false, |y| y.is_overlapping(&self.cursor)))
                .enumerate()
                .filter(|x| x.1)
                .map(|x| x.0)
                .collect();
            if overlapping.len() > 0 {
                assert_eq!(overlapping.len(), 1);
                self.board.tiles[overlapping[0]].take();
                self.score += 1;
                println!("{:?}", self.score);
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
        GameManager::new(WINDOW_XY, 3.0, 1.0)
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
    fn reset_game() {
        let game1 = make_manager();
        let mut game2 = make_manager();
        assert!(game1 == game2);
        game2.cursor.pos.x = 50.0;
        game2.board.add_tile();
        game2.board.add_tile();
        game2.state = GameState::Lose;
        game2.score = 200;
        assert!(game1 != game2);
        game2.reset();
        assert!(game1 == game2);
    }
}