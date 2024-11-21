use std::collections::HashMap;
use crate::models::win_streak::WinStreak;

pub fn print_top_streaks(streaks: &[WinStreak], streak_name: &str) {
    println!("Top 5 längsta {}-sviter i ligan:", streak_name);
    for streak in streaks.iter().take(5) {
        println!(
            "Lag: {}, Längd: {}, Start: {}, Slut: {}",
            streak.team(),
            streak.wins(),
            streak.start_date().format("%Y-%m-%d"),
            streak.end_date().format("%Y-%m-%d")
        );
    }
}

pub fn print_team_streaks(longest_streaks_per_team: &HashMap<String, WinStreak>, streak_name: &str) {
    println!("\nLängsta {}-sviter per lag:", streak_name);
    for (team, streak) in longest_streaks_per_team {
        println!(
            "Lag: {}, Längd: {}, Start: {}, Slut: {}",
            team,
            streak.wins(),
            streak.start_date().format("%Y-%m-%d"),
            streak.end_date().format("%Y-%m-%d")
        );
    }
}
