use std::{collections::HashMap, ffi::OsString, fmt::Display, path::PathBuf};

use chrono::Utc;

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

pub const ALL_SONGS: &'static str = "All songs";

pub struct PlaylistManager {
    songs_meta: OsString,
    playlists_meta: OsString,
    playlists: Vec<Playlist>,
}

impl PlaylistManager {
    pub fn load(songs_meta: OsString, playlists_meta: OsString) -> Result<Self, Box<dyn std::error::Error>> {
        let mut playlists = vec![];

        let files = std::fs::read_dir(&playlists_meta)?;

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

    /// Ensures that basic playlists are loaded and creates them if they don't
    /// already exists. Basic playlists are: playlists of all downloaded songs
    /// (named `All songs`)
    pub fn ensure_basics(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.names().contains(&ALL_SONGS) {
            let mut playlist = Playlist::new(ALL_SONGS, Utc::now());
            // Loads all downloaded songs and their metadata
            let songs_meta = std::fs::read_dir(&self.songs_meta)?;
            for song_meta in songs_meta {
                if let Ok(song_meta) = song_meta {
                    let song = Song::load(serde_json::from_str(
                        &std::fs::read_to_string(song_meta.path())?
                    )?);
                    playlist.add(song);
                }
            }
            self.playlists.push(playlist);
        }

        Ok(())
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

    /// Addds `song` to the playlist named `playlist`, but only if this exits.
    /// Nothing is done otherwise.
    pub fn add_to(&mut self, song: Song, playlist: &str) {
        for pl in &mut self.playlists {
            if pl.name() == playlist {
                pl.add(song);
                break;
            }
        }
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
