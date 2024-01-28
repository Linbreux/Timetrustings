use crate::core::database::Timepoint;
use clap::Args;

#[derive(Debug, Args)]
pub struct StopArgs {
    #[arg(help = "Project to stop tracking")]
    project: Option<String>,

    #[arg(help = "Feature to start tracking")]
    #[arg(long = "feature", short = 'f')]
    feature: Option<String>,

    #[arg(long = "time", short = 't')]
    #[arg(help = "Stopping project time (e.g. 14:51).")]
    #[arg(default_value = "now")]
    time: String,
}

pub fn handle_stop(data: &StopArgs) {
    let tp = std::time::SystemTime::now();
    Timepoint::stop(tp, &data.project, &data.feature);
}
