use std::collections::HashMap;
use std::env;
use chrono::NaiveDateTime;

mod fetch;
mod models;
mod parse;
mod game_factory;
mod enums;
mod classify_game_result;
mod classify_match_type;
mod win_streak;

use models::team_match::TeamMatch;
use crate::enums::{HomeAway};
use classify_game_result::classify_result;
use classify_match_type::classify_match_type;
use win_streak::{analyze_win_streaks};

#[tokio::main]
async fn main() {
    // Hämta kommandoradsargumentet
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Fel: Du måste ange ett femsiffrigt nummer som argument.");
        std::process::exit(1);
    }
    let id = &args[1];

    if id.len() != 5 || !id.chars().all(|c| c.is_digit(10)) {
        eprintln!("Fel: Argumentet måste vara ett femsiffrigt nummer.");
        std::process::exit(1);
    }

    use parse::parse_matches;

    match fetch::fetch_html(id).await {
        Ok(html) => {
            let games = parse_matches(&html);

            if games.is_empty() {
                println!("Inga matcher hittades.");
            } else {
                let mut team_matches_map: HashMap<String, Vec<TeamMatch>> = HashMap::new();

                for game in games {
                    let datetime = NaiveDateTime::parse_from_str(
                        &format!("{} {}", game.date(), game.time()),
                        "%Y-%m-%d %H:%M",
                    )
                        .expect("Ogiltigt datum eller tid");

                    // Klassificera matchtypen
                    let match_type = classify_match_type(game.period_result().unwrap_or(""));

                    // Skapa och lägg till hemmalagets match
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

                    // Skapa och lägg till bortalagets match
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

                analyze_win_streaks(team_matches_map);

            }
        }
        Err(e) => eprintln!("Fel vid hämtning: {}", e),
    }
}