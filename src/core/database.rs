use serde::{Deserialize, Serialize};

use super::data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    projects: Vec<Project>,
}

impl Database {
    pub fn open_or_create_new() -> Database {
        let mut database: Database = Database {
            projects: Vec::new(),
        };
        database = data::read::read_from_file(database);
        return database;
    }

    pub fn does_project_already_exist(&self, project: &str) -> bool {
        return self.projects.iter().any(|p| p.name == project);
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn add_feature_to_project(&mut self, project: &String, feature: Feature) {
        //check if the project already exists
        if let Some(existing_project) = self.projects.iter_mut().find(|p| p.name == *project) {
            existing_project.features.push(feature);
            return;
        }
        eprintln!("Could not add the feature. Could not find the project. :(")
    }

    pub fn get_project(&mut self, project_name: &str) -> Option<&Project> {
        return self.projects.iter().find(|p| p.name == project_name);
    }

    pub fn get_projects(self) -> Vec<Project> {
        return self.projects;
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum TimepointType {
    START,
    STOP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timepoint {
    tp: std::time::SystemTime,
    tp_type: TimepointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    feature_name: String,
    timeoints: Vec<Timepoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    features: Vec<Feature>,
}

impl Project {
    pub fn create_project(project_name: String, _details: Option<String>) -> Self {
        //create not-featured feature to track non feature specific time.
        let not_featured = Feature {
            feature_name: "Not Featured".to_string(),
            timeoints: Vec::new(),
        };

        let the_project = Project {
            name: project_name.clone(),
            features: vec![not_featured],
        };
        return the_project;
    }

    pub fn create_feature(feature_name: &String) -> Feature {
        let feature = Feature {
            feature_name: feature_name.clone(),
            timeoints: Vec::new(),
        };
        return feature;
    }

    pub fn get_features(self) -> Vec<Feature> {
        return self.features;
    }

    pub fn get_project_info(&self) -> &String {
        return &self.name;
    }
}

impl Feature {
    pub fn get_name(&self) -> &String {
        return &self.feature_name;
    }

    pub fn get_time_spent_minutes(&self) -> String {
        return "0 min".to_string();
    }
}
