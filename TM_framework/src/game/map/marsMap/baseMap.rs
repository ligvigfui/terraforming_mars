use super::*;

use MarsTileType::Land as Land;
use MarsTileType::ReservedOcean as ReservedOcean;
use MarsTileType::Vulcano as Vulcano;

use OnCardAction::ModifyResources as ModifyResources;
use OnCardAction::DrawCard as DrawCard;

use Resource::Steel as Steel;
use Resource::Titanium as Titanium;
use Resource::Plant as Plant;

impl MarsMap {
    pub fn base_map() -> Self {
        MarsMap::new().set_noctis(true)
            //row 1
            .add(-4, 4, MarsTile { tile: Land, reward: vec![ModifyResources(Steel(2))] })
            .add(-3, 4, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Steel(2))] })
            .add(-2, 4, MarsTile { tile: Land, reward: vec![] })
            .add(-1, 4, MarsTile { tile: ReservedOcean, reward: vec![DrawCard(1)] })
            .add(0, 4, MarsTile { tile: ReservedOcean, reward: vec![] })
            //row 2
            .add(-4, 3, MarsTile { tile: Land, reward: vec![] })
            .add(-3, 3, MarsTile { tile: Vulcano("Tharsis Tholus".to_string()), reward: vec![ModifyResources(Steel(1))] })
            .add(-2, 3, MarsTile { tile: Land, reward: vec![] })
            .add(-1, 3, MarsTile { tile: Land, reward: vec![] })
            .add(0, 3, MarsTile { tile: Land, reward: vec![] })
            .add(1, 3, MarsTile { tile: ReservedOcean, reward: vec![DrawCard(2)] })
            //row 3
            .add(-4, 2, MarsTile { tile: Vulcano("Ascraeus Mons".to_string()), reward: vec![DrawCard(1)] })
            .add(-3, 2, MarsTile { tile: Land, reward: vec![] })
            .add(-2, 2, MarsTile { tile: Land, reward: vec![] })
            .add(-1, 2, MarsTile { tile: Land, reward: vec![] })
            .add(0, 2, MarsTile { tile: Land, reward: vec![] })
            .add(1, 2, MarsTile { tile: Land, reward: vec![] })
            .add(2, 2, MarsTile { tile: Land, reward: vec![ModifyResources(Steel(1))] })
            //row 4
            .add(-4, 1, MarsTile { tile: Vulcano("Pavonis Mons".to_string()), 
                reward: vec![ModifyResources(Plant(1)), ModifyResources(Titanium(1))] })
            .add(-3, 1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(-2, 1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(-1, 1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(0, 1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(2))] })
            .add(1, 1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(2, 1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(3, 1, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Plant(2))] })
            //row 5
            .add(-4, 0, MarsTile { tile: Vulcano("Arsia Mons".to_string()), reward: vec![ModifyResources(Plant(2))] })
            .add(-3, 0, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(2))] })
            .add(-2, 0, MarsTile { tile: MarsTileType::NoctisCityReserved, reward: vec![ModifyResources(Plant(2))] })
            .add(-1, 0, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Plant(2))] })
            .add(0, 0, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Plant(2))] })
            .add(1, 0, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Plant(2))] })
            .add(2, 0, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(2))] })
            .add(3, 0, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(2))] })
            .add(4, 0, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(2))] })
            //row 6
            .add(-3, -1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(-2, -1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(2))] })
            .add(-1, -1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(0, -1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(1, -1, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(2, -1, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Plant(1))] })
            .add(3, -1, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Plant(1))] })
            .add(4, -1, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Plant(1))] })
            //row 7
            .add(-2, -2, MarsTile { tile: Land, reward: vec![] })
            .add(-1, -2, MarsTile { tile: Land, reward: vec![] })
            .add(0, -2, MarsTile { tile: Land, reward: vec![] })
            .add(1, -2, MarsTile { tile: Land, reward: vec![] })
            .add(2, -2, MarsTile { tile: Land, reward: vec![] })
            .add(3, -2, MarsTile { tile: Land, reward: vec![ModifyResources(Plant(1))] })
            .add(4, -2, MarsTile { tile: Land, reward: vec![] })
            //row 8
            .add(-1, -3, MarsTile { tile: Land, reward: vec![ModifyResources(Steel(2))] })
            .add(0, -3, MarsTile { tile: Land, reward: vec![] })
            .add(1, -3, MarsTile { tile: Land, reward: vec![DrawCard(1)] })
            .add(2, -3, MarsTile { tile: Land, reward: vec![DrawCard(1)] })
            .add(3, -3, MarsTile { tile: Land, reward: vec![] })
            .add(4, -3, MarsTile { tile: Land, reward: vec![ModifyResources(Titanium(1))] })
            //row 9
            .add(0, -4, MarsTile { tile: Land, reward: vec![ModifyResources(Steel(1))] })
            .add(1, -4, MarsTile { tile: Land, reward: vec![ModifyResources(Steel(2))] })
            .add(2, -4, MarsTile { tile: Land, reward: vec![] })
            .add(3, -4, MarsTile { tile: Land, reward: vec![] })
            .add(4, -4, MarsTile { tile: ReservedOcean, reward: vec![ModifyResources(Titanium(2))] })
        }
}
