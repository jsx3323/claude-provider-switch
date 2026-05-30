use clap::Parser;
use claude_provider_switch::cli::{Cli, Commands};
use claude_provider_switch::command;
use claude_provider_switch::error::CsError;
use claude_provider_switch::input;
use claude_provider_switch::output;
use claude_provider_switch::store;

fn main() {
    let cli = Cli::parse();

    let result = run(cli);

    if let Err(e) = result {
        output::error(&e.to_string());
        if let Some(h) = e.hint() {
            output::hint(&h);
        }
        std::process::exit(e.exit_code());
    }
}

fn run(cli: Cli) -> Result<(), CsError> {
    match cli.command {
        Commands::List => {
            let project = store::find_project_dir()?;
            command::list::run(&project)
        }
        Commands::Use { name } => {
            let project = store::find_project_dir()?;
            if !store::has_claude_dir(&project) {
                output::warn("当前目录没有 .claude 目录");
                if !input::prompt_create_settings()? {
                    return Err(CsError::NoClaudeDir);
                }
            }
            command::use_profile::run(&name, &project)
        }
        Commands::Add { name, force } => command::add::run(&name, force),
        Commands::Current => {
            let project = store::find_project_dir()?;
            command::current::run(&project)
        }
        Commands::Delete { name, force } => {
            let project = store::find_project_dir()?;
            command::delete::run(&name, force, &project)
        }
        Commands::Diff { name } => {
            let project = store::find_project_dir()?;
            if !store::has_claude_dir(&project) {
                output::warn("当前目录没有 .claude 目录");
                if !input::prompt_create_settings()? {
                    return Err(CsError::NoClaudeDir);
                }
            }
            command::diff::run(&name, &project)
        }
        Commands::Edit { name } => command::edit::run(&name),
    }
}
