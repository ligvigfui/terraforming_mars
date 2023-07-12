use crate::*;

impl Map<SpaceTile, SpaceTileType> for SpaceMap {
    fn new() -> Self {
        todo!()
    }

    fn add(self, x: i32, y: i32, tile: SpaceTile) -> Self {
        todo!()
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut SpaceTile> {
        todo!()
    }

    fn where_can_place_tile(&self, player_id: &u8, tile: &SpaceTileType) -> Vec<(i32, i32)> {
        todo!()
    }

    fn on_before_where_can_place_tile(&self, player_id: &u8, tile: &PlaceableTileType) ->
        (std::collections::HashMap<(i32, i32), usize>, Vec<MarsTile>) {
        todo!()
    }
}
pub struct SpaceMap {
    tiles: Vec<SpaceTile>,
}