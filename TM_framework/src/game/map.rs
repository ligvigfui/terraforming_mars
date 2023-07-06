use crate::*;

use self::tile::Tile;

pub mod marsMap;
pub mod tile;


//     (-1,1)    (0,1)
//(-1,0)    (0,0)     (1,0)
//     (0,-1)    (1,-1)

pub trait Map<T: Tile> {
    fn new() -> Self;
    fn can_place_tile(&self, player: &Player, tile: &T, x: i32, y: i32) -> Result<(), String>;
    fn add(self, x: i32, y: i32, tile: T) -> Self;
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T>;
}