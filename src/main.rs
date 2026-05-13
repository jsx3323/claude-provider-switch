use clap::Parser;
use claude_switch::cli::{Cli, Commands};
use claude_switch::command;
use claude_switch::output;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::List => command::list::run(),
        Commands::Use { name } => command::use_profile::run(&name),
        Commands::Add { name, force } => command::add::run(&name, force),
        Commands::Current => command::current::run(),
        Commands::Delete { name, force } => command::delete::run(&name, force),
        Commands::Diff { name } => command::diff::run(&name),
    };

    if let Err(e) = result {
        output::error(&e.to_string());
        if let Some(h) = e.hint() {
            output::hint(&h);
        }
        std::process::exit(e.exit_code());
    }
}