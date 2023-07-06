use crate::OnCardAction;

use super::Tile;


pub trait TMarsTile: Tile {
    
}

#[derive(Debug)]
pub struct MarsTile {
    pub(crate) tile: MarsTileType,
    pub(crate) reward: Vec<OnCardAction>,
}

impl Tile for MarsTile {}

#[derive(Debug)]
pub enum MarsTileType {
    Land,
    Vulcano(String),
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
    Ocean,
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