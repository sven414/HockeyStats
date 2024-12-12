use std::collections::HashMap;
use crate::analytics::find_streak::analyze_streaks;
use crate::enums::MatchResult;
use crate::models::team_match::TeamMatch;

/// Visar sviter per lag om flaggan är aktiverad.
pub fn display_team_streaks(
    team_matches_map: &HashMap<String, Vec<TeamMatch>>,
    analyze_win: bool,
    analyze_loss: bool,
) {
    println!("Visar sviter per lag...");

    if analyze_win {
        analyze_streaks(
            team_matches_map.clone(),
            MatchResult::Win,
            "seger",
            None,
            None,
        );
    }

    if analyze_loss {
        analyze_streaks(
            team_matches_map.clone(),
            MatchResult::Loss,
            "förlust",
            None,
            None,
        );
    }
}
