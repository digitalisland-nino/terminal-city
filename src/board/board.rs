use super::grid::{Tile, TILE_SIZE};

pub struct Size {
    pub y: i32,
    pub x: i32,
}

pub struct Board {
    pub rows: i32,
    pub columns: i32,
    pub row_tiles: Vec<Tile>,
    pub column_tiles: Vec<Tile>,
}

impl Board {
    pub fn new(size: Size) -> Self {
        let rows = size.y;
        let columns = size.x;
        let mut row_tiles = Vec::new();
        let mut column_tiles = Vec::new();

        for _column in 0..size.y {
            column_tiles.push(Tile::new(TILE_SIZE[0], TILE_SIZE[1]))
        }

        for _row in 0..size.x {
            row_tiles.push(Tile::new(TILE_SIZE[0], TILE_SIZE[1]))
        }

        Board {
            rows,
            columns,
            row_tiles,
            column_tiles,
        }
    }
}
