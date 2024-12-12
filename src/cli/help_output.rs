pub fn print_help_and_exit() {
    println!("Usage: hockeystats <ID> [FLAGS]");
    println!("Analyze hockey statistics based on match data.");
    println!();
    println!("Arguments:");
    println!("  <ID>        A five-digit series number from the league's URL.");
    println!();
    println!("Flags:");
    println!("  -w, --win        Analyze winning streaks. (Default)");
    println!("  -l, --lose       Analyze losing streaks.");
    println!("  -t, --allteams   Show longest streaks for all teams.");
    println!("  -wl, -w -l, --win --lose");
    println!("                   Analyze both winning and losing streaks.");
    println!("  --help           Display this help message and exit.");
    println!("  --version        Display the program version and exit.");
    println!();
    println!("Examples:");
    println!("  Analyze only winning streaks (default):");
    println!("    hockeystats 12345");
    println!();
    println!("  Analyze losing streaks:");
    println!("    hockeystats 12345 -l");
    println!();
    println!("  Analyze both winning and losing streaks:");
    println!("    hockeystats 12345 -wl");
    println!();
    println!("  Analyze both streaks and display all teams:");
    println!("    hockeystats 12345 -wlt");
    std::process::exit(0);
}

pub fn print_version_and_exit() {
    println!("hockeystats version {}", env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}
