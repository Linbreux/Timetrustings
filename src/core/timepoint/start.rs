use crate::core::database::{Timepoint, TimepointType};

impl Timepoint {
    pub fn start(tp: std::time::SystemTime, project: &Option<String>, feature: &Option<String>) {
        Timepoint::add(tp, project, feature, TimepointType::START);
    }
}
