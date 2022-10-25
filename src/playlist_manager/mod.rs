use std::{collections::HashMap, ffi::OsString, fmt::Display, path::PathBuf};

use crate::song::Song;

pub use self::playlist::Playlist;

mod playlist;

#[derive(Debug)]
pub enum PlaylistManagerError {
    InvalidPlaylistName(OsString),
}

impl Display for PlaylistManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistManagerError::InvalidPlaylistName(name) => {
                writeln!(
                    f,
                    "`{}` isn't a valid name for a playlist",
                    name.to_string_lossy()
                )
            }
        }
    }
}

impl std::error::Error for PlaylistManagerError {}

const PLAYLIST_DATA: &'static str = "playlists";
const SONGS_DATA: &'static str = "songs";

pub struct PlaylistManager {
    base_config_dir: OsString,
    playlists: Vec<Playlist>,
}

impl PlaylistManager {
    pub fn load(base_config_dir: OsString) -> Result<Self, Box<dyn std::error::Error>> {
        let mut playlists = vec![];

        let mut path = PathBuf::from(&base_config_dir);
        path.push(PLAYLIST_DATA);
        let files = std::fs::read_dir(&path)?;

        for file in files {
            let file = file?;
            let playlist = Playlist::load(&file.path().into_os_string())?;
            playlists.push(playlist);
        }

        Ok(Self {
            base_config_dir,
            playlists,
        })
    }

    pub fn playlists(&self) -> &Vec<Playlist> {
        &self.playlists
    }

    /// Returns all playlist names
    pub fn names(&self) -> Vec<&str> {
        self.playlists.iter().map(|p| p.name()).collect()
    }

    fn songs(&self) -> Vec<&Song> {
        let mut songs: HashMap<&OsString, &Song> = HashMap::new();

        for playlist in self.playlists.iter() {
            for song in playlist.songs() {
                songs.insert(song.path(), song);
            }
        }

        songs.values().map(|value| *value).collect()
    }
}

impl Drop for PlaylistManager {
    fn drop(&mut self) {
        let songs = self.songs();
        let mut songs_meta = PathBuf::new();
        songs_meta.push(&self.base_config_dir);
        songs_meta.push(SONGS_DATA);

        for song in songs {
            let file_name = crate::file_name_from_song(song);
            songs_meta.push(file_name);
            let _ = std::fs::write(
                songs_meta.as_path(),
                serde_json::to_string(song.details()).unwrap(),
            );
            songs_meta.pop();
        }

        let mut playlist_meta = PathBuf::new();
        playlist_meta.push(&self.base_config_dir);
        playlist_meta.push(PLAYLIST_DATA);

        for playlist in self.playlists.iter() {
            let file_name = crate::file_name_from_playlist(playlist);
            playlist_meta.push(file_name);
            let _ = std::fs::write(
                playlist_meta.as_path(),
                serde_json::to_string(playlist).unwrap(),
            );
            playlist_meta.pop();
        }
    }
}
