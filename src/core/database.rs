use humantime;
use std::{
    fmt::{Debug, Display},
    time::UNIX_EPOCH,
};

use chrono;
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

    pub fn add_timepoint(&mut self, project: String, feature: Feature, tp: Timepoint) {
        let project = self.projects.iter_mut().find(|p| p.name == project);
        match project {
            Some(p) => {
                // look for the feature
                let feature = p
                    .features
                    .iter_mut()
                    .find(|f| f.feature_name == feature.feature_name);
                match feature {
                    Some(f) => {
                        f.add_timepoint(tp);
                    }
                    None => {
                        eprintln!("Feature does not exist");
                    }
                }
            }
            None => {
                eprintln!("project does not exists.");
            }
        }
    }

    pub fn add_feature_to_project(&mut self, project: &String, feature: Feature) -> bool {
        //check if the project already exists
        if let Some(existing_project) = self.projects.iter_mut().find(|p| p.name == *project) {
            let exist = existing_project
                .get_features()
                .iter()
                .any(|f| f.feature_name == feature.feature_name);
            if !exist {
                existing_project.features.push(feature);
                return true;
            }
            return false;
        }
        eprintln!("Could not add the feature. Could not find the project. :(");
        return false;
    }

    pub fn get_project_mut(&mut self, project_name: &String) -> Option<&mut Project> {
        return self.projects.iter_mut().find(|p| &p.name == project_name);
    }

    pub fn get_project(&self, project_name: &String) -> Option<&Project> {
        return self.projects.iter().find(|p| &p.name == project_name);
    }

    pub fn get_projects(&self) -> &Vec<Project> {
        return &self.projects;
    }

    pub fn get_projects_mut(self) -> Vec<Project> {
        return self.projects;
    }

    pub fn get_all_features(&self) -> Vec<(&Project, &Feature)> {
        let mut project_features: Vec<(&Project, &Feature)> = Vec::new();
        for project in &self.projects {
            for feature in project.get_features() {
                project_features.push((&project, &feature));
            }
        }
        return project_features;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimepointType {
    START,
    STOP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timepoint {
    pub tp: std::time::SystemTime,
    pub tp_type: TimepointType,
}

impl Timepoint {
    pub fn timepoint_till_now(&self) -> String {
        let tp_duration = self
            .tp
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let till_now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("");
        let duration = till_now - tp_duration;
        let sec_duration = std::time::Duration::new(duration.as_secs(), 0);
        let test = humantime::format_duration(sec_duration);
        test.to_string()
    }

    fn pretty_print_system_time(t: std::time::SystemTime) -> String {
        let dt: chrono::DateTime<chrono::Utc> = t.clone().into();
        let s: String = dt.format("%d-%b-%Y %H:%M:%S").to_string();
        return s;
    }

    pub fn get_human_time(&self) -> String {
        return Self::pretty_print_system_time(self.tp);
    }
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

    pub fn get_feature(&self, feature_name: &str) -> Option<&Feature> {
        return self
            .features
            .iter()
            .find(|f| f.feature_name == feature_name);
    }

    pub fn get_feature_mut(&mut self, feature_name: &str) -> Option<&mut Feature> {
        return self
            .features
            .iter_mut()
            .find(|f| f.feature_name == feature_name);
    }

    pub fn get_features(&self) -> &Vec<Feature> {
        return &self.features;
    }

    pub fn get_features_mut(&mut self) -> &mut Vec<Feature> {
        return &mut self.features;
    }

    pub fn get_project_info(&self) -> &String {
        return &self.name;
    }
}

impl Feature {
    pub fn get_name(&self) -> &String {
        return &self.feature_name;
    }

    pub fn add_timepoint(&mut self, tp: Timepoint) {
        self.timeoints.push(tp);
    }
    pub fn get_timepoints(&self) -> &Vec<Timepoint> {
        return &self.timeoints;
    }

    pub fn get_timepoint_counts(&self) -> (usize, usize) {
        let start_count = self
            .timeoints
            .iter()
            .filter(|tp| matches!(tp.tp_type, TimepointType::START))
            .count();

        let stop_count = self
            .timeoints
            .iter()
            .filter(|tp| matches!(tp.tp_type, TimepointType::STOP))
            .count();
        return (start_count, stop_count);
    }

    pub fn can_start_timepoint(&self) -> bool {
        let (start_count, stop_count) = self.get_timepoint_counts();
        return start_count == stop_count;
    }

    pub fn can_stop_timepoint(&self) -> bool {
        let (start_count, stop_count) = self.get_timepoint_counts();
        return start_count > stop_count;
    }

    pub fn get_time_spent_minutes(&self) -> String {
        return "0 min".to_string();
    }
}
