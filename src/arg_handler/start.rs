use clap::Args;

#[derive(Debug, Args)]
pub struct StartArgs {
    #[arg(help = "Project to start tracking")]
    project: Option<String>,

    #[arg(help = "Feature to start tracking")]
    #[arg(long = "feature", short = 'f')]
    feature: Option<String>,

    #[arg(long = "time", short = 't')]
    #[arg(help = "Starting project time (e.g. 14:51).")]
    #[arg(default_value = "now")]
    time: String,
}

pub fn handle_start(data: &StartArgs) {
    println!("Start timepoint, {data:?}");
}
