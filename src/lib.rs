extern crate rand;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
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

    #[derive(Debug, Copy, Clone)]
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

    #[derive(Debug)]
    pub struct Board {
        pub tiles: Tiles,
        pub length: f64,
    }

    impl Tile {
        pub fn new(x: f64, y: f64, wh: f64, colour: Colour) -> Tile {
            Tile {
                rect: graphics::rectangle::square(0.0, 0.0, wh),
                pos: Point { x: x, y: y },
                colour: colour,
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
    use gobs::{Board, Tiles, Tile};

    pub fn initialise_board(length: f64) -> Board {
        Board {
            tiles: [None; 9],
            length: length,
        }
    }

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
        rand::thread_rng().gen_range(0, 9)
    }

    fn x_from_index(i: usize, board_length: f64) -> f64 {
        let tile_length = (board_length / 3.0);
        println!("{:?} {:?}", i, (i as f64 % 3.0));

        ((i as f64 % 3.0) * tile_length) + (0.5 * tile_length)
    }

    fn y_from_index(i: usize, board_length: f64) -> f64 {
        let tile_length = (board_length / 3.0);

        ((i as f64 / 3.0).floor() * tile_length) + (0.5 * tile_length)
    }
}