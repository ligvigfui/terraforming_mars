use std::collections::HashMap;

use crate::*;


pub mod tile;
pub mod marsMap;
pub mod spaceMap;


//     (-1,1)    (0,1)
//(-1,0)    (0,0)     (1,0)
//     (0,-1)    (1,-1)

pub trait Map<T: Tile, PT: PlaceableTile> {
    fn new() -> Self;
    fn add(self, x: i32, y: i32, tile: T) -> Self;
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T>;
    fn where_can_place_tile(&self, player_id: &u8, tile: &PT) -> Vec<(i32, i32)>;
    fn on_before_where_can_place_tile(&self, player_id: &u8, tile: &PlaceableTileType) -> (HashMap<(i32, i32), usize>, Vec<MarsTile>);
}

pub enum Maps {
    Base,
    Hellas,
    Elysium,
    Custom(String),
}