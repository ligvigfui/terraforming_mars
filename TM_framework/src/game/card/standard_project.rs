use crate::*;


#[derive(Debug)]
pub struct StandardProject {
    name: String,
    cost: Vec<OnCardAction>,
    reward: Vec<OnCardAction>,
}