use crate::*;


#[derive(Debug, Clone)]
pub struct Corporation {
    name: Vec<Language>,
    effect: Vec<Language>,
    description: Vec<Language>,
    tags: Vec<Tag>,
}