use playlist_manager::Playlist;

pub mod playlist_manager;
pub mod queue;
pub mod song;

/// Returnes the name that can represent the provided song
/// 
/// # Example
/// A song called `song` made by `author` will be associated
/// to a file called `song--author.json`. Everything is lowercase
/// and spaces are replaced by `_`.
pub fn file_name_from_song(song: &song::Song) -> String {
    let details = song.details();
    let name = details.name();
    let artist = details.artist().unwrap_or("");
    format!("{}--{}.json", name, artist)
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
    format!("{}.json", playlist.name())
        .to_lowercase()
        .replace(" ", "_")
}
