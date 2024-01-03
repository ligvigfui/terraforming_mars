use core::fmt;

use crate::*;

pub mod corporation;
pub mod prelude;
pub mod project;
pub mod standard_project;
pub mod action;
    
#[derive(Debug)]
enum CardType {
    Corporation,
    Prelude,
    Project,
}

pub trait Playable: Sized {
    // does not check if the card can be played
    fn play(self: &Self, player: &mut Player) -> Result<(), String>;
    // checks if the card can be played
    fn can_be_played(self: &Self, player: &Player) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct Effect {
    criteria: Vec<OnCardAction>,
    reward: OnCardAction,
}

#[derive(Clone, PartialEq)]
pub enum OnCardAction {
    // move cards from the deck to the players reserach place
    ResearchCard(u8),
    // buy card from research to hand
    BuyCardAfterResearch(u8),
    // draw random card from deck witouth moving them to research (you don't have an option to chose)
    DrawCard(u8),
    // move card from research or hand to discard
    Discard(u8),
    ModifyResources(Resource),
    ModifyProduction(Resource),
    MustRemoveFromAnyPlayersResources(Resource), 
    MustRemoveFromAnyPlayersProduction(Resource),
    RemoveFromAnyPlayersResources(Resource),
    PlaceTile(PlaceableTile),
    RemoveTile(PlaceableTile),
    ModifyTerraformRating(i8),
    ModifyGlobalParameter(GlobalParameter),
    ModifyCardResource(CardResource),
    PlaceColony,
    MoveDelegete,
    Custom(fn(&mut Game, params: Vec<String>) -> Result<(), String>),
}

impl fmt::Debug for OnCardAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OnCardAction::ResearchCard(amount) => {
                write!(f, "ResearchCard({})", amount)
            },
            OnCardAction::BuyCardAfterResearch(amount) => {
                write!(f, "BuyCard({})", amount)
            },
            OnCardAction::DrawCard(amount) => {
                write!(f, "DrawCard({})", amount)
            },
            OnCardAction::Discard(amount) => {
                write!(f, "Discard({})", amount)
            },
            OnCardAction::ModifyResources(resource) => {
                write!(f, "ModifyResources({:?})", resource)
            },
            OnCardAction::ModifyProduction(resource) => {
                write!(f, "ModifyProduction({:?})", resource)
            },
            OnCardAction::MustRemoveFromAnyPlayersResources(resource) => {
                write!(f, "MustRemoveFromAnyPlayersResources({:?})", resource)
            },
            OnCardAction::MustRemoveFromAnyPlayersProduction(resource) => {
                write!(f, "MustRemoveFromAnyPlayersProduction({:?})", resource)
            },
            OnCardAction::RemoveFromAnyPlayersResources(resource) => {
                write!(f, "RemoveFromAnyPlayersResources({:?})", resource)
            },
            OnCardAction::PlaceTile(tile) => {
                write!(f, "PlaceTile({:?})", tile)
            },
            OnCardAction::RemoveTile(tile) => {
                write!(f, "RemoveTile({:?})", tile)
            },
            OnCardAction::ModifyTerraformRating(amount) => {
                write!(f, "ModifyTerraformRating({})", amount)
            },
            OnCardAction::ModifyGlobalParameter(parameter) => {
                write!(f, "ModifyGlobalParameter({:?})", parameter)
            },
            OnCardAction::ModifyCardResource(card_resource) => {
                write!(f, "ModifyCardResource({:?})", card_resource)
            },
            OnCardAction::PlaceColony => {
                write!(f, "PlaceColony")
            },
            OnCardAction::MoveDelegete => {
                write!(f, "MoveDelegete")
            },
            OnCardAction::Custom(_) => {
                write!(f, "Custom")
            },
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum CardResource {
    Microbe(i8),
    Animal(i8),
    Science(i8),
    Floaters(i8),
    Asteroid(i8),
    Data(i8),
    Custom(String, i8),
}