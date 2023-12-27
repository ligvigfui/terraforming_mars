#![allow(dead_code)]
#![allow(unreachable_code)]

pub use crate::game::{*, 
        map::{*, 
            tile::{*,
                marsTile::*,
                spaceTile::*,
                customTile::*,},
            mars_map::{*, 
                base_map::*, 
                hellas_map::*, 
                elysium_map::*},
            space_map::*,},
        card::{*,
            action::*,
            corporation::*, 
            prelude::*, 
            project::*,
            standard_project::*,
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