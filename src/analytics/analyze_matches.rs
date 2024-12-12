use std::collections::HashMap;
use crate::analytics::find_streak::analyze_streaks;
use crate::enums::{MatchResult, HomeAway, MatchType};
use crate::models::team_match::TeamMatch;

/// Utför analys av matcher baserat på flaggor.
pub fn analyze_matches(
    team_matches_map: &HashMap<String, Vec<TeamMatch>>,
    analyze_win: bool,
    analyze_loss: bool,
) {
    if analyze_win {
        println!("Analyserar seger-sviter...");
        analyze_streaks(
            team_matches_map.clone(),
            MatchResult::Win,
            "seger",
            None,
            None,
        );
    }

    if analyze_loss {
        println!("Analyserar förlust-sviter...");
        analyze_streaks(
            team_matches_map.clone(),
            MatchResult::Loss,
            "förlust",
            None,
            None,
        );
    }
}
