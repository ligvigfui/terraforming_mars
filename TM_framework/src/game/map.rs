use OnCardAction as OCA;
use std::vec;

use crate::*;

//     (-1,1)    (0,1)
//(-1,0)    (0,0)     (1,0)
//     (0,-1)    (1,-1)

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: Vec::new(),
        }
    }

    pub fn base_board() -> Self {
        Map { tiles: vec![
            Tile { x: 0, y: 0, tile: TileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] },
        ]}
    }
}


#[derive(Debug)]
pub struct Tile {
    x: i32,
    y: i32,
    tile: TileType,
    reward: Vec<OnCardAction>,
}

#[derive(Debug)]
pub enum TileType {
    Land,
    Ocean,
    Mountin(String),
    ReservedOcean,
    NoctisCityReserved,
    Occupied(OccupiedTile),
}

#[derive(Debug)]
pub struct OccupiedTile {
    tile: OccupiedTileType,
    owner_id: u8,
}

#[derive(Debug)]
pub enum OccupiedTileType {
    City,
    Greenery,
    Special(SpecialTile),
}

#[derive(Debug)]
pub enum SpecialTile {
    NaturalPreserve,
    ExcavationSite,
    CommercialDistrict,
    NuclearZone,
    IndustrialCenter,
    EcologicalZone,
    LavaFlows,
    MoholeArea,
    RestrictedArea,
    Capital,
    DeimosDown,
    GreatDam,
    MagneticFieldGenerator,
}

