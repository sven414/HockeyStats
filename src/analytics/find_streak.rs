use std::collections::HashMap;
use crate::analytics::output;
use crate::models::team_match::TeamMatch;
use crate::enums::{MatchResult, HomeAway, MatchType};
use crate::models::win_streak::WinStreak;

/// Finds streaks for a team based on multiple parameters.
///
/// This function iterates through a list of matches for a team and identifies streaks
/// of a specified type (e.g., wins or losses). It can also filter the streaks based on
/// home/away games and match type. The identified streaks are returned as a vector of `WinStreak` objects.
fn find_streaks(
    team: &str,
    matches: &[TeamMatch],
    streak_type: MatchResult,
    home_away: Option<HomeAway>,
    match_type: Option<MatchType>,
) -> Vec<WinStreak> {
    let mut streaks = Vec::new();
    let mut current_streak: Vec<TeamMatch> = Vec::new();

    for game in matches {
        let matches_criteria =
            game.result() == streak_type &&                              // Matchar MatchResult
                home_away.as_ref().map_or(true, |ha| game.home_away() == *ha) && // Matchar HomeAway (om angivet)
                match_type.as_ref().map_or(true, |mt| game.match_type() == *mt); // Matchar MatchType (om angivet)

        if matches_criteria {
            current_streak.push(game.clone());
        } else {
            if !current_streak.is_empty() {
                // Avsluta sviten och spara
                let start = current_streak.first().unwrap().datetime().clone();
                let end = current_streak.last().unwrap().datetime().clone();
                streaks.push(WinStreak::new(
                    team.to_string(),
                    current_streak.len(),
                    start,
                    end,
                    current_streak.clone(),
                ));
                current_streak.clear();
            }
        }
    }

    if !current_streak.is_empty() {
        let start = current_streak.first().unwrap().datetime().clone();
        let end = current_streak.last().unwrap().datetime().clone();
        streaks.push(WinStreak::new(
            team.to_string(),
            current_streak.len(),
            start,
            end,
            current_streak,
        ));
    }

    streaks
}

/// Analyzes streaks based on multiple parameters.
///
/// This function takes a map of team matches and analyzes streaks of a specified type
/// (e.g., wins or losses) for each team. It can also filter the streaks based on
/// home/away games and match type. The results are printed to the console.
///
/// # Arguments
///
/// * `team_matches_map` - A `HashMap` where the key is the team name and the value is a vector of `TeamMatch` objects.
/// * `streak_type` - The type of streak to analyze (e.g., `MatchResult::Win` or `MatchResult::Loss`).
/// * `streak_name` - A string representing the name of the streak (e.g., "win" or "loss").
/// * `home_away` - An optional filter for home or away games (`Some(HomeAway::Home)` or `Some(HomeAway::Away)`).
/// * `match_type` - An optional filter for the type of match (`Some(MatchType::Regular)` or `Some(MatchType::Overtime)`).
pub fn analyze_streaks(
    team_matches_map: HashMap<String, Vec<TeamMatch>>,
    streak_type: MatchResult,
    streak_name: &str,
    home_away: Option<HomeAway>,
    match_type: Option<MatchType>,
) {
    let mut all_streaks = Vec::new();
    let mut longest_streaks_per_team = HashMap::new();

    for (team, matches) in team_matches_map {
        let streaks = find_streaks(
            &team,
            &matches,
            streak_type.clone(),
            home_away.clone(),
            match_type.clone(),
        );

        if let Some(longest) = streaks.iter().max_by_key(|streak| streak.wins()) {
            longest_streaks_per_team.insert(team.clone(), longest.clone());
        }
        all_streaks.extend(streaks);
    }

    // Sortera alla sviter efter längd och skriv ut de 5 längsta
    all_streaks.sort_by_key(|streak| streak.wins());
    all_streaks.reverse();

    output::print_top_streaks(&all_streaks, streak_name);

    // Sortera längsta sviter per lag efter lagnamn (alfabetisk ordning)
    let mut sorted_longest_streaks: Vec<_> = longest_streaks_per_team.into_iter().collect();
    sorted_longest_streaks.sort_by(|(team_a, _), (team_b, _)| team_a.cmp(team_b));

    output::print_team_streaks(&sorted_longest_streaks, streak_name);
}
