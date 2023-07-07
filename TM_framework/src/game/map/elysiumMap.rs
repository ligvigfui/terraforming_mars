impl marsMap {
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
