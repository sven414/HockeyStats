use crate::models::game::Game;

/// Skapar ett `Game`-objekt
pub fn create_game(
    date: String,
    time: String,
    home_team: String,
    away_team: String,
    goal_for: Option<u8>,
    goal_against: Option<u8>,
    period_result: Option<String>,
    match_id: Option<String>,
    spectators: Option<u32>,
    venue: String,
    league: String,
    season: String,
) -> Game {
    Game::new(
        date,
        time,
        home_team,
        away_team,
        goal_for,
        goal_against,
        period_result,
        match_id,
        spectators,
        venue,
        league,
        season,
    )
}
