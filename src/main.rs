use std::env;
mod fetch;
mod models;
mod parse;

#[tokio::main]
async fn main() {
    // Hämta kommandoradsargumentet
    let args: Vec<String> = env::args().collect();

    // Kontrollera att minst ett argument skickades utöver programmets namn
    if args.len() < 2 {
        eprintln!("Fel: Du måste ange ett 5-siffrigt nummer som argument.");
        std::process::exit(1);
    }
    let id = &args[1];

    if id.len() != 5 || !id.chars().all(|c| c.is_digit(10)) {
        eprintln!("Fel: Argumentet måste vara ett 5-siffrigt nummer.");
        std::process::exit(1);
    }

    use parse::parse_matches;

    match fetch::fetch_html(id).await {
        Ok(html) => {
            // Använd parse_matches för att extrahera matcherna från HTML
            let matches = parse_matches(&html);

            // Skriv ut varje match i listan
            if matches.is_empty() {
                println!("Inga matcher hittades.");
            } else {
                for game in matches {
                    println!("{:?}", game);
                }
            }
        },
        Err(e) => eprintln!("Fel vid hämtning: {}", e),
    }
}
