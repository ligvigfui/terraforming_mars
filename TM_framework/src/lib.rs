#![allow(dead_code)]
#![allow(unreachable_code)]

pub use crate::game::{*, 
        map::{*, 
            tile::{*,
                marsTile::*,
                spaceTile::*,
                customTile::*,},
            marsMap::{*, 
                baseMap::*, 
                hellasMap::*, 
                elysiumMap::*},
            spaceMap::*,},
        card::{*,
            action::*,
            corporation::*, 
            prelude::*, 
            project::*,
            standardProject::*,
        },
        award::*,
        board::*,
        milestone::*, 
        player::*};

pub mod game;

pub const VERSION: &str = "0.1.0-dev";


pub fn to_string<T>(something: T) -> String where T: std::fmt::Debug {
    format!("{:?}", something)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Picture {
    pub path: String,
}