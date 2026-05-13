use colored::Colorize;

pub fn success(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn error(msg: &str) {
    eprintln!("{}", format!("Error: {}", msg).red().bold());
}

pub fn hint(msg: &str) {
    eprintln!("{}", format!("Hint: {}", msg).blue());
}

pub fn info(msg: &str) {
    println!("{}", msg);
}

pub fn list_item(name: &str, is_active: bool) {
    if is_active {
        println!("  {} {} {}", "*".green().bold(), name.bold(), "(active)".green());
    } else {
        println!("    {}", name);
    }
}