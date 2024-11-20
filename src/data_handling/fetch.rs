
pub async fn fetch_html(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://stats.swehockey.se/ScheduleAndResults/Schedule/{}", id);
    println!("Hämtar data från: {}", url);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        // Skicka tillbaka ett generiskt felmeddelande som Box<dyn std::error::Error>
        Err(format!("Misslyckades med att hämta sidan. Statuskod: {}", response.status()).into())
    }
}
