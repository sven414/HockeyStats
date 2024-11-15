use regex::Regex;
use crate::enums::MatchType;

pub fn classify_match_type(period_result: &str) -> MatchType {
    let dash_count = Regex::new(r"[-–]").unwrap().find_iter(period_result).count();

    match dash_count {
        3 => MatchType::Regular,
        4 => MatchType::Overtime,
        5 => MatchType::Shootout,
        _ => MatchType::Overtime,
    }
}
