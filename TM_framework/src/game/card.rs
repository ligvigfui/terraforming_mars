use super::{player::{Player, Resource}, GlobalParameter, Icon};



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
    BuyCard(usize),
    // draw random card from deck
    DrawCard(usize),
    // move card from research or hand to discard
    Discard(usize),
    ModifyResources(Resource),
    ModifyProduction(Resource),
    MustRemoveFromAnyPlayersResources(Resource), 
    MustRemoveFromAnyPlayersProduction(Resource),
    RemoveFromAnyPlayersResources(Resource),
    PlaceTile,
    RemoveTile,
    ModifyTerraformRating(i32),
    ModifyGlobalParameter(GlobalParameter, i32),
    ModifyCardResource(CardResource, i32),
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