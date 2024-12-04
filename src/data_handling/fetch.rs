/// Fetches HTML content from a specified URL.
///
/// This asynchronous function takes an ID, constructs a URL using that ID, and fetches the HTML content
/// from the constructed URL. If the request is successful, it returns the HTML content as a `String`.
/// If the request fails, it returns an error.
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
