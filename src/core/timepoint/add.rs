use crate::core::{
    data,
    database::{self, Timepoint, TimepointType},
};

impl Timepoint {
    pub fn add(
        tp: std::time::SystemTime,
        project: &Option<String>,
        feature: &Option<String>,
        timepoint_type: database::TimepointType,
    ) {
        let mut db = database::Database::open_or_create_new();
        let feature_to_use: String;
        let mut project_to_use: String = "".to_string();
        if project.is_none() && feature.is_none() {
            eprintln!("Define at least one project or a feature.");
            return;
        }
        match feature {
            Some(f) => feature_to_use = f.clone(),
            None => feature_to_use = "Not Featured".to_string(),
        }
        match project {
            Some(p) => project_to_use = p.clone(),
            None => {
                // Try finding a unique project bind to the feature.
                let mut matches: u32 = 0;
                for project in db.get_projects() {
                    // check if feature exists in project
                    let feature = project.get_feature(&feature_to_use);
                    match feature {
                        Some(_) => {
                            matches += 1;
                            project_to_use = project.get_project_info().clone();
                        }
                        None => {}
                    }
                }
                if matches != 1 {
                    eprintln!("Did not find a unique project to link the feature to.");
                    return;
                }
            }
        }
        let found_project = db.get_project_mut(&project_to_use);
        match found_project {
            Some(fp) => {
                let found_feature = fp.get_feature_mut(&feature_to_use);
                match found_feature {
                    Some(ff) => {
                        match timepoint_type {
                            TimepointType::START => {
                                if !ff.can_start_timepoint() {
                                    eprintln!("There is still a timepoint in progress.");
                                    return;
                                }
                            }
                            TimepointType::STOP => {
                                if !ff.can_stop_timepoint() {
                                    eprintln!("There is no timepoint active. First start one.");
                                    return;
                                }
                            }
                        };
                        let tp_object = Timepoint {
                            tp,
                            tp_type: timepoint_type,
                        };
                        // add the timepoint
                        ff.add_timepoint(tp_object);
                        data::write::write_to_file(&db);
                        println!("✔ Timepoint added.");
                    }
                    None => {
                        eprintln!("Could not add the timepoint. Could not find the feature.");
                    }
                }
            }
            None => {
                eprintln!("Could not add the timepoint. Could not find the project.");
            }
        }
    }
}
