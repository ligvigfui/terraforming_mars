use std::fmt::{Formatter, Debug};

use crate::*;

impl PartialEq for CustomTile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.picture == other.picture &&
        format!("{:?}",self.get_placement_locations) == format!("{:?}",other.get_placement_locations)
    }
}

impl Debug for CustomTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let placement_locations = format!("{:?}",self.get_placement_locations);
        f.debug_struct("CustomTile")
            .field("name", &self.name)
            .field("picture", &self.picture)
            .field("function", &placement_locations)
            .finish()
    }
}

impl Placeable for CustomTile {}
#[derive(Clone)]
pub struct CustomTile {
    pub name: String,
    pub picture: Picture,
    pub get_placement_locations: Vec<fn(map: &MarsMap, player_id: &u8) -> Vec<(i32, i32, u8)>>,
}