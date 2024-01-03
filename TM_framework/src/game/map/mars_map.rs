use std::{collections::HashMap, vec};

use crate::*;

pub mod base_map;
pub mod hellas_map;
pub mod elysium_map;

/// # Map layout
///         (-1,1)    (0,1)
/// 
///     (-1,0)    (0,0)     (1,0)
/// 
///         (0,-1)    (1,-1)
pub struct MarsMap {
    pub has_vulcanos: bool,
    pub has_noctis: bool,
    pub map: HashMap<(i32, i32, u8), MarsTile>,
}

impl MapType<MarsTile, PlaceableTile> for MarsMap {
    fn new(has_vulcanos: bool, has_noctis: bool) -> Self {
        MarsMap {
            has_vulcanos,
            has_noctis,
            map: HashMap::new(),
        }
    }
    
    fn add(mut self, x: i32, y: i32, tile: MarsTile) -> Self {
        self.map.insert((x, y, 0), tile);
        self
    }

    fn where_can_place_tile(&self, player_id: &u8, tile: &PlaceableTile) -> Vec<(i32, i32, u8)> {
        match tile {
            PlaceableTile::Ocean => {
                self.map.iter().filter_map(|(&(x, y, _), tile)| {
                    if tile.kind == MarsTileKind::Natural(NaturalTile::ReservedOcean) && !self.map.contains_key(&(x, y, 1)) {
                        Some((x, y, 1))
                    } else {
                        None
                    }
                }).collect()
            }
            PlaceableTile::City => {
                let free_lands: Vec<(i32, i32, u8)> = self.map.iter().filter_map(|(&(x, y, _), tile)| {
                    match &tile.kind {
                        MarsTileKind::Natural(NaturalTile::Land) |
                        MarsTileKind::Natural(NaturalTile::Vulcano(_)) => {
                            if self.map.contains_key(&(x, y, 1)) {
                                None
                            } else {
                                Some((x, y, 1))
                            }
                        }
                        _ => None,
                    }
                }).collect();
                let city_nearby: Vec<(i32, i32, u8)> = self.map.iter().filter_map(|(&(x, y, z), tile)| {
                    match &tile.kind {
                        MarsTileKind::Occupied { tile: PlaceableTile::City, player_id: _ } => {
                            for tile in next_to((x, y)) {
                                if self.map.contains_key(&(tile.0, tile.1, z)) {
                                    return Some((x, y, z));
                                }
                            }
                            None
                        }
                        _ => None,
                    }
                }).collect();
                free_lands.into_iter().filter(|land| !city_nearby.contains(land)).collect()
            }
            PlaceableTile::Greenery => {
                let free_lands: Vec<(i32, i32, u8)> = self.map.iter().filter_map(|(&(x, y, _), tile)| {
                    match &tile.kind {
                        MarsTileKind::Natural(NaturalTile::Land) |
                        MarsTileKind::Natural(NaturalTile::Vulcano(_)) =>
                            match !self.map.contains_key(&(x, y, 1)) {
                                true => Some((x, y, 1)),
                                false => None,
                            }
                        _ => None,
                    }
                }).collect();
                let players_tiles: Vec<(i32, i32, u8)> = self.map.iter().filter_map(|(&(x, y, _), tile)| {
                    match &tile.kind {
                        MarsTileKind::Occupied { tile: _, player_id: player_id_inner } => {
                            if player_id_inner == player_id {
                                Some((x, y, 1))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }).collect();
                if players_tiles.is_empty() {
                    return free_lands;
                } else {
                    let mut result: Vec<(i32, i32, u8)> = Vec::new();
                    for players_tile in players_tiles {
                        for tile in next_to((players_tile.0, players_tile.1)) {
                            if free_lands.contains(&(tile.0, tile.1, 1 as u8)) {
                                result.push((tile.0, tile.1, 1 as u8));
                            }
                        }
                    }
                    return result;
                }
            }
            PlaceableTile::Custom(tile) => {
                let mut vectors = Vec::new();
                for function in &tile.get_placement_locations {
                    vectors.push(function(self, player_id));
                }
                let mut result = vectors.pop().unwrap();
                for vector in vectors {
                    // get the intersection of all vectors
                    result = result.into_iter().filter(|point| vector.contains(point)).collect();
                }
                result
            }
        }
    }
}

impl MarsMap {
    fn free_places(&self, layer: u8, allowed_tiles_below: Vec<MarsTile>) -> Vec<(i32, i32, u8)> {
        self.map.iter().filter_map(|(&(x, y, z), tile)| {
            if z == layer - 1 && allowed_tiles_below.contains(tile) && !self.map.contains_key(&(x, y, layer)) {
                Some((x, y, layer))
            } else {
                None
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mars_base_map() {
        let map = MarsMap::base_map();
        let point = map.map.get(&(0, 0, 0)).unwrap();
        let tile = MarsTile { kind: MarsTileKind::Natural(NaturalTile::ReservedOcean),
            reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] };
        assert_eq!(to_string(point), to_string(tile));
        let point = map.map.get(&(3, 1, 0)).unwrap();
        let tile = MarsTile { kind: MarsTileKind::Natural(NaturalTile::ReservedOcean),
            reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] };
        assert_eq!(to_string(point), to_string(tile));
    }
}
