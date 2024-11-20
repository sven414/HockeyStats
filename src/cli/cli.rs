pub fn validate_id(args: &[String]) -> String {
    if args.len() < 2 {
        eprintln!("Fel: Du måste ange ett femsiffrigt nummer som argument.");
        std::process::exit(1);
    }
    let id = &args[1];
    if id.len() != 5 || !id.chars().all(|c| c.is_digit(10)) {
        eprintln!("Fel: Argumentet måste vara ett femsiffrigt nummer.");
        std::process::exit(1);
    }
    id.clone()
}
