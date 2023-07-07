use std::{collections::HashMap, vec};

use crate::{*, tile::{marsTile::*}};


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

    fn filter_tiles(&self, predicate: impl Fn(&MarsTile) -> bool) -> Vec<(i32, i32)> {
        let mut result = vec![];
        for ((x, y), tile_id) in self.coordinate_map.iter() {
            if predicate(&self.tiles[*tile_id]) {
                result.push((*x, *y));
            }
        }
        result
    }
}

impl Map<MarsTile, PlaceableTileType> for MarsMap {
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

    fn where_can_place_tile(&self, player: &Player, tile: &PlaceableTileType) -> Vec<(i32, i32)> {
        let mut places: Vec<(i32, i32)> =  Vec::new();
        match tile {
            PlaceableTileType::Ocean => {
                for ((x, y), tile_id) in self.coordinate_map.iter() {
                    if self.tiles[*tile_id].tile == MarsTileType::ReservedOcean {
                        places.push((*x, *y));
                    }
                }
            }
            PlaceableTileType::City => {
                self.coordinate_map.iter().filter(predicate)
            }
            PlaceableTileType::Greenery => {}
            PlaceableTileType::Special(_) => {}
        }
        todo!();
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::marsTile::MarsTile;

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

    #[test]
    fn test_filter_tiles() {
        let map = MarsMap::base_map();
        let mut tiles = map.filter_tiles(|tile| tile.tile == MarsTileType::ReservedOcean);
        assert_eq!(tiles.len(), 12);
        assert!(tiles.contains(&(-3, 4)));
        tiles = map.filter_tiles(|tile| tile.tile == MarsTileType::NoctisCityReserved);
        assert!(tiles.contains(&(-2, 0)));
        tiles = map.filter_tiles(|tile| tile.reward == vec![OnCardAction::ModifyResources(Resource::Titanium(1)), OnCardAction::ModifyResources(Resource::Plant(1))]);
        assert!(tiles.contains(&(-4, 1)));
        tiles = map.filter_tiles(|tile| tile.tile == MarsTileType::Occupied(OccupiedTile{ tile: PlaceableTileType::City, owner_id: 0}));

    }
}