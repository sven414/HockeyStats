use super::help_output;

pub fn validate_args(args: &[String]) -> (String, bool, bool, bool) {
    if args.len() < 2 {
        help_output::print_help_and_exit();
    }

    let mut analyze_win = true;
    let mut analyze_loss = false;
    let mut show_all_teams = false;

    for arg in &args[2..] {
        if arg.starts_with('-') && !arg.starts_with("--") {
            for flag in arg.chars().skip(1) {
                match flag {
                    'w' => analyze_win = true,
                    'l' => analyze_loss = true,
                    't' => show_all_teams = true,
                    _ => {
                        eprintln!("Error: Unknown flag '-{}'.", flag);
                        std::process::exit(1);
                    }
                }
            }
        } else if arg.starts_with("--") {
            match arg.as_str() {
                "--win" => analyze_win = true,
                "--lose" => analyze_loss = true,
                "--allteams" => show_all_teams = true,
                "--help" => {
                    help_output::print_help_and_exit();
                    unreachable!();
                }
                "--version" => {
                    help_output::print_version_and_exit();
                    unreachable!();
                }
                _ => {
                    eprintln!("Error: Unknown flag '{}'.", arg);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error: Unknown argument '{}'.", arg);
            std::process::exit(1);
        }
    }

    let id = args[1].clone();
    if id.len() != 5 || !id.chars().all(|c| c.is_digit(10)) {
        eprintln!("Error: ID must be a five-digit number.");
        std::process::exit(1);
    }

    (id, analyze_win, analyze_loss, show_all_teams)
}
