#![allow(dead_code)]
#![allow(unreachable_code)]

pub use std::collections::HashMap;

pub use crate::game::{*, 
    map::{*, 
        tile::{*,
            mars_tile::*,
            custom_tile::*,},
        mars_map::*,
    },
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
    player::*,
    turmoil::*,
};

pub mod game;

pub const VERSION: &str = "0.1.0-dev";


pub fn to_string<T>(something: T) -> String where T: std::fmt::Debug {
    format!("{:?}", something)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Picture {
    pub path: String,
}