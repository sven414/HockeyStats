use crate::models::game::Game;
use crate::enums::{HomeAway, MatchResult};

pub fn classify_result(game: &Game, location: HomeAway) -> MatchResult {
    let (goal_for, goal_against) = match (game.goal_for(), game.goal_against()) {
        (Some(gf), Some(ga)) => (gf, ga),
        _ => panic!("Match utan resultat kan inte klassificeras"),
    };

    if goal_for == goal_against {
        MatchResult::Draw
    } else if (location == HomeAway::Home && goal_for > goal_against)
        || (location == HomeAway::Away && goal_for < goal_against)
    {
        MatchResult::Win
    } else {
        MatchResult::Loss
    }
}
