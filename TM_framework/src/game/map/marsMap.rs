use std::collections::HashMap;

use crate::*;



#[derive(Debug)]
struct MarsMap {
    has_vulcanos: bool,
    has_noctis: bool,
    tiles: Vec<MarsTile>,
    coordinate_map: HashMap<(i32, i32), usize>,
}

impl MarsMap {
    fn vulcanos(mut self, has_vulcanos: bool) -> Self {
        self.has_vulcanos = has_vulcanos;
        self
    }

    fn noctis(mut self, has_noctis: bool) -> Self {
        self.has_noctis = has_noctis;
        self
    }

    pub fn base_map() -> Self {
        MarsMap::new().noctis(true)
            //row 1
            .add(-4, 4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(-3, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(-2, 4, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-1, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::DrawCard(1)] })
            .add(0, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![] })
            //row 2
            .add(-4, 3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-3, 3, MarsTile { tile: MarsTileType::Vulcano("Tharsis Tholus".to_string()), reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(-2, 3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-1, 3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(0, 3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, 3, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::DrawCard(2)] })
            //row 3
            .add(-4, 2, MarsTile { tile: MarsTileType::Vulcano("Ascraeus Mons".to_string()), reward: vec![OnCardAction::DrawCard(1)] })
            .add(-3, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-2, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-1, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(0, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(2, 2, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            //row 4
            .add(-4, 1, MarsTile { tile: MarsTileType::Vulcano("Pavonis Mons".to_string()), 
                reward: vec![OnCardAction::ModifyResources(Resource::Plant(1)), OnCardAction::ModifyResources(Resource::Titanium(1))] })
            .add(-3, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-2, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-1, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(0, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(1, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(2, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(3, 1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            //row 5
            .add(-4, 0, MarsTile { tile: MarsTileType::Vulcano("Arsia Mons".to_string()), reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-3, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-2, 0, MarsTile { tile: MarsTileType::NoctisCityReserved, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-1, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(0, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(1, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(2, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(3, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(4, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            //row 6
            .add(-3, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-2, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-1, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(0, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(1, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(2, -1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(3, -1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(4, -1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            //row 7
            .add(-2, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-1, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(0, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(2, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(3, -2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(4, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            //row 8
            .add(-1, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(0, -3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(2, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(3, -3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(4, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(1))] })
            //row 9
            .add(0, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(1, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(2, -4, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(3, -4, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(4, -4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(2))] })
        }
    pub fn hellas_map() -> Self {
        MarsMap::new().vulcanos(false)
            //row 1
            .add(-4, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-3, 4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-2, 4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-1, 4, MarsTile { tile: MarsTileType::Land, 
                reward: vec![OnCardAction::ModifyResources(Resource::Plant(1)), OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(0, 4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            //row 2
            .add(-4, 3, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-3, 3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-2, 3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-1, 3, MarsTile { tile: MarsTileType::Land, 
                reward: vec![OnCardAction::ModifyResources(Resource::Plant(1)), OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(0, 3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(1, 3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            //row 3
            .add(-4, 2, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-3, 2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-2, 2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(-1, 2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(0, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, 2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(2, 2, MarsTile { tile: MarsTileType::Land, 
                reward: vec![OnCardAction::ModifyResources(Resource::Plant(1)), OnCardAction::DrawCard(1)] })
            //row 4
            .add(-4, 1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-3, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-2, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(-1, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(0, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(1, 1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(2, 1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(3, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            //row 5
            .add(-4, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(-3, 0, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-2, 0, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-1, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(0, 0, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::DrawCard(1)] })
            .add(2, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Heat(3))] })
            .add(3, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![] })
            .add(4, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            //row 6
            .add(-3, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(1))] })
            .add(-2, -1, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-1, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(0, -1, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, -1, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(2, -1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![] })
            .add(3, -1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(4, -1, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            //row 7
            .add(-2, -2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(2))] })
            .add(-1, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(0, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, -2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(2, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(3, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(4, -2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(1))] })
            //row 8
            .add(-1, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(0, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(1, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Heat(2))] })
            .add(2, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Heat(2))] })
            .add(3, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(1))] })
            .add(4, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(1))] })
            //row 9
            .add(0, -4, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Heat(2))] })
            .add(2, -4, MarsTile { tile: MarsTileType::Land, reward: vec![
                OnCardAction::ModifyResources(Resource::Money(-5)), OnCardAction::PlaceTile(OccupiedTileType::Ocean)] })
            .add(3, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Heat(2))] })
            .add(4, -4, MarsTile { tile: MarsTileType::Land, reward: vec![] })
    }
    pub fn elisium_map() -> Self {
        MarsMap::new()
            .add(-4, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![] })
            .add(-3, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(1))] })
            .add(-2, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::DrawCard(1)] })
            .add(-1, 4, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(0, 4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            //row 2
            .add(-4, 3, MarsTile { tile: MarsTileType::Vulcano("Arsia Mons".to_string()), 
                reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(-3, 3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-2, 3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-1, 3, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![] })
            .add(0, 3, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![] })
            .add(1, 3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            //row 3
            .add(-4, 2, MarsTile { tile: MarsTileType::Vulcano("Elysium Mons".to_string()), 
                reward: vec![OnCardAction::ModifyResources(Resource::Titanium(2))] })
            .add(-3, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(-2, 2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(-1, 2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(0, 2, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(1, 2, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![] })
            .add(2, 2, MarsTile { tile: MarsTileType::Vulcano("Olympus Mons".to_string()), reward: vec![OnCardAction::DrawCard(3)] })
            //row 4
            .add(-4, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-3, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-2, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-1, 1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(0, 1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(1, 1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(2, 1, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(3, 1, MarsTile { tile: MarsTileType::Land, 
                reward: vec![OnCardAction::ModifyResources(Resource::Plant(1)), OnCardAction::ModifyResources(Resource::Steel(1))] })
            //row 5
            .add(-4, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-3, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-2, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(-1, 0, MarsTile { tile: MarsTileType::ReservedOcean, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(0, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(1, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(3))] })
            .add(2, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(3, 0, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))] })
            .add(4, 0, MarsTile { tile: MarsTileType::Vulcano("Arsia Mons".to_string()), 
                reward: vec![OnCardAction::ModifyResources(Resource::Plant(1)), OnCardAction::ModifyResources(Resource::Titanium(1))] })
            //row 6
            .add(-3, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(-2, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(-1, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(0, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(1, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(2, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(3, -1, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Plant(1))] })
            .add(4, -1, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            //row 7
            .add(-2, -2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Titanium(1))] })
            .add(-1, -2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(0, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(2, -2, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(3, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(4, -2, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            //row 8
            .add(-1, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(0, -3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(1, -3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(2, -3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(3, -3, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
            .add(4, -3, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            //row 9
            .add(0, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(1))] })
            .add(1, -4, MarsTile { tile: MarsTileType::Land, reward: vec![] })
            .add(2, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(3, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::DrawCard(1)] })
            .add(4, -4, MarsTile { tile: MarsTileType::Land, reward: vec![OnCardAction::ModifyResources(Resource::Steel(2))] })
    }
}


impl Map for MarsMap {
    fn new() -> Self {
        MarsMap {
            has_vulcanos: true,
            has_noctis: false,
            tiles: Vec::new(),
            coordinate_map: HashMap::new(),
        }
    }
    fn add(mut self, x: i32, y: i32, tile: dyn map::Tile) -> Self {
        self.coordinate_map.insert((x,y), self.tiles.len());
        self.tiles.push(tile);
        self
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut dyn map::Tile> {
        let usize1 = self.coordinate_map.get(&(x,y))?;
        self.tiles.get_mut(*usize1)
    }

    fn can_place_tile(&self, player: &Player, tile: &dyn map::Tile, x: i32, y: i32) -> Result<(), String> {
        
        
        todo!();
        Ok(())
    }
}

#[derive(Debug)]
pub struct MarsTile {
    tile: MarsTileType,
    reward: Vec<OnCardAction>,
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

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_modify_vulcanos() {
        let map = MarsMap::new().vulcanos(false);
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