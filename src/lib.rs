use std::ffi::OsString;

/// Represents a song. A Song is any file which can
/// be reproduced.
pub struct Song {
    /// Position of the file in the file system
    path: OsString,
}

impl Song {
    pub fn new(path: OsString) -> Self {
        Self { path }
    }

    /// Returns the position of the song in
    /// the file system
    pub fn path(&self) -> &OsString {
        &self.path
    }
}

impl PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Song {}

/// A playlist is just a collection of songs
/// identified by a name.
pub struct Playlist {
    name: String,
    songs: Vec<Song>
}

impl Playlist {
    /// Creates a new empty playlist with
    /// name `name`
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            songs: vec![]
        }
    }

    /// Returns playlist name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns playlist songs
    pub fn songs(&self) -> &Vec<Song> {
        &self.songs
    }

    /// Adds `song` to the playlist
    pub fn add(&mut self, song: Song) {
        self.songs.push(song);
    }

    /// Adds `song` to the playlist, but only
    /// if wasn't previously inserted
    pub fn add_unique(&mut self, song: Song) {
        for item in &self.songs {
            if *item == song { return; }
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
            if *item == *song { return Some(index) }
        }

        return None
    }
}