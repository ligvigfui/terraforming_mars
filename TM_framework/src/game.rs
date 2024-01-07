use crate::*;

pub mod card;
pub mod player;
pub mod milestone;
pub mod award;
pub mod map;
pub mod board;
pub mod turmoil;


//disable warn dead code
#[allow(dead_code)]


#[derive(Debug)]
enum Phase {
    // somehow get cards to chose from
    Draft,
    // buy cards to your hand
    Research,
    Action,
    Production,
    // end game check
    End,
    // push one Global parameter for pussies
    WorldGovernmentTerraforming,
    ColonyProduction,
    Turmoil,
    Custom(String),
    NextPlayer,
}

#[derive(Debug, Clone)]
pub enum Origin{
    Base,
    CorporateEra,
    Prelude,
    VenusNext,
    Colonies,
    Turmoil,
    Promo,
    Custom(String),
}

#[derive(Debug)]
///! still not good need to change milestone, award, phase, history, include turmoil, colonies
pub struct Game {
    rules: Rules,
    players: Vec<Player>,
    deck: Vec<ProjectCard>,
    discard: Vec<ProjectCard>,
    current_player: u8,
    generation: u8,
    map: MarsMap,
    temperature: i8,
    oxygen_times_10: u8,
    ocean: u8,
    venus: u8,
    milestones: Vec<Milestone>,
    awards: Vec<Award>,
    phase: Phase,
    history: Vec<String>,
}

impl Game {
    pub fn current_player(&mut self) -> &mut Player {
        &mut self.players[self.current_player as usize]
    }

    fn player(&mut self, id: PlayerId) -> Result<&mut Player, String> {
        if id >= self.players.len() as u8 {
            return Err(format!("Internal error: game.player player_id: {id} is greater then player number"));
        }
        Ok(&mut self.players[id as usize])
    }

    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len() as u8;
    }

    fn reshoufle(&mut self) {
        if self.deck.len() != 0 {return;}
        //todo rand:thread_random()
        let random = 0;
        for i in (0..self.discard.len()).rev() {
            //todo get rangom between 0..i
            let rand = i;
            self.deck.push(self.discard.remove(i));
        }
    }

    fn get_card_from_deck(&mut self) -> Option<ProjectCard> {
        match self.deck.pop() {
            Some(card) => Some(card),
            None => {
                if self.discard.is_empty() {return None;};
                self.reshoufle();
                self.deck.pop()
            }
        }
    }

    pub(crate) fn get_cards_from_deck(&mut self, n: usize) -> Vec<ProjectCard> {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            match self.get_card_from_deck() {
                Some(card) => vec.push(card),
                None => return vec,
            }
        }
        vec
    }

    pub(crate) fn play_oca(&mut self, action: OnCardAction, params: Vec<String>) -> Result<(), String> {
        match action {
        OnCardAction::ResearchCard(n) => {
            let cards = self.get_cards_from_deck(n.into());
            self.current_player().research.extend(cards);
            Ok(())
        },
        OnCardAction::DrawCard(n) => {
            let cards = self.get_cards_from_deck(n.into());
            self.current_player().hand.extend(cards);
            Ok(())
        }
        OnCardAction::Discard(n) => {
            if self.current_player().hand.len() < n.into() {
                let player_name = &self.current_player().name;
                return Err(format!("{player_name} doesn't have {n} cards in hand"));
            }
            if params.len() != n.into() {return Err("Internal error: game.play_oca/Discard param.len != n".to_string());}
            for card_id in params {
                let card = match self.current_player().hand.iter().position(|card| card_id == card.id) {
                    Some(position) => self.current_player().hand.remove(position),
                    None => return Err("Internal error: game.play_oca/discard card number is wrong".to_string()),
                };
                self.discard.push(card)
            }
            Ok(())
        }
        OnCardAction::BuyCardAfterResearch(n) => {
            let current_player = self.current_player();
            if Into::<u16>::into(current_player.characteristics.card_cost * n) < current_player.resources.money {
                let player_name = &current_player.name;
                let adequate_money = current_player.characteristics.card_cost * n;
                return Err(format!("{player_name} doesn't have {adequate_money} M€ to buy {n} cards"));
            }
            if params.len() != n.into() {return Err("Internal error: game.play_oca/BuyCard param.len != n".to_string());}
            for card_id in params {
                let card = match current_player.research.iter().position(|card| card_id == card.id) {
                    Some(position) => current_player.research.remove(position),
                    None => return Err("Internal error: game.play_oca/BuyCard number is wrong".to_string()),
                };
                current_player.hand.push(card)
            }
            Ok(())
        }
        OnCardAction::ModifyResources(resource) => self.current_player().resources.modify(resource),
        OnCardAction::ModifyProduction(production) => self.current_player().production.modify(production),
        OnCardAction::MustRemoveFromAnyPlayersResources(resource) => {
            let player_id = match params[0].parse::<PlayerId>() {
                Ok(index) => index,
                Err(e) => return Err(format!("Internal error: game.play_oca/MustRemoveFromAnyPlayersResources {e}")),
            };
            let player = &mut self.player(player_id)?;
            player.resources.modify(resource)
        },
        OnCardAction::MustRemoveFromAnyPlayersProduction(production) => {
            let player_id = match params[0].parse::<PlayerId>() {
                Ok(index) => index,
                Err(e) => return Err(format!("Internal error: game.play_oca/MustRemoveFromAnyPlayersProduction {e}")),
            };
            let player = &mut self.player(player_id)?;
            player.production.modify(production)
        }
        OnCardAction::RemoveFromAnyPlayersResources(resource) => {
            /// params: 
            /// 0: u8 player_id
            let player_index = match params[0].parse::<PlayerId>() {
                Ok(index) => index,
                Err(e) => return Err(format!("Internal error: game.play_oca/RemoveFromAnyPlayersResources {e}")),
            };
            let player = self.player(player_index)?;
            player.resources.remove_upto(resource)
        }
        OnCardAction::PlaceTile(tile_type) => {
            todo!()
        }
        OnCardAction::RemoveTile(_) => todo!(),
        OnCardAction::ModifyTerraformRating(_) => todo!(),
        OnCardAction::ModifyGlobalParameter(_) => todo!(),
        OnCardAction::ModifyCardResource(_) => todo!(),
        OnCardAction::PlaceColony => todo!(),
        OnCardAction::MoveDelegete => todo!(),
        OnCardAction::Custom(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rules {
    actions_per_turn: u8,
    award_rules: AwardRules,
    prelude: bool,
    map: Map,
    venus_next: bool,
    colonies: bool,
    turmoil: bool,
}

#[derive(Debug, Clone)]
pub enum Language {
    English(String),
    Hungarian(String),
}

#[derive(Debug,  Clone)]
pub enum VictoryPoint {
    VP(i8),
    PerTag(i8, Tag, u8),
    PerResource(i8, CardResource),
    PerNeighborTile(i8, PlaceableTile),
    Custom(fn(game: &Game) -> i8),
}