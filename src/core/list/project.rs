use crate::core::database;
use colored::Colorize;

pub fn list_projects() {
    let db = database::Database::open_or_create_new();
    println!("{}", "PROJECTS".bold().green());
    println!("{}", "========".bold().green());
    for project in db.get_projects() {
        println!(" {} {}", "★".yellow(), project.get_project_info().yellow());
        for feature in project.get_features() {
            println!(
                "   • {:<5} ➤ {}",
                feature.calculate_total_time(),
                feature.get_name().blue(),
            );
        }
        println!("   TOTAL: 0\n");
    }
}
