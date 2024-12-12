mod models;
mod enums;
mod analytics;
mod cli;
mod data_handling;
mod utils;
mod orchestrator;

use analytics::analyze_matches::analyze_matches;
use analytics::team_matches::create_team_matches_map;
use cli::cli::validate_args;
use analytics::display_team_streaks::display_team_streaks;
use crate::orchestrator::prepare_data::prepare_data;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (id, analyze_win, analyze_loss, show_all_teams) = validate_args(&args);

    match prepare_data(&id).await {
        Ok(games) => {
            let team_matches_map = create_team_matches_map(games);

            // Utför analys
            analyze_matches(&team_matches_map, analyze_win, analyze_loss);

            // Visa sviter per lag om flaggan är aktiverad
            if show_all_teams {
                display_team_streaks(&team_matches_map, analyze_win, analyze_loss);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
