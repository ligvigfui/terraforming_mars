use crate::{OnCardAction, Card, Player};


#[derive(Debug)]
pub struct Action {
    played_this_generation: bool,
    cost: Vec<OnCardAction>,
    reward: Vec<OnCardAction>
}

impl Card for Action {
    fn play(player: &mut Player, card: &Self) -> Result<(), String>{
        todo!();
        Ok(())
    }
    fn can_be_played(player: &Player, card: &Self) -> Result<(), String> {
        todo!("implement card for action");
        Ok(())
    }
}