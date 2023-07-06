use core::fmt;

use crate::{*, tile::marsTile::*};






pub mod corporation;
pub mod prelude;
pub mod project;
pub mod standardProject;
pub mod action;


    
#[derive(Debug)]
enum CardType {
    Corporation,
    Prelude,
    Project,
}

pub trait Card: Sized {
    // does not check if the card can be played
    fn play(player: &mut Player, card: &Self) -> Result<(), String>;
    // checks if the card can be played
    fn can_be_played(player: &Player, card: &Self) -> Result<(), String>;
}

#[derive(Debug)]
pub struct Effect {
    criteria: Vec<OnCardAction>,
    reward: OnCardAction,
    prelude: prelude::Prelude,
}

#[derive(Debug)]
pub enum CardPhase {

}

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
    ModifyGlobalParameter(GlobalParameter),
    ModifyCardResource(CardResource),
    PlaceColony,
    MoveDelegete,
    Custom(Box<dyn FnOnce(&mut Game) -> Result<(), String>>),
}

impl fmt::Debug for OnCardAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OnCardAction::BuyCard(amount) => {
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


#[derive(Debug)]
pub enum VictoryPoint {
    None,
    VP(i8),
    PerTag(i8, Tag, u8),
    PerResource(i8, CardResource),
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
    Microbe(i8),
    Animal(i8),
    Science(i8),
    Floaters(i8),
    Asteroid(i8),
    Data(i8),
    Radtiation(i8),
    Custom(String, Icon, i8),
}