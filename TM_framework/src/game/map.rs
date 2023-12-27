use crate::*;

pub mod tile;
pub mod mars_map;


//     (-1,1)    (0,1)
//(-1,0)    (0,0)     (1,0)
//     (0,-1)    (1,-1)

pub trait MapType<T: Tile, PT: Placeable> {
    fn new(has_vulcanos: bool, has_noctis: bool) -> Self;
    fn add(self, x: i32, y: i32, tile: T) -> Self;
    fn where_can_place_tile(&self, player_id: &u8, tile: &PT) -> Vec<(i32, i32, u8)>;
}

#[derive(Debug, Clone)]
pub enum Map {
    Base,
    Hellas,
    Elysium,
    Custom(String),
}