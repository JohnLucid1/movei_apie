extern crate diesel;
extern crate dotenvy;

mod args;
mod db;
mod models;
mod ops;
mod schema;


use ops::video_ops::handle_video_command;
use args::{EntityType, RustflixArgs};
use clap::Parser;

fn main() {
    let args = RustflixArgs::parse();    
    
    match args.entity_type {
        EntityType::Video(video) => handle_video_command(video)
    }
}
