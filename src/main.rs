mod models;
mod enums;
mod analytics;
mod cli;
mod data_handling;
mod utils;

use data_handling::fetch::fetch_html;
use data_handling::parse::parse_matches;
use analytics::find_streak::analyze_streaks;
use analytics::team_matches::create_team_matches_map;
use crate::cli::cli::validate_id;
use crate::enums::{HomeAway, MatchResult, MatchType};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let id = validate_id(&args);

    match fetch_html(&id).await {
        Ok(html) => {
            let games = parse_matches(&html);

            if games.is_empty() {
                println!("Inga matcher hittades.");
            } else {
                let team_matches_map = create_team_matches_map(games);
                analyze_streaks(
                    team_matches_map.clone(),
                    MatchResult::Win,
                    "seger",
                    None,
                    None,
                );
                analyze_streaks(
                    team_matches_map.clone(),
                    MatchResult::Loss,
                    "förlust",
                    None,
                    None,
                );
            }
        }
        Err(e) => eprintln!("Fel vid hämtning: {}", e),
    }
}
