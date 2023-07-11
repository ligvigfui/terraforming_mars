use core::fmt;

use crate::*;






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
}

#[derive(Clone)]
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
    PlaceTile(PlaceableTileType),
    RemoveTile(PlaceableTileType),
    ModifyTerraformRating(i8),
    ModifyGlobalParameter(GlobalParameter),
    ModifyCardResource(CardResource),
    PlaceColony,
    MoveDelegete,
    Custom(fn(&mut Game) -> Result<(), String>),
}

impl PartialEq for OnCardAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BuyCard(l0), Self::BuyCard(r0)) => l0 == r0,
            (Self::DrawCard(l0), Self::DrawCard(r0)) => l0 == r0,
            (Self::Discard(l0), Self::Discard(r0)) => l0 == r0,
            (Self::ModifyResources(l0), Self::ModifyResources(r0)) => l0 == r0,
            (Self::ModifyProduction(l0), Self::ModifyProduction(r0)) => l0 == r0,
            (Self::MustRemoveFromAnyPlayersResources(l0), Self::MustRemoveFromAnyPlayersResources(r0)) => l0 == r0,
            (Self::MustRemoveFromAnyPlayersProduction(l0), Self::MustRemoveFromAnyPlayersProduction(r0)) => l0 == r0,
            (Self::RemoveFromAnyPlayersResources(l0), Self::RemoveFromAnyPlayersResources(r0)) => l0 == r0,
            (Self::PlaceTile(l0), Self::PlaceTile(r0)) => l0 == r0,
            (Self::RemoveTile(l0), Self::RemoveTile(r0)) => l0 == r0,
            (Self::ModifyTerraformRating(l0), Self::ModifyTerraformRating(r0)) => l0 == r0,
            (Self::ModifyGlobalParameter(l0), Self::ModifyGlobalParameter(r0)) => l0 == r0,
            (Self::ModifyCardResource(l0), Self::ModifyCardResource(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub enum CardResource {
    Microbe(i8),
    Animal(i8),
    Science(i8),
    Floaters(i8),
    Asteroid(i8),
    Data(i8),
    Radtiation(i8),
    Custom(String, Picture, i8),
}