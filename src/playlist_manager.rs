use std::{ffi::OsString, fmt::Display};

use crate::playlist::Playlist;

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
    config: OsString,
    playlists: Vec<Playlist>,
}

impl PlaylistManager {
    pub fn load(config: OsString) -> Result<Self, Box<dyn std::error::Error>> {
        let mut playlists = vec![];

        let files = std::fs::read_dir(&config)?;

        for file in files {
            let file = file?;
            /*let name = match file.file_name().into_string() {
                Ok(name) => name,
                Err(name) => return Err(Box::new(PlaylistManagerError::InvalidPlaylistName(name))),
            };*/
            let playlist = Playlist::load(&file.path().into_os_string())?;
            playlists.push(playlist);
        }

        Ok(Self { config, playlists })
    }

    pub fn playlists(&self) -> &Vec<Playlist> {
        &self.playlists
    }

    /// Returns all playlist names
    pub fn names(&self) -> Vec<&str> {
        self.playlists.iter().map(|p| {p.name()}).collect()
    }
}
