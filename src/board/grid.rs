pub const TILE_SIZE: [i32; 2] = [4, (4.0 * 2.2) as i32];
pub const SUB_TILE_SIZE: [i32; 2] = [2, (2.0 * 2.2) as i32];

pub struct Tile {
    pub y: i32,
    pub x: i32,
    pub position_y: i32,
    pub position_x: i32,
}

impl Tile {
    pub fn new(y: i32, x: i32, position_y: i32, position_x: i32) -> Self {
        Tile {
            y,
            x,
            position_y,
            position_x,
        }
    }
}
