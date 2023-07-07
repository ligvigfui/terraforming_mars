#![allow(dead_code)]
#![allow(unreachable_code)]

pub use crate::{
    game::{*, 
           map::{*, 
                 marsMap::{*, 
                           baseMap::*, 
                           hellasMap::*, 
                           elysiumMap::*}, 
                 tile::*},
           board::*, 
           card::{*, 
                  project::*, 
                  corporation::*, 
                  prelude::*}, 
           award::*, 
           milestone::*, 
           player::*}, 
    user::*};

pub mod game;
pub mod user;

pub const version: &str = "0.1.0-dev";


pub fn to_string<T>(something: T) -> String where T: std::fmt::Debug {
    format!("{:?}", something)
}
