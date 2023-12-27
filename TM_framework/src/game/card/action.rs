use crate::*;


#[derive(Debug, Clone)]
pub struct Action {
    played_this_generation: bool,
    cost: Vec<OnCardAction>,
    reward: Vec<OnCardAction>,
}

impl Playable for Action {
    fn play(self: &Self, player: &mut Player) -> Result<(), String> {
        todo!();
        Ok(())
    }

    fn can_be_played(self: &Self, player: &Player) -> Result<(), String> {
        todo!();
        Ok(())
    }
}