use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    // User(UserCommand),
    Video(VideoCommand),
    // View(ViewCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Create(CreateUser),
    Update(UpdateUser),
    Delete(DeleteEntity),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(CreateVideo),

    /// Update an existing video
    Update(UpdateVideo),

    /// Delete a video
    Delete(DeleteEntity),

    /// Show all videos
    Show,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    /// The title of the video to create
    pub title: String,

    /// The description of the video to create
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    /// The id of the video to update
    pub id: i32,

    /// The title of the video
    pub title: String,

    /// The description of the video
    pub description: String,
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub command: ViewSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    /// Create a new view
    Create(CreateView),

    /// Show all views with id numbers for users and videos
    Show,

    /// Show all views with names for users and videos
    ShowPretty,
}

#[derive(Debug, Args)]
pub struct CreateView {
    /// The id of the user who watched the video
    pub user_id: i32,

    /// The id of the video the user watched
    pub video_id: i32,

    /// The time the user watched the video
    pub watch_start: chrono::NaiveDateTime,

    /// How long the user watched the video for
    pub duration: i32,
}
