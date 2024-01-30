use crate::core;
use clap::Args;

#[derive(Debug, Args)]
pub struct ActiveArgs {}

pub fn handle_active(_data: &ActiveArgs) {
    core::list::active::list_active();
}
