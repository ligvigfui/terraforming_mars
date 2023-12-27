use std::{collections::{HashMap, HashSet}, vec};

use crate::*;

pub mod base_map;
pub mod hellas_map;
pub mod elysium_map;

struct MarsMap {
    has_vulcanos: bool,
    has_noctis: bool,
    tiles: Vec<MarsTile>,
    coordinate_map: HashMap<(i32, i32), usize>,
}

impl MarsMap {
    fn set_vulcanos(mut self, has_vulcanos: bool) -> Self {
        self.has_vulcanos = has_vulcanos;
        self
    }

    fn set_noctis(mut self, has_noctis: bool) -> Self {
        self.has_noctis = has_noctis;
        self
    }
}

impl MapType<MarsTile, PlaceableTileType> for MarsMap {
    fn new() -> Self {
        MarsMap {
            has_vulcanos: true,
            has_noctis: false,
            tiles: Vec::new(),
            coordinate_map: HashMap::new(),
        }
    }
    fn add(mut self, x: i32, y: i32, tile: MarsTile) -> Self {
        self.coordinate_map.insert((x,y), self.tiles.len());
        self.tiles.push(tile);
        self
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut MarsTile> {
        let usize1 = self.coordinate_map.get(&(x,y))?;
        self.tiles.get_mut(*usize1)
    }

    fn where_can_place_tile(&self, player_id: &u8, tile: &PlaceableTileType) -> Vec<(i32, i32)> {
        let mut places: Vec<(i32, i32)> =  Vec::new();
        let (filtered_coordinate_map, filtered_tiles) = 
            self.on_before_where_can_place_tile(player_id, tile);

        match tile {
            PlaceableTileType::Ocean => {
                for ((x, y), tile_id) in filtered_coordinate_map.iter() {
                    if filtered_tiles[*tile_id].tile == MarsTileType::ReservedOcean {
                        places.push((*x, *y));
                    }
                }
            }
            PlaceableTileType::City => {
                let mut hashset = HashSet::new();
                filtered_coordinate_map.iter().filter(|(_, &tile_id)|
                    match filtered_tiles[tile_id].tile {
                        MarsTileType::Land => true,
                        MarsTileType::Vulcano(_) => true,
                        _ => false
                }).for_each(|(&(x, y), _)| { hashset.insert((x, y)); });

                filtered_coordinate_map.iter().filter(|(_, &tile_id)|
                    match filtered_tiles[tile_id].tile {
                        //? is this correct?
                        MarsTileType::Occupied(OccupiedTile { tile: PlaceableTileType::City, owner_id: _, }) => true,
                        _ => false
                }).for_each(|(&city_tile_coord, _)| 
                    for nearby_city_tiles in next_to(city_tile_coord).iter(){
                        hashset.remove(nearby_city_tiles);
                });
                places = hashset.into_iter().collect();
            }
            PlaceableTileType::Greenery => {
                // gets tiles next to player placed cards
                for (&(x, y), &tile_id) in filtered_coordinate_map.iter() {
                    match &filtered_tiles[tile_id].tile {
                        MarsTileType::Land | MarsTileType::Vulcano(_) => {
                            for neighbour_tile in next_to((x,y)){
                                match &filtered_tiles[filtered_coordinate_map[&neighbour_tile]].tile {
                                    MarsTileType::Occupied(OccupiedTile { tile: _, owner_id }) 
                                        if owner_id == player_id => {places.push((x,y)); break;},
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                // reurns all empty places if places is empty.
                if places.len() == 0 {
                    return filtered_coordinate_map.iter().filter(|(_, &tile_id)|
                        match filtered_tiles[tile_id].tile {
                            MarsTileType::Land => true,
                            MarsTileType::Vulcano(_) => true,
                            _ => false
                    }).map(|(&(x, y), _)| (x, y)).collect();
                }
            }
            _ => {}
        }
        todo!();
        vec![]
    }

    fn on_before_where_can_place_tile(&self, player_id: &u8, tile: &PlaceableTileType) -> (HashMap<(i32, i32), usize>, Vec<MarsTile>) {
        
        todo!("remove standard return");
        (self.coordinate_map.clone(), self.tiles.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_modify_vulcanos() {
        let map = MarsMap::new().set_vulcanos(false);
        assert!(!map.has_vulcanos)
    }

    #[test]
    fn test_mars_base_map() {
        let mut map = MarsMap::base_map();
        let point = map.get_mut(0, 0).unwrap();
        let tile = MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] };
        assert_eq!(to_string(point), to_string(tile));
        let point = map.get_mut(3, 1).unwrap();
        let tile = MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] };
        assert_eq!(to_string(point), to_string(tile));
        
    }
}
