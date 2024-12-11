pub fn validate_id(args: &[String]) -> String {
    if args.len() < 2 {
        print_help_and_exit(); // Skriv ut hjälp om inga argument anges.
    }

    let arg = &args[1];
    if arg == "--help" {
        print_help_and_exit();
    } else if arg == "--version" {
        print_version_and_exit();
    }

    // Validera om det är ett femsiffrigt nummer.
    if arg.len() != 5 || !arg.chars().all(|c| c.is_digit(10)) {
        eprintln!("Fel: Argumentet måste vara ett femsiffrigt nummer.");
        std::process::exit(1);
    }
    arg.clone()
}

fn print_help_and_exit() {
    println!("Användning: hockeystats <ID>");
    println!("Ett CLI-verktyg för att analysera hockeystatistik.");
    println!();
    println!("Flaggor:");
    println!("  --help       Visar denna hjälptext.");
    println!("  --version    Visar programversionen.");
    std::process::exit(0);
}

fn print_version_and_exit() {
    println!("hockeystats version {}", env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}
