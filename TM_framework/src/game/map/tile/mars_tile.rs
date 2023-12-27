use crate::*;

impl Tile for MarsTile {}
#[derive(Debug, Clone)]
pub struct MarsTile {
    pub kind: MarsTileKind,
    pub reward: Vec<OnCardAction>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MarsTileKind {
    Natural(NaturalTile),
    Occupied { tile: PlaceableTile, player_id: u8 },
}

#[derive(Debug, PartialEq, Clone)]
pub enum NaturalTile {
    Land,
    Vulcano(String),
    ReservedOcean,
    NoctisCityReserved,
}

impl Placeable for PlaceableTile {}
#[derive(Debug, PartialEq, Clone)]
pub enum PlaceableTile {
    Ocean,
    City,
    Greenery,
    Custom(CustomTile),
}