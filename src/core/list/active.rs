use crate::core::database;

pub fn get_active() {
    let mut db = database::Database::open_or_create_new();
    for (project, feature) in db.get_all_features() {
        println!("{project:?} -> {feature:?}");
    }
}

pub fn list_active() {
    let mut db = database::Database::open_or_create_new();
    for (project, feature) in db.get_all_features() {
        println!("{project:?} -> {feature:?}");
    }
}
