use crate::core::{
    data,
    database::{self, Project},
};

pub fn create_project(project_name: &String, details: Option<String>) {
    let mut db = database::Database::open_or_create_new();
    if !db.does_project_already_exist(&project_name) {
        let proj = database::Project::create_project(project_name.clone(), details);
        db.add_project(proj);
        println!("{project_name} added to the database");
        data::write::write_to_file(&db);
        return;
    }
    eprintln!("{project_name} already exists in the database");
}

pub fn create_feature(project_name: &String, feature_name: &String) {
    let mut db = database::Database::open_or_create_new();
    if db.does_project_already_exist(&project_name) {
        let feature = Project::create_feature(feature_name);
        db.add_feature_to_project(&project_name, feature);
        println!("{feature_name} added to the project {project_name}");
        data::write::write_to_file(&db);
        return;
    }
    eprintln!("{project_name} does not exists.");
    eprintln!("\nCreate the project first: ");
    eprintln!("    timetrustings project add {project_name}");
}
