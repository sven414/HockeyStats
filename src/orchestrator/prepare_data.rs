use crate::data_handling::fetch::fetch_html;
use crate::data_handling::parse::parse_matches;
use crate::models::game::Game;

/// Förbereder data för analys genom att hämta och parsa HTML.
pub async fn prepare_data(id: &str) -> Result<Vec<Game>, String> {
    match fetch_html(id).await {
        Ok(html) => {
            let matches = parse_matches(&html);
            if matches.is_empty() {
                Err("Inga matcher hittades.".to_string())
            } else {
                Ok(matches)
            }
        }
        Err(e) => Err(format!("Fel vid hämtning: {}", e)),
    }
}
