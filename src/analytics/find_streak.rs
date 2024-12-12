use std::collections::HashMap;
use crate::cli::output;
use crate::models::team_match::TeamMatch;
use crate::enums::{MatchResult, HomeAway, MatchType};
use crate::models::win_streak::WinStreak;

fn matches_criteria(
    game: &TeamMatch,
    streak_type: &MatchResult, // Ändrat till referens
    home_away: Option<HomeAway>,
    match_type: Option<MatchType>,
) -> bool {
    game.result() == *streak_type && // Avreferera streak_type vid jämförelse
        home_away.as_ref().map_or(true, |ha| game.home_away() == *ha) &&
        match_type.as_ref().map_or(true, |mt| game.match_type() == *mt)
}

fn finalize_streak(
    team: &str,
    current_streak: &mut Vec<TeamMatch>,
    streaks: &mut Vec<WinStreak>,
) {
    if !current_streak.is_empty() {
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
        if matches_criteria(game, &streak_type, home_away.clone(), match_type.clone()) {
            current_streak.push(game.clone());
        } else {
            finalize_streak(team, &mut current_streak, &mut streaks);
        }
    }

    // Avsluta eventuell kvarvarande streak
    finalize_streak(team, &mut current_streak, &mut streaks);

    streaks
}

fn longest_streak_per_team(
    team: &str,
    streaks: &[WinStreak],
    longest_streaks_per_team: &mut HashMap<String, WinStreak>,
) {
    if let Some(longest) = streaks.iter().max_by_key(|streak| streak.wins()) {
        longest_streaks_per_team.insert(team.to_string(), longest.clone());
    }
}

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

        longest_streak_per_team(&team, &streaks, &mut longest_streaks_per_team);
        all_streaks.extend(streaks);
    }

    all_streaks.sort_by_key(|streak| streak.wins());
    all_streaks.reverse();

    output::print_top_streaks(&all_streaks, streak_name);

    let mut sorted_longest_streaks: Vec<_> = longest_streaks_per_team.into_iter().collect();
    sorted_longest_streaks.sort_by(|(team_a, _), (team_b, _)| team_a.cmp(team_b));

    output::print_team_streaks(&sorted_longest_streaks, streak_name);
}
