use crate::models::win_streak::WinStreak;

/// Prints the top 5 longest streaks in the league.
///
/// This function takes a slice of `WinStreak` objects and prints the top 5 longest streaks
/// of a specified type (e.g., wins or losses) to the console.
pub fn print_top_streaks(streaks: &[WinStreak], streak_name: &str) {
    println!("\nTop 5 längsta {}-sviter i ligan:", streak_name);
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

/// Prints the longest streaks per team.
///
/// This function takes a `HashMap` where the key is the team name and the value is the longest
/// `WinStreak` object for that team. It prints the longest streaks of a specified type (e.g., wins or losses)
/// for each team to the console.
pub fn print_team_streaks(
    sorted_longest_streaks: &[(String, WinStreak)],
    streak_name: &str,
) {
    println!("\nLängsta {}-sviter per lag:", streak_name);
    for (team, streak) in sorted_longest_streaks {
        println!(
            "Lag: {}, Längd: {}, Start: {}, Slut: {}",
            team,
            streak.wins(),
            streak.start_date().format("%Y-%m-%d"),
            streak.end_date().format("%Y-%m-%d")
        );
    }
}

