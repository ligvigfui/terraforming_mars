use crate::*;






pub mod corporation;
pub mod prelude;
pub mod project;


    
#[derive(Debug)]
enum CardType {
    Corporation,
    Prelude,
    Project,
}

pub trait Card: Sized {
    fn play(player: &mut Player, card: &Self);
}


pub struct Effect {
    criteria: Option<OnCardAction>,
    reward: OnCardAction,
    prelude: prelude::Prelude,
}



#[derive(Debug)]
pub enum OnCardAction {
    // move card from research to hand
    BuyCard(u8),
    // draw random card from deck
    DrawCard(u8),
    // move card from research or hand to discard
    Discard(u8),
    ModifyResources(Resource),
    ModifyProduction(Resource),
    MustRemoveFromAnyPlayersResources(Resource), 
    MustRemoveFromAnyPlayersProduction(Resource),
    RemoveFromAnyPlayersResources(Resource),
    PlaceTile(OccupiedTileType),
    RemoveTile(OccupiedTileType),
    ModifyTerraformRating(i8),
    ModifyGlobalParameter(GlobalParameter, i8),
    ModifyCardResource(CardResource, i8),
    PlaceColony,
    MoveDelegete,
}



#[derive(Debug)]
pub enum VictoryPoint {
    None,
    VP(i32),
    PerTag(i32, Tag, usize),
    PerResource(i32, CardResource, usize),
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    Building,
    Space,
    Science,
    Plant,
    Microbe,
    Animal,
    City,
    Earth,
    Jovian,
    Power,
    Event,
    Venus,
    Wild,
    Custom(String),
}

#[derive(Debug)]
pub enum CardResource {
    Microbe(usize),
    Animal(usize),
    Science(usize),
    Floaters(usize),
    Asteroid(usize),
    Data(usize),
    Radtiation(usize),
    Custom(String, Icon, usize),
}