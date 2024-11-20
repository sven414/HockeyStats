    use std::collections::HashMap;
    use chrono::NaiveDateTime;
    use crate::enums::{HomeAway};
    use crate::models::game::Game;
    use crate::models::team_match::TeamMatch;
    use crate::utils::classify_game_result::classify_result;
    use crate::utils::classify_match_type::classify_match_type;

    pub fn create_team_matches_map(games: Vec<Game>) -> HashMap<String, Vec<TeamMatch>> {
        let mut team_matches_map: HashMap<String, Vec<TeamMatch>> = HashMap::new();

        for game in games {
            let datetime = NaiveDateTime::parse_from_str(
                &format!("{} {}", game.date(), game.time()),
                "%Y-%m-%d %H:%M",
            )
                .expect("Ogiltigt datum eller tid");

            let match_type = classify_match_type(game.period_result().unwrap_or(""));

            let home_result = classify_result(&game, HomeAway::Home);
            team_matches_map
                .entry(game.home_team().to_string())
                .or_insert_with(Vec::new)
                .push(TeamMatch::new(
                    datetime.clone(),
                    game.away_team().to_string(),
                    home_result,
                    match_type.clone(),
                    HomeAway::Home,
                ));

            let away_result = classify_result(&game, HomeAway::Away);
            team_matches_map
                .entry(game.away_team().to_string())
                .or_insert_with(Vec::new)
                .push(TeamMatch::new(
                    datetime,
                    game.home_team().to_string(),
                    away_result,
                    match_type,
                    HomeAway::Away,
                ));
        }

        team_matches_map
    }
