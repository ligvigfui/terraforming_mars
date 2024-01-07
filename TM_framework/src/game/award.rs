use crate::*;

type PointsTowardsAward = i16;
#[derive(Debug)]
pub struct Award {
    founded_by: Option<PlayerId>,
    victory_points_winner: i8,
    victory_points_second: i8,
    victory_points_losers: i8,
    points_for_player: fn(game: &Game, player_id: PlayerId) -> PointsTowardsAward,
}

impl Award {
    pub(crate) fn calculate_points(&self, game: &Game) -> HashMap<PlayerId, i8> {
        let mut player_award_points: Vec<(PlayerId, PointsTowardsAward)> = Vec::new();
        for player in &game.players {
            player_award_points.push((player.id, (self.points_for_player)(game, player.id)));
        }
        player_award_points.sort_by_key(|(_, points_for_award)| *points_for_award);
        let mut result: HashMap<PlayerId, i8> = HashMap::new();
        let first_points = player_award_points[0].1;
        while first_points == player_award_points[0].1 {
            result.insert(player_award_points[0].0, self.victory_points_winner);
            player_award_points.remove(0);
        }

        if game.players.len() == 2 {return result;}

        if result.keys().len() == 1 {
            let second_points = player_award_points[0].1;
            while second_points == player_award_points[0].1 {
                result.insert(player_award_points[0].0, self.victory_points_second);
                player_award_points.remove(0);
            }
        }

        while !player_award_points.is_empty() {
            result.insert(player_award_points[0].0, self.victory_points_losers);
            player_award_points.remove(0);
        }
        HashMap::new()
    }

    pub(crate) fn give_points(game: &mut Game) {
        let mut points: HashMap<PlayerId, i8> = HashMap::new();
        game.awards.iter().for_each(|award|
            if award.founded_by.is_some() {
                let inner_points = award.calculate_points(&game);
                for (player_id, point) in inner_points {
                    if points.contains_key(&player_id) {
                        *points.get_mut(&player_id).unwrap() += point;
                    } else {
                        points.insert(player_id, point);
                    }
                }
            }
        );
        for (player_id, point) in points {
            game.player(player_id).unwrap().vp += point as i16;
        }
    }
}

#[derive(Debug, Clone)]
pub struct AwardRules {
    initial_cost: i8,
    cost_increase: i8,
    number_of_awards_foundable: u8,
}

impl AwardRules {
    pub fn new() -> Self {
        AwardRules {
            initial_cost: 8,
            cost_increase: 6,
            number_of_awards_foundable: 3,
        }
    }
}