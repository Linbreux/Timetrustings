use clap::Args;

#[derive(Debug, Args)]
pub struct StopArgs {
    #[arg(help = "Project to stop tracking")]
    project: Option<String>,

    #[arg(long = "time", short = 't')]
    #[arg(help = "Stopping project time (e.g. 14:51).")]
    #[arg(default_value = "now")]
    time: String,
}

pub fn handle_stop(data: &StopArgs) {
    println!("stop timepoint, {data:?}");
}
