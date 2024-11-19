use chrono::NaiveDateTime;
use crate::models::team_match::TeamMatch;

/// Struktur för att representera en vinstsvit
#[derive(Debug, Clone)]
pub struct WinStreak {
    team: String,
    wins: usize,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    details: Vec<TeamMatch>, // Detaljer om matcherna i sviten
}

impl WinStreak {
    pub fn new(
        team: String,
        wins: usize,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        details: Vec<TeamMatch>,
    ) -> Self {
        WinStreak {
            team,
            wins,
            start_date,
            end_date,
            details,
        }
    }

    // Getter- och setter-metoder
    pub fn team(&self) -> &String {
        &self.team
    }

    pub fn set_team(&mut self, team: String) {
        self.team = team;
    }

    pub fn wins(&self) -> usize {
        self.wins
    }

    pub fn set_wins(&mut self, wins: usize) {
        self.wins = wins;
    }

    pub fn start_date(&self) -> &NaiveDateTime {
        &self.start_date
    }

    pub fn set_start_date(&mut self, start_date: NaiveDateTime) {
        self.start_date = start_date;
    }

    pub fn end_date(&self) -> &NaiveDateTime {
        &self.end_date
    }

    pub fn set_end_date(&mut self, end_date: NaiveDateTime) {
        self.end_date = end_date;
    }

    pub fn details(&self) -> &Vec<TeamMatch> {
        &self.details
    }

    pub fn set_details(&mut self, details: Vec<TeamMatch>) {
        self.details = details;
    }
}
