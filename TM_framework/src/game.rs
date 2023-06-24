use self::{player::Player, card::project::ProjectCard, milestone::Milestone, award::{Award, AwardRules}};

pub mod card;
pub mod player;
pub mod milestone;
pub mod award;



//disable warn dead code
#[allow(dead_code)]









#[derive(Debug)]
enum Phase {
    Draft,
    Research,
    Action,
    Production,
    End,
    WorldGovernmentTerraforming,
    ColonyProduction,
    Turmoil,
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
    pub fn currentPlayer(&self) -> &Player {
        &self.players[self.current_player as usize]
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

#[derive(Debug)]
pub enum MinMax {
    Min,
    Max,
}

#[derive(Debug)]
pub enum GlobalParameter {
    Temperature(i32),
    Oxygen(usize),
    Ocean(usize),
    Venus(usize),
}

#[derive(Debug)]
pub struct GlobalParameters {
    temperature: GlobalParameter,
    oxygen: GlobalParameter,
    ocean: GlobalParameter,
    venus: GlobalParameter,
}

impl GlobalParameters {
    pub fn new() -> Self {
        GlobalParameters {
            temperature: GlobalParameter::Temperature(-30),
            oxygen: GlobalParameter::Oxygen(0),
            ocean: GlobalParameter::Ocean(0),
            venus: GlobalParameter::Venus(0),
        }
    }
}

///! TODO
#[derive(Debug)]
pub enum Icon {
    Icon(String),
}