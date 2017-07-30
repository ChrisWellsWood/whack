//! Contains code for various different game objects used in **Whack!**
extern crate graphics;
extern crate rand;

use rand::sample;
use colours::{Colour, RED};

/// Represents two-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    /// Returns a new `Vec2D` instance.
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x: x, y: y }
    }

    /// Returns an new `Vec2D` instance where `x` and `y` are equal to `0.0`.
    pub fn empty() -> Vec2D {
        Vec2D { x: 0.0, y: 0.0 }
    }

    /// Updates the fields of the `Vec2D` by pairwise addition of another instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use whack::gobs::Vec2D;
    ///
    /// let mut v1 = Vec2D::new(10.0, -13.2);
    /// let v2 = Vec2D::new(-57.2, -99.3);
    /// v1.add(v2);
    /// ```
    pub fn add(&mut self, other: Vec2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Represents a sprite that can be rendered.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sprite {
    pub pos: Vec2D,
    pub width: f64,
    pub height: f64,
    pub colour: Colour,
}

impl Sprite {
    /// Returns a tile struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use whack::colours;
    /// use whack::gobs::Sprite;
    ///
    /// let tile = Sprite::new(100.0, 100.0, 50.0, 50.0, colours::BLUE);
    /// ```
    pub fn new(x: f64, y: f64, width: f64, height: f64, colour: Colour) -> Sprite {
        Sprite {
            pos: Vec2D { x: x, y: y },
            width: width,
            height: height,
            colour: colour,
        }
    }

    /// Creates a rect type array from the `Sprite`.
    ///
    /// # Examples
    ///
    /// ```
    /// use whack::colours;
    /// use whack::gobs::Sprite;
    ///
    /// let tile = Sprite::new(100.0, 100.0, 50.0, 50.0, colours::GREEN);
    /// assert_eq!([tile.pos.x, tile.pos.y, tile.width, tile.height], tile.get_rect())
    pub fn get_rect(&self) -> [f64; 4] {
        [self.pos.x, self.pos.y, self.width, self.height]
    }

    /// Tests if the `Sprite` overlaps with a reference `Sprite`.
    ///
    /// # Examples
    ///
    /// ```
    /// use whack::gobs::Sprite;
    /// use whack::colours;
    ///
    /// let s1 = Sprite::new(100.0, 100.0, 50.0, 50.0, colours::YELLOW);
    /// let s2 = Sprite::new(125.0, 100.0, 50.0, 50.0, colours::YELLOW);
    /// let s3 = Sprite::new(155.0, 100.0, 50.0, 50.0, colours::YELLOW);
    /// assert!(s1.is_overlapping(&s2));
    /// assert!(!s1.is_overlapping(&s3));
    /// assert!(s2.is_overlapping(&s3));
    /// ```
    pub fn is_overlapping(&self, other: &Sprite) -> bool {
        if (self.pos.x + self.width < other.pos.x) || (other.pos.x + other.width < self.pos.x) ||
           (self.pos.y + self.height < other.pos.y) ||
           (other.pos.y + other.height < self.pos.y) {
            return false;
        }
        true
    }
}

/// Represents the game board.
#[derive(Debug, PartialEq)]
pub struct Board {
    pub tiles: Tiles,
    pub length: f64,
}

impl Board {
    /// Returns a Board struct with an empty Tiles array
    ///
    /// # Examples
    ///
    /// ```
    /// use whack::gobs::Board;
    ///
    /// let board = Board::from_length(300.0);
    /// ```
    pub fn from_length(length: f64) -> Board {
        Board {
            tiles: [None; 9],
            length: length,
        }
    }

    /// Returns a vector containing the indices of all the free positions on the `Board`.
    pub fn free_positions(&self) -> Vec<usize> {
        let positions: Vec<usize> = self.tiles
            .iter()
            .enumerate()
            .filter(|t| t.1.is_none())
            .map(|t| t.0)
            .collect();
        positions
    }

    /// True if there are no free positions on the `Board`.
    pub fn is_full(&self) -> bool {
        if self.free_positions().is_empty() {
            true
        } else {
            false
        }
    }

    /// Adds a tile to a random position on the `Board`.
    pub fn add_tile(&mut self) {
        let new_pos = self.random_position();
        if let Some(i) = new_pos {
            let new_tile = Sprite::new(self.x_from_index(i),
                                       self.y_from_index(i),
                                       self.length / 3.0,
                                       self.length / 3.0,
                                       RED);
            self.tiles[i] = Some(new_tile);
        }
    }

    /// Generates a random index if the `Board` is not full.
    fn random_position(&self) -> Option<usize> {
        let free_positions = self.free_positions();
        if free_positions.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let sample = sample(&mut rng, free_positions.into_iter(), 1);
        Some(sample[0])
    }

    /// Calculates the x coordinate of a position on the `Board` from its index.
    pub fn x_from_index(&self, i: usize) -> f64 {
        let tile_length = self.length / 3.0;
        ((i as f64 % 3.0) * tile_length)
    }

    /// Calculates the y coordinate of a position on the `Board` from its index.
    pub fn y_from_index(&self, i: usize) -> f64 {
        let tile_length = self.length / 3.0;
        ((i as f64 / 3.0).floor() * tile_length)
    }

    /// Removes all tiles from the `Board`.
    pub fn clear_board(&mut self) {
        self.tiles = [None; 9];
    }
}

/// Array that represents the tile positions of the game `Board`.
pub type Tiles = [Option<Sprite>; 9];

#[cfg(test)]
mod tests {
    use super::*;
    use colours;

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
    fn is_overlapping() {
        let window_size = 300.0;
        let mut board = Board::from_length(window_size);
        let mut cursor = Sprite::new(window_size / 2.0,
                                     window_size / 2.0,
                                     window_size / 16.0,
                                     window_size / 16.0,
                                     colours::YELLOW);
        for _ in 0..9 {
            board.add_tile();
        }
        let overlapping: Vec<bool> = board.tiles
            .iter()
            .map(|x| x.unwrap())
            .map(|x| cursor.is_overlapping(&x))
            .collect();
        assert_eq!(overlapping,
                   [false, false, false, false, true, false, false, false, false]);
        cursor.pos.x -= 100.0;
        let overlapping: Vec<bool> = board.tiles
            .iter()
            .map(|x| x.unwrap())
            .map(|x| cursor.is_overlapping(&x))
            .collect();
        assert_eq!(overlapping,
                   [false, false, false, true, false, false, false, false, false]);
        cursor.pos.y -= 100.0;
        let overlapping: Vec<bool> = board.tiles
            .iter()
            .map(|x| x.unwrap())
            .map(|x| cursor.is_overlapping(&x))
            .collect();
        assert_eq!(overlapping,
                   [true, false, false, false, false, false, false, false, false]);
    }

    #[test]
    fn move_cursor() {
        let window_size = 300.0;
        let mut cursor = Sprite::new(window_size / 2.0,
                                     window_size / 2.0,
                                     window_size / 16.0,
                                     window_size / 16.0,
                                     colours::YELLOW);
        cursor.pos.add(Vec2D {
            x: -100.0,
            y: 0.0,
        });
        assert_eq!(cursor.pos.x, 50.0);
        assert_eq!(cursor.pos.y, 150.0);
        cursor.pos.add(Vec2D {
            x: 100.0,
            y: 100.0,
        });
        assert_eq!(cursor.pos.x, 150.0);
        assert_eq!(cursor.pos.y, 250.0);
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
        assert_eq!(board.x_from_index(0), 0.0);
        assert_eq!(board.x_from_index(1), 100.0);
        assert_eq!(board.x_from_index(2), 200.0);
        assert_eq!(board.x_from_index(8), 200.0);
    }

    #[test]
    fn check_y_from_i() {
        let board = Board::from_length(300.0);
        assert_eq!(board.y_from_index(0), 0.0);
        assert_eq!(board.y_from_index(1), 0.0);
        assert_eq!(board.y_from_index(2), 0.0);
        assert_eq!(board.y_from_index(8), 200.0);
    }
}