use super::*;

use MarsTileKind::Natural as Natural;
use NaturalTile::Land as Land;
use NaturalTile::Vulcano as Vulcano;
use NaturalTile::ReservedOcean as ReservedOcean;

use OnCardAction::ModifyResources as ModifyResources;
use OnCardAction::DrawCard as DrawCard;

use Resource::Steel as Steel;
use Resource::Titanium as Titanium;
use Resource::Plant as Plant;

impl MarsMap {
    pub fn base_map() -> Self {
        MarsMap::new(true, true)
            //row 1
            .add(-4, 4, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Steel(2))] })
            .add(-3, 4, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Steel(2))] })
            .add(-2, 4, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(-1, 4, MarsTile { kind: Natural(ReservedOcean), reward: vec![DrawCard(1)] })
            .add(0, 4, MarsTile { kind: Natural(ReservedOcean), reward: vec![] })
            //row 2
            .add(-4, 3, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(-3, 3, MarsTile { kind: Natural(Vulcano("Tharsis Tholus".to_string())), reward: vec![ModifyResources(Steel(1))] })
            .add(-2, 3, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(-1, 3, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(0, 3, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(1, 3, MarsTile { kind: Natural(ReservedOcean), reward: vec![DrawCard(2)] })
            //row 3
            .add(-4, 2, MarsTile { kind: Natural(Vulcano("Ascraeus Mons".to_string())), reward: vec![DrawCard(1)] })
            .add(-3, 2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(-2, 2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(-1, 2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(0, 2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(1, 2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(2, 2, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Steel(1))] })
            //row 4
            .add(-4, 1, MarsTile { kind: Natural(Vulcano("Pavonis Mons".to_string())), 
                reward: vec![ModifyResources(Plant(1)), ModifyResources(Titanium(1))] })
            .add(-3, 1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(-2, 1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(-1, 1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(0, 1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(2))] })
            .add(1, 1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(2, 1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(3, 1, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Plant(2))] })
            //row 5
            .add(-4, 0, MarsTile { kind: Natural(Vulcano("Arsia Mons".to_string())), reward: vec![ModifyResources(Plant(2))] })
            .add(-3, 0, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(2))] })
            .add(-2, 0, MarsTile { kind: Natural(NaturalTile::NoctisCityReserved), reward: vec![ModifyResources(Plant(2))] })
            .add(-1, 0, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Plant(2))] })
            .add(0, 0, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Plant(2))] })
            .add(1, 0, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Plant(2))] })
            .add(2, 0, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(2))] })
            .add(3, 0, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(2))] })
            .add(4, 0, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(2))] })
            //row 6
            .add(-3, -1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(-2, -1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(2))] })
            .add(-1, -1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(0, -1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(1, -1, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(2, -1, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Plant(1))] })
            .add(3, -1, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Plant(1))] })
            .add(4, -1, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Plant(1))] })
            //row 7
            .add(-2, -2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(-1, -2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(0, -2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(1, -2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(2, -2, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(3, -2, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Plant(1))] })
            .add(4, -2, MarsTile { kind: Natural(Land), reward: vec![] })
            //row 8
            .add(-1, -3, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Steel(2))] })
            .add(0, -3, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(1, -3, MarsTile { kind: Natural(Land), reward: vec![DrawCard(1)] })
            .add(2, -3, MarsTile { kind: Natural(Land), reward: vec![DrawCard(1)] })
            .add(3, -3, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(4, -3, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Titanium(1))] })
            //row 9
            .add(0, -4, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Steel(1))] })
            .add(1, -4, MarsTile { kind: Natural(Land), reward: vec![ModifyResources(Steel(2))] })
            .add(2, -4, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(3, -4, MarsTile { kind: Natural(Land), reward: vec![] })
            .add(4, -4, MarsTile { kind: Natural(ReservedOcean), reward: vec![ModifyResources(Titanium(2))] })
        }
}
