use std::ffi::OsString;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::song::Song;

/// A playlist is just a collection of songs
/// identified by a name.
#[derive(Deserialize, Serialize)]
pub struct Playlist {
    name: String,
    creation_date: DateTime<Utc>,
    songs: Vec<Song>,
}

impl Playlist {
    /// Creates a new empty playlist with
    /// name `name`
    pub fn new(name: &str, creation_date: DateTime<Utc>) -> Self {
        Self {
            name: String::from(name),
            creation_date,
            songs: vec![],
        }
    }

    pub fn load(path: &OsString) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(path)?;
        let mut playlist: Playlist = serde_json::from_str(&file)?;
        let mut songs = vec![];
        for song in playlist.songs.into_iter() {
            let detailed_song = song.load();
            songs.push(detailed_song);
        }
        playlist.songs = songs;
        Ok(playlist)
    }

    /// Returns playlist name
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn creation_date(&self) -> &DateTime<Utc> {
        &self.creation_date
    }

    /// Returns playlist songs
    pub fn songs(&self) -> &Vec<Song> {
        &self.songs
    }

    pub fn songs_mut(&mut self) -> &mut Vec<Song> {
        &mut self.songs
    }

    /// Adds `song` to the playlist
    pub fn add(&mut self, song: Song) {
        self.songs.push(song);
    }

    /// Adds `song` to the playlist, but only
    /// if wasn't previously inserted
    pub fn add_unique(&mut self, song: Song) {
        for item in &self.songs {
            if *item == song {
                return;
            }
        }
        self.songs.push(song);
    }

    /// Removes `song` from the playlist
    pub fn remove(&mut self, song: &Song) {
        if let Some(index) = self.find(song) {
            self.songs.remove(index);
        }
    }

    /// Returns the position of `song` in the
    /// playlist.
    fn find(&mut self, song: &Song) -> Option<usize> {
        for (index, item) in self.songs.iter().enumerate() {
            if *item == *song {
                return Some(index);
            }
        }

        return None;
    }
}
