use crate::arg_handler::{active, project, start, stop};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "timetrustings")]
#[command(author = "Linbreux <linbreux@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Advance time tracking", long_about = None)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start tracking a project
    Start(start::StartArgs),
    /// Stop tracking a project
    Stop(stop::StopArgs),
    /// project commands
    #[command(subcommand)]
    Project(project::ProjectCommands),
    /// List active projects
    Active(active::ActiveArgs),
}

fn handle_command(command: &Commands) {
    match &command {
        Commands::Start(data) => {
            start::handle_start(&data);
        }
        Commands::Stop(data) => {
            stop::handle_stop(&data);
        }
        Commands::Project(command) => {
            project::handle_command(&command);
        }
        Commands::Active(data) => active::handle_active(data),
    }
}

// Parsing arguments
pub fn parse() {
    let cli = Cli::parse();

    handle_command(&cli.command);
}
