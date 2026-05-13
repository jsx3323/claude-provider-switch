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

pub fn removed(key: &str) {
    println!("  - {} (removed)", key.yellow());
}

pub fn list_item(name: &str, is_active: bool) {
    if is_active {
        println!("  {} {} {}", "*".green().bold(), name.bold(), "(active)".green());
    } else {
        println!("    {}", name);
    }
}

pub fn list_item_missing(name: &str) {
    println!("  {} {} {}", "*".green().bold(), name.bold(), "(active - missing!)".red());
}

pub fn diff_header(current_label: &str, profile_label: &str) {
    println!("--- {}", current_label);
    println!("+++ {}", profile_label);
}

pub fn diff_deleted(line: &str) {
    println!("-{}", line.red());
}

pub fn diff_inserted(line: &str) {
    println!("+{}", line.green());
}

pub fn diff_equal(line: &str) {
    println!(" {}", line);
}
