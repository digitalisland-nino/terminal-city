pub const TILE_SIZE: [i32; 2] = [4, (4.0 * 2.2) as i32];

pub struct Tile {
    pub y: i32,
    pub x: i32,
}

impl Tile {
    pub fn new(y: i32, x: i32) -> Self {
        Tile { y, x }
    }
}
