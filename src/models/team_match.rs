use chrono::NaiveDateTime;
use std::fmt;

use crate::enums::{HomeAway, MatchResult, MatchType};


#[derive(Debug, Clone)]
pub struct TeamMatch {
    datetime: NaiveDateTime, // Kombinerar datum och tid
    opponent: String,        // Motståndarlag
    result: MatchResult,     // Vinst/Förlust/Oavgjort
    match_type: MatchType,   // Ordinarie/Övertid/Straffar
    home_away: HomeAway,     // Hemma/Borta
}

impl TeamMatch {
    // Konstruktor
    pub fn new(
        datetime: NaiveDateTime,
        opponent: String,
        result: MatchResult,
        match_type: MatchType,
        home_away: HomeAway,
    ) -> Self {
        TeamMatch {
            datetime,
            opponent,
            result,
            match_type,
            home_away,
        }
    }
}

impl fmt::Display for TeamMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} mot {}, Resultat: {:?}, Typ: {:?}, Plats: {:?}",
            self.datetime, self.opponent, self.result, self.match_type, self.home_away
        )
    }
}

impl TeamMatch {
    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn result(&self) -> MatchResult {
        self.result.clone()
    }

    pub fn opponent(&self) -> String {
        self.opponent.clone()
    }

    pub fn match_type(&self) -> MatchType {
        self.match_type.clone()
    }

    pub fn home_away(&self) -> HomeAway {
        self.home_away.clone()
    }
}
