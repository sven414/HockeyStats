#[derive(Debug, Clone)]
pub struct Game {
    date: String,
    time: String,
    home_team: String,
    away_team: String,
    goal_for: Option<u8>,
    goal_against: Option<u8>,
    period_result: Option<String>,
    match_id: Option<String>,
    spectators: Option<u32>,
    venue: String,
    league: String,
    season: String,
}

impl Game {
    // Konstruktor
    pub fn new(
        date: String,
        time: String,
        home_team: String,
        away_team: String,
        goal_for: Option<u8>,
        goal_against: Option<u8>,
        period_result: Option<String>,
        match_id: Option<String>,
        spectators: Option<u32>,
        venue: String,
        league: String,
        season: String,
    ) -> Self {
        Game {
            date,
            time,
            home_team,
            away_team,
            goal_for,
            goal_against,
            period_result,
            match_id,
            spectators,
            venue,
            league,
            season,
        }
    }

    // Getter-metoder
    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn time(&self) -> &str {
        &self.time
    }

    pub fn home_team(&self) -> &str {
        &self.home_team
    }

    pub fn away_team(&self) -> &str {
        &self.away_team
    }

    pub fn goal_for(&self) -> Option<u8> {
        self.goal_for
    }

    pub fn goal_against(&self) -> Option<u8> {
        self.goal_against
    }

    pub fn period_result(&self) -> Option<&str> {
        self.period_result.as_deref()
    }

    pub fn match_id(&self) -> Option<&str> {
        self.match_id.as_deref()
    }

    pub fn spectators(&self) -> Option<u32> {
        self.spectators
    }

    pub fn venue(&self) -> &str {
        &self.venue
    }

    pub fn league(&self) -> &str {
        &self.league
    }

    pub fn season(&self) -> &str {
        &self.season
    }
}
