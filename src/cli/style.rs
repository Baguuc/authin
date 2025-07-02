pub fn print_error(msg: impl std::fmt::Display, error: impl std::fmt::Display) {
    use colored::Colorize; 
    
    println!("{} {} {}", "error:".red(), msg, error);
}

pub fn print_ok(msg: impl std::fmt::Display) {
    use colored::Colorize;

    println!("{} {}", "ok:".green(), msg);
}

pub fn print_header(msg: impl std::fmt::Display) {
    use colored::Colorize;

    println!("\n{}", format!("{} {}", "---".yellow(), msg).bold());
}
