use crate::game::{Language};

use super::Tag;


#[derive(Debug)]
pub struct Corporation {
    name: Vec<Language>,
    effect: Vec<Language>,
    description: Vec<Language>,
    tags: Vec<Tag>,
}