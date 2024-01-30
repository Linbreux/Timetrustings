use crate::core::database::{self, Feature, Project};
use colored::Colorize;

pub fn list_active() {
    let db = database::Database::open_or_create_new();
    println!("{}", "ACTIVE FEATURES".bold().green());
    println!("{}", "===============".bold().green());
    let mut active_feature: Vec<(&Project, &Feature)> = Vec::new();
    for (project, feature) in db.get_all_features() {
        // if we can stop the timepoint it's active
        if feature.can_stop_timepoint() {
            active_feature.push((project, feature))
        }
    }

    if active_feature.is_empty() {
        println!("No active features");
        return;
    }

    let mut prev_project: Option<&Project> = None;
    for (project, feature) in active_feature {
        let active_timepoint = feature.get_timepoints().last();
        let tp = active_timepoint.expect("No Timepoints");
        if prev_project.is_none() || !std::ptr::eq(project, prev_project.unwrap()) {
            println!(" {} {}", "★".yellow(), project.get_project_info().yellow());
        }
        println!(
            "    {} {} ➤ {} ({})",
            "•".blue(),
            feature.get_name().blue(),
            tp.get_human_time(),
            tp.timepoint_till_now()
        );
        prev_project = Some(&project);
    }
}
