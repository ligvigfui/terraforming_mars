use crate::*;

impl Tile for SpaceTile {}
pub struct SpaceTile {
    pub tile: SpaceTileType,
    pub reward: Vec<OnCardAction>,
}

impl PlaceableTile for SpaceTileType {}
pub enum SpaceTileType {
    City,
    Custom(CustomTile),
}