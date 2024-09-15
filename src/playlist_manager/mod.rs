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

pub struct PlaylistManager {
    songs_meta: OsString,
    playlists_meta: OsString,
    playlists: Vec<Playlist>,
}

impl PlaylistManager {
    pub fn load(songs_meta: OsString, playlists_meta: OsString) -> Result<Self, Box<dyn std::error::Error>> {
        let mut playlists = vec![];

        let path = PathBuf::from(&playlists_meta);
        let files = std::fs::read_dir(&path)?;

        for file in files {
            let file = file?;
            let playlist = Playlist::load(&file.path().into_os_string())?;
            playlists.push(playlist);
        }

        Ok(Self {
            songs_meta,
            playlists_meta,
            playlists,
        })
    }

    pub fn playlists(&self) -> &Vec<Playlist> {
        &self.playlists
    }

    pub fn playlists_mut(&mut self) -> &mut Vec<Playlist> {
        &mut self.playlists
    }

    /// Returns all playlist names
    pub fn names(&self) -> Vec<&str> {
        self.playlists.iter().map(|p| p.name()).collect()
    }

    /// Returns a vector of all the song saved in any playlist.
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
        // For any song still in a playslist make sure that its meta file
        // exits
        let songs = self.songs();
        let mut songs_meta = PathBuf::from(&self.songs_meta);
        for song in songs {
            let file_name = crate::file_name_from_song(song);
            songs_meta.push(file_name);
            let _ = std::fs::write(
                songs_meta.as_path(),
                serde_json::to_string(song.details()).unwrap(),
            );
            songs_meta.pop();
        }

        // For any playlist make sure its meta file exists
        let mut playlist_meta = PathBuf::from(&self.playlists_meta);
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
