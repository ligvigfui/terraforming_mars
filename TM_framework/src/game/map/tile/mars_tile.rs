use crate::*;

impl Tile for MarsTile {}
#[derive(Debug, Clone)]
pub struct MarsTile {
    pub kind: MarsTileKind,
    pub reward: Vec<OnCardAction>,
}

impl PartialEq for MarsTile {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone)]
pub enum MarsTileKind {
    Natural(NaturalTile),
    Occupied { tile: PlaceableTile, player_id: u8 },
}

impl PartialEq for MarsTileKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Natural(l), Self::Natural(r)) => l == r,
            (Self::Occupied { tile: l, player_id: _ }, Self::Occupied { tile: r, player_id: _ }) =>
                l == r,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NaturalTile {
    Land,
    Vulcano(String),
    ReservedOcean,
    NoctisCityReserved,
}

impl PartialEq for NaturalTile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Vulcano(_), Self::Vulcano(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Placeable for PlaceableTile {}
#[derive(Debug, PartialEq, Clone)]
pub enum PlaceableTile {
    Ocean,
    City,
    Greenery,
    Custom(CustomTile),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_tile_partial_eq() {
        let mut tile1 = NaturalTile::Vulcano("()".to_string());
        let mut tile2 = NaturalTile::Vulcano("a".to_string());
        assert_eq!(tile1, tile2);
        tile2 = NaturalTile::Land;
        assert_ne!(tile1, tile2);
        tile1 = NaturalTile::Land;
        assert_eq!(tile1, tile2);
    }

    #[test]
    fn mars_tile_kind_partial_eq() {
        let mut tile1 = MarsTileKind::Occupied { tile: PlaceableTile::City, player_id: 1 };
        let mut tile2 = MarsTileKind::Occupied { tile: PlaceableTile::City, player_id: 2 };
        assert_eq!(tile1, tile2);
        tile2 = MarsTileKind::Occupied { tile: PlaceableTile::Greenery, player_id: 2 };
        assert_ne!(tile1, tile2);
        tile1 = MarsTileKind::Occupied { tile: PlaceableTile::Greenery, player_id: 1 };
        assert_eq!(tile1, tile2);

        tile2 = MarsTileKind::Natural(NaturalTile::Vulcano("()".to_string()));
        assert_ne!(tile1, tile2);
        tile1 = MarsTileKind::Natural(NaturalTile::Land);
        assert_ne!(tile1, tile2);
    }

    #[test]
    fn mars_tile_partial_eq() {
        let tile1 = MarsTile {
            kind: MarsTileKind::Natural(NaturalTile::Land),
            reward: vec![OnCardAction::DrawCard(2)],
        };
        let tile2: MarsTile = MarsTile {
            kind: MarsTileKind::Natural(NaturalTile::Land),
            reward: vec![OnCardAction::ModifyResources(Resource::Plant(2))],
        };
        assert_eq!(tile1, tile2);
    }
}