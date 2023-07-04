use self::{player::Player, card::project::ProjectCard, milestone::Milestone, award::{Award, AwardRules}};

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
    current_player: usize,
    generation: usize,
    map: String,
    temperature: i32,
    oxygen: usize,
    ocean: usize,
    venus: usize,
    milestones: Vec<Milestone>,
    awards: Vec<Award>,
    phase: String,
    history: String,
}

impl Game {
    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player as usize]
    }
    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

}


pub struct Rules {
    award_rules: AwardRules,
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


///! TODO
#[derive(Debug)]
pub enum Icon {
    Icon(String),
}