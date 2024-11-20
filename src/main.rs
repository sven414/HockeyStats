mod models;
mod enums;
mod analytics;
mod cli;
mod data_handling;
mod utils;

use data_handling::fetch::fetch_html;
use data_handling::parse::parse_matches;
use analytics::win_streak::analyze_win_streaks;
use analytics::team_matches::create_team_matches_map;
use crate::cli::cli::validate_id;

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
                analyze_win_streaks(team_matches_map);
            }
        }
        Err(e) => eprintln!("Fel vid hämtning: {}", e),
    }
}
