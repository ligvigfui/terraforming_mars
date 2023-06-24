use super::{*, card::{project::ProjectCard, Tag, prelude::Prelude, corporation::Corporation}};


#[derive(Debug)]
///! still not good need to change milestone, award, action
pub struct Player {
    id: usize,
    name: String,
    hand: Vec<ProjectCard>,
    played_cards: Vec<ProjectCard>,
    production: Vec<Resource>,
    resources: Vec<Resource>,
    tags: Vec<(Tag, usize)>,
    vp: i32,
    terraform_rating: usize,
    corporation: Vec<Corporation>,
    prelude: Vec<Prelude>,
    actions: usize,
}

impl Player {
    pub fn hand(&self) -> &Vec<ProjectCard> {
        &self.hand
    }
    pub fn tags(&self) -> &Vec<(Tag, usize)> {
        &self.tags
    }
}

#[derive(Debug)]
pub enum Resource {
    Money(usize),
    Steel(usize),
    Titanium(usize),
    Plant(usize),
    Energy(usize),
    Heat(usize),
}