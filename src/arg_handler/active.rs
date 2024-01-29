use crate::core;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ActiveArgs {}

pub fn handle_active(data: &ActiveArgs) {
    core::list::active::list_active();
    println!("active: {:?}", data);
}
