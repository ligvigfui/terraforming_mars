use crate::*;

pub mod card;
pub mod player;
pub mod milestone;
pub mod award;
pub mod map;
pub mod board;



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

#[derive(Debug)]
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
    players: Vec<Player>,
    deck: Vec<ProjectCard>,
    discard: Vec<ProjectCard>,
    current_player: u8,
    generation: u8,
    map: String,
    temperature: i8,
    oxygen: u8,
    ocean: u8,
    venus: u8,
    milestones: Vec<Milestone>,
    awards: Vec<Award>,
    phase: Phase,
    history: String,
}

impl Game {
    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player as usize]
    }
    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len() as u8;
    }

}


pub struct Rules {
    actions_per_turn: u8,
    award_rules: AwardRules,
    prelude: bool,
    map: Maps,
    venus_next: bool,
    colonies: bool,
    turmoil: bool,
}

#[derive(Debug)]
pub enum Language {
    English(String),
    Hungarian(String),
}


#[derive(Debug)]
pub enum TurmoilParty {
    Scientists,
    Unity,
    Reds,
    Greens,
    Kelvinists,
    MarsFirst,
}

#[derive(Debug)]
pub enum VictoryPoint {
    VP(i8),
    PerTag(i8, Tag, u8),
    PerResource(i8, CardResource),
}