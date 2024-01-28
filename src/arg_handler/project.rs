use crate::core::{self, create::project};
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    /// list all projects
    List,
    /// Add a new project or feature to an existing project
    Add(AddArgs),
    /// Remove a project
    Del(DelArgs),
}

const ADD_EXAMPLE: &str = "
Examples:
    timetrustings project add <project_to_add>
    timetrustings project add <exisiting_project> --feature <feature_to_add>
";

#[derive(Debug, Args)]
#[command(about = "Add a new project or feature to an existing project")]
#[command(after_help = ADD_EXAMPLE)]
pub struct AddArgs {
    #[arg(help = "Name of the project to add or to append to")]
    #[arg(required = true)]
    project: String,

    #[arg(help = "Feature to add to the project")]
    #[arg(long = "feature", short = 'f')]
    feature: Option<String>,
}

#[derive(Debug, Args)]
pub struct DelArgs {
    #[arg(help = "Name of the project to remove")]
    project: Option<String>,
}

pub fn handle_command(command: &ProjectCommands) {
    match &command {
        ProjectCommands::Add(data) => {
            handle_add(&data);
        }
        ProjectCommands::Del(_data) => {}
        ProjectCommands::List => {
            handle_list();
        }
    }
}

fn handle_add(data: &AddArgs) {
    if data.feature.is_none() {
        // Do not create a feature. Just create the project
        project::create_project(&data.project, None);
        return;
    }
    project::create_feature(&data.project, &data.feature.as_ref().unwrap());
}

fn handle_list() {
    core::list::project::list_projects();
}
