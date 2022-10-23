use std::{ffi::OsString, time::Duration};

use serde::{Deserialize, Serialize};

/// Represents a song. A Song is any file which can
/// be reproduced.
#[derive(Deserialize, Serialize)]
pub struct Song {
    /// Position of the file in the file system
    #[serde(skip)]
    path: OsString,
    #[serde(skip)]
    details_path: OsString,

    #[serde(rename = "path")]
    path_string: String,
    #[serde(rename = "details_path")]
    details_path_string: String,

    /// Detailed information about the song
    #[serde(skip)]
    details: SongDetails,
}

impl Song {
    pub fn load(mut self) -> Self {
        self.path = OsString::from(&self.path_string);
        self.details_path = OsString::from(&self.details_path_string);
        self.details = SongDetails::load(&self.details_path);
        self
    }

    /// Returns the position of the song in
    /// the file system
    pub fn path(&self) -> &OsString {
        &self.path
    }

    pub fn details(&self) -> &SongDetails {
        &self.details
    }

    pub fn details_mut(&mut self) -> &mut SongDetails {
        &mut self.details
    }
}

impl PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Song {}

/// Holds detailed information about
/// a song.
#[derive(Deserialize, Serialize)]
pub struct SongDetails {
    name: Option<String>,
    artist: Option<String>,
    year: Option<u16>,
    duration: Option<Duration>,
}

impl Default for SongDetails {
    fn default() -> Self {
        Self {
            name: None,
            artist: None,
            year: None,
            duration: None,
        }
    }
}

impl SongDetails {
    /// Loads song details from a config file
    pub fn load(details: &OsString) -> Self {
        let file = match std::fs::read_to_string(details) {
            Ok(file) => file,
            Err(_err) => {
                return SongDetails::default();
            }
        };

        let details: SongDetails = match serde_json::from_str(&file) {
            Ok(details) => details,
            Err(_err) => {
                return SongDetails::default();
            }
        };

        details
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }

    pub fn set_artist(&mut self, artist: &str) {
        self.artist = Some(String::from(artist));
    }

    pub fn set_year(&mut self, year: u16) {
        self.year = Some(year);
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    pub fn name(&self) -> Option<&str> {
        if let Some(name) = &self.name {
            return Some(&name[..]);
        }
        None
    }

    pub fn artist(&self) -> Option<&str> {
        if let Some(artist) = &self.artist {
            return Some(&artist[..]);
        }
        None
    }

    pub fn year(&self) -> Option<u16> {
        self.year
    }

    pub fn duration(&self) -> Option<&Duration> {
        self.duration.as_ref()
    }
}
