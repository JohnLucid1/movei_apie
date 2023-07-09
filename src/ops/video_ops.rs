use crate::args::{CreateVideo, DeleteEntity, UpdateVideo, VideoCommand, VideoSubcommand};

use crate::db::establish_connection;
use crate::models::{NewVideo, Video};
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;

pub fn handle_video_command(video: VideoCommand) {
    let command = video.command;

    match command {
        VideoSubcommand::Create(video) => create_video(video),
        VideoSubcommand::Update(video) => update_video(video),
        VideoSubcommand::Delete(video) => delete_video(video),
        VideoSubcommand::Show => show_videos(),
    }
}

fn delete_video(video: DeleteEntity) {
    println!("Deleting video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(videos.find(video.id))
        .execute(&mut connection)
        .expect("Couldn't delete video");
}

fn update_video(video: UpdateVideo) {
    println!("Updating video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let mut connection = establish_connection();
    let db_video = Video {
        id: video.id,
        title: video.title,
        description: video.description,
        removed: false,
    };

    diesel::update(videos.find(video.id))
        .set(&db_video)
        .execute(&mut connection)
        .expect("Error updating video");
}

fn create_video(video: CreateVideo) {
    println!("Createing video : {:?}", video);
    use crate::schema::videos::dsl::*;

    let mut connection = establish_connection();
    let new_video = NewVideo {
        title: &video.title,
        description: &video.description,
        removed: false,
    };

    diesel::insert_into(videos)
        .values(&new_video)
        .execute(&mut connection)
        .expect("Error saving video");
}

pub fn show_videos() {
    println!("Showing videos");

    use crate::schema::videos::dsl::*;

    let mut connection = establish_connection();

    let results: Vec<Video> = FilterDsl::filter(videos, removed.eq(false))
        .load::<Video>(&mut connection)
        .expect("error loading videos");

    println!("Displaying {} videos", results.len());
    for video in results {
        println!("{:?}", video)
    }
}
