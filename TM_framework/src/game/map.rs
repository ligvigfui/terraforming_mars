use crate::*;

use self::tile::*;

pub mod marsMap;
pub mod baseMap;
pub mod hellasMap;
pub mod elysiumMap;
pub mod tile;


//     (-1,1)    (0,1)
//(-1,0)    (0,0)     (1,0)
//     (0,-1)    (1,-1)

pub trait Map<T: Tile, Ti: PlaceableTile> {
    fn new() -> Self;
    fn add(self, x: i32, y: i32, tile: T) -> Self;
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T>;
    fn where_can_place_tile(&self, player: &Player, tile: &Ti) -> Vec<(i32, i32)>;
}