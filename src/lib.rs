use playlist_manager::Playlist;
use song::Song;
use sanitise_file_name::sanitise;

pub mod playlist_manager;
pub mod plugin_manager;
pub mod queue;
pub mod song;

/// Returnes the name that can represent the provided song. NO EXTENSION!
///
/// # Example
/// A song called `song` made by `author` will be associated
/// to a file called `song--author`. Everything is lowercase
/// and spaces are replaced by `_`.
pub fn file_name_from_song(song: &song::Song) -> String {
    let details = song.details();
    let name = sanitise(details.name());
    let artist = sanitise(details.artist().unwrap_or(""));
    format!("{}--{}", name, artist)
        .to_lowercase()
        .replace(" ", "_")
}

/// As `file_name_from_song` by takes in input only name and artist of that song.
pub fn file_name_from_basics(song_name: &str, song_artist: &str) -> String {
    format!("{}--{}", sanitise(song_name), sanitise(song_artist))
        .to_lowercase()
        .replace(" ", "_")
}

/// Returnes the name that can represent the provided playlist
///
/// # Example
/// A playlist called `playlist` will be associated to a file called
/// `playlist.json`. Everything is lowercase and spaces are replaced by
/// `_`.
pub fn file_name_from_playlist(playlist: &Playlist) -> String {
    format!("{}.json", sanitise(playlist.name()))
        .to_lowercase()
        .replace(" ", "_")
}

/// This enum should be used to pass around the app tranking information for
/// downloads.
pub enum TrackInfo {
    /// The download for the song has been registered, but not yet started
    New(Song),
    /// The download for the song is started
    Started(Song),
    /// The download for the song has made progresses
    Progress(Song, f32),
    /// The download for the song is terminated. Terminated should mean that the
    /// file is totally downloaded and the raw media file is ready to be read
    Finished(Song),
    /// The download for the song is failed.
    Failed(Song, String),
}
