use crate::*;

pub mod marsMap;


//     (-1,1)    (0,1)
//(-1,0)    (0,0)     (1,0)
//     (0,-1)    (1,-1)

pub trait Map {
    fn new() -> Self;
    fn can_place_tile(&self, player: &Player, tile: &dyn Tile, x: i32, y: i32) -> Result<(), String>;
    fn add(self, x: i32, y: i32, tile: dyn Tile) -> Self;
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut dyn Tile>;
}

pub trait Tile {}



