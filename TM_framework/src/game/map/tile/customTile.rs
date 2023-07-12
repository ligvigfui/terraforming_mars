use std::fmt::{Formatter, Debug};

use crate::*;

impl PartialEq for CustomTile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.picture == other.picture &&
        stringify!(self.where_can_place_function) == stringify!(other.where_can_place_function)
    }
}
impl Debug for CustomTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomTile")
            .field("name", &self.name)
            .field("picture", &self.picture)
            .field("function", &stringify!(&self.where_can_place_function))
            .finish()
    }
}
impl PlaceableTile for CustomTile {}
#[derive(Clone)]
pub struct CustomTile {
    pub name: String,
    pub picture: Picture,
    pub where_can_place_function: fn(map: &MarsTile, player_id: &u8, tile: &PlaceableTileType) -> Vec<(i32, i32)>,
}