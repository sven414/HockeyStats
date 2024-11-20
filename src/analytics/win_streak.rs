use std::collections::HashMap;
use crate::models::team_match::TeamMatch;
use crate::enums::MatchResult;
use crate::models::win_streak::WinStreak;

/// Hitta alla vinstsviter för ett lag
fn find_win_streaks(team: &str, matches: &[TeamMatch]) -> Vec<WinStreak> {
    let mut streaks = Vec::new();
    let mut current_streak: Vec<TeamMatch> = Vec::new();

    for game in matches {
        if game.result() == MatchResult::Win {
            current_streak.push(game.clone());
        } else {
            if !current_streak.is_empty() {
                // Stäng av sviten och spara
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

/// Hitta de längsta sviterna i ligan och per lag
pub fn analyze_win_streaks(team_matches_map: HashMap<String, Vec<TeamMatch>>) {
    let mut all_streaks = Vec::new();
    let mut longest_streaks_per_team = HashMap::new();

    for (team, matches) in team_matches_map {
        let streaks = find_win_streaks(&team, &matches);

        if let Some(longest) = streaks.iter().max_by_key(|streak| streak.wins()) {
            longest_streaks_per_team.insert(team.clone(), longest.clone());
        }
        all_streaks.extend(streaks);
    }

    // Sortera alla sviter efter längd och skriv ut de 5 längsta
    all_streaks.sort_by_key(|streak| streak.wins());
    all_streaks.reverse();

    println!("Top 5 längsta segersviter i ligan:");
    for streak in all_streaks.iter().take(5) {
        println!(
            "Lag: {}, Vinster: {}, Start: {}, Slut: {}",
            streak.team(),
            streak.wins(),
            streak.start_date().format("%Y-%m-%d"), // Endast datum
            streak.end_date().format("%Y-%m-%d")   // Endast datum
        );
    }

    // Skriv ut de längsta sviterna per lag
    println!("\nLängsta segersviter per lag:");
    for (team, streak) in longest_streaks_per_team {
        println!(
            "Lag: {}, Vinster: {}, Start: {}, Slut: {}",
            team,
            streak.wins(),
            streak.start_date().format("%Y-%m-%d"), // Endast datum
            streak.end_date().format("%Y-%m-%d")   // Endast datum
        );
    }
}

