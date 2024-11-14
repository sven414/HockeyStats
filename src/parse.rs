use regex::Regex;
use scraper::{Html, Selector};
use crate::models::game::Game;
use crate::game_factory::create_game;

/// Extraherar och formaterar tiden från ett tidfält, tar bort datum om det finns.
fn extract_time(current_date: &str, time_field: &str) -> String {
    if time_field.contains('-') {
        time_field.replace(current_date, "").split_whitespace().last().unwrap_or("").to_string()
    } else {
        time_field.to_string()
    }
}

/// Parsar ett resultatfält till två mål-siffror, `goal_for` och `goal_against`.
fn parse_result(result: &str) -> (Option<u8>, Option<u8>) {
    let separator_re = Regex::new(r"[-–]").unwrap();

    if !result.is_empty() && separator_re.is_match(result) {
        let parts: Vec<&str> = separator_re.split(result).collect();
        (
            parts[0].trim().parse::<u8>().ok(),
            parts[1].trim().parse::<u8>().ok()
        )
    } else {
        (None, None)
    }
}

/// Extraherar datum och tid från de första två fälten i en rad och uppdaterar `current_date` vid behov.
fn extract_date_and_time(first_field: &str, second_field: &str, current_date: &mut String) -> (String, String) {
    if first_field.contains('-') {
        *current_date = first_field.to_string();
        (current_date.clone(), extract_time(current_date, second_field))
    } else {
        (current_date.clone(), extract_time(current_date, first_field))
    }
}

/// Parsar lagsträngen och returnerar en tuple med `home_team` och `away_team`.
fn parse_teams(teams: &str) -> (String, String) {
    let teams_split: Vec<&str> = teams.split(" - ").collect();
    if teams_split.len() == 2 {
        (teams_split[0].trim().to_string(), teams_split[1].trim().to_string())
    } else {
        ("".to_string(), "".to_string())
    }
}

/// Formaterar en sträng så att varje ord börjar med stor bokstav och resten är små.
fn format_title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Extraherar namnet på ligan från `<title>`-taggen i HTML och formaterar det med stor bokstav i början av varje ord.
fn extract_league(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("title").unwrap();

    if let Some(title_element) = document.select(&title_selector).next() {
        let title_text = title_element.text().collect::<Vec<_>>().join("");
        // Dela på '|' och trimma resultatet
        let parts: Vec<&str> = title_text.split('|').collect();
        if let Some(league) = parts.get(0) {
            return Some(format_title_case(league.trim()));
        }
    }
    None
}

/// Extraherar den aktuella säsongen från `<option>`-taggen med `selected="selected"`.
fn extract_season(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let option_selector = Selector::parse("option[selected]").unwrap();

    if let Some(option_element) = document.select(&option_selector).next() {
        return Some(option_element.text().collect::<Vec<_>>().join("").trim().to_string());
    }
    None
}

/// Huvudfunktionen som parser matcher och skapar `Game`-objekt.
pub fn parse_matches(html: &str) -> Vec<Game> {
    let document = Html::parse_document(&html.replace("\u{00A0}", " "));

    let table_selector = Selector::parse("table.tblContent").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    // Anropa subrutiner för att hämta liga och säsong
    let league = extract_league(html).unwrap_or_else(|| "Ingen liga hittad".to_string());
    let season = extract_season(html).unwrap_or_else(|| "Ingen säsong hittad".to_string());

    // Skriv ut liga och säsong för att verifiera resultat
    println!("Liga: {}", league);
    println!("Säsong: {}", season);

    let mut matches = Vec::new();
    let mut current_date = String::new();

    if let Some(table) = document.select(&table_selector).next() {
        for (_index, row) in table.select(&row_selector).enumerate() {
            let cells: Vec<_> = row.select(&cell_selector).collect();

            if cells.len() >= 8 {
                let first_field = cells[0].text().collect::<Vec<_>>().join("").trim().to_string();
                let second_field = cells[1].text().collect::<Vec<_>>().join("").trim().to_string();

                let (date, time) = extract_date_and_time(&first_field, &second_field, &mut current_date);

                let teams = cells[3].text().collect::<Vec<_>>().join("").trim().to_string();
                let result = cells[4].text().collect::<Vec<_>>().join("").trim().to_string();
                let period_result = cells[5].text().collect::<Vec<_>>().join("").trim().to_string();

                let (goal_for, goal_against) = parse_result(&result);

                let period_result = if period_result.is_empty() {
                    None
                } else {
                    Some(period_result)
                };

                let spectators = cells[6].text().collect::<Vec<_>>().join("").trim().parse().ok();
                let venue = cells[7].text().collect::<Vec<_>>().join("").trim().to_string();

                let (home_team, away_team) = parse_teams(&teams);

                let game = create_game(
                    date,
                    time,
                    home_team,
                    away_team,
                    goal_for,
                    goal_against,
                    period_result,
                    spectators,
                    venue,
                    league.clone(),
                    season.clone(),
                );

                matches.push(game.clone());
            }
        }
    } else {
        println!("Tabell med klassen 'tblContent' hittades inte.");
    }

    matches
}
