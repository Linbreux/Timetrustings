use crate::core::database;

pub fn list_projects() {
    let db = database::Database::open_or_create_new();
    println!("PROJECTS");
    println!("========");
    for project in db.get_projects() {
        println!(" * {}", project.get_project_info());
        for feature in project.get_features() {
            println!(
                "   - {:<5} -> {}",
                feature.get_time_spent_minutes(),
                feature.get_name(),
            );
        }
        println!("   TOTAL: 0");
    }
}
