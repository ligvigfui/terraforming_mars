use std::slice::Iter;

use crate::*;


pub trait TMarsTile: Tile {
    
}

#[derive(Debug, Clone)]
pub struct MarsTile {
    pub(crate) tile: MarsTileType,
    pub(crate) reward: Vec<OnCardAction>,
}

impl Tile for MarsTile {}

#[derive(Debug, PartialEq, Clone)]
pub enum MarsTileType {
    Land,
    Vulcano(String),
    ReservedOcean,
    NoctisCityReserved,
    Occupied(OccupiedTile),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OccupiedTile {
    pub(crate) tile: PlaceableTileType,
    pub(crate) owner_id: u8,
}

impl OccupiedTile {
    pub fn new() -> Self {
        Self {
            tile: PlaceableTileType::Ocean,
            owner_id: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlaceableTileType {
    Ocean,
    City,
    Greenery,
    Special(SpecialTile),
}

impl PlaceableTile for PlaceableTileType {}

#[derive(Debug, PartialEq, Clone)]
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