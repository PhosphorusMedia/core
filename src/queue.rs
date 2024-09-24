use crate::{
    playlist_manager::Playlist,
    song::{Song, SongDetails},
};

/// Handles a reproduction queue
pub struct QueueManager {
    songs: Vec<Song>,
    /// The index of the selected song in the queue, e.g. the one that is being
    /// played.
    current: usize,
}

impl Default for QueueManager {
    fn default() -> Self {
        Self {
            songs: vec![],
            current: 0,
        }
    }
}

impl QueueManager {
    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }

    /// Returns all the songs in the queue
    pub fn songs(&self) -> Vec<&Song> {
        self.songs.iter().map(|song| song).collect()
    }

    /// Returns a list holding the name of songs in the queue
    pub fn songs_names(&self) -> Vec<&str> {
        self.songs
            .iter()
            .map(|song| song.details().name())
            .collect()
    }

    /// Returns a list holding details about songs in the queue
    pub fn details(&self) -> Vec<&SongDetails> {
        self.songs.iter().map(|song| song.details()).collect()
    }

    /// Returns a list holding details about current and next songs
    pub fn pending(&self) -> Vec<&SongDetails> {
        if self.songs.is_empty() {
            return vec![];
        }
        let mut slice = self.songs.as_slice();
        slice = &slice[self.current - 1..];
        slice.iter().map(|song| song.details()).collect()
    }

    /// Adds a song to the end of the queue
    pub fn push(&mut self, song: Song) {
        self.songs.push(song)
    }

    /// Returns the next song in the queue, if there is one
    pub fn next(&mut self) -> Option<&Song> {
        if self.current + 1 < self.songs.len() {
            self.current += 1;
            let song = self.songs.get(self.current);
            return song;
        }

        None
    }

    /// Returns the previous song in the queue, is there is one.
    ///
    /// A call to `next` followed by a call to `previous` will
    /// results in the calls returning the same song, so to actually
    /// go back, `previous` will have to be called another time.
    pub fn previous(&mut self) -> Option<&Song> {
        if self.current > 0 {
            self.current -= 1;
            return self.songs.get(self.current);
        }

        None
    }

    /// Remove a song from the queue based on an index
    pub fn remove(&mut self, index: usize) {
        if index < self.songs.len() {
            self.songs.remove(index);
        }
    }

    /// Removes every song from the queue
    pub fn clear(&mut self) {
        self.songs.clear();
        self.current = 0;
    }

    /// Removes all remaining songs in the queue
    pub fn clear_pending(&mut self) {
        if self.current + 1 < self.songs.len() {
            self.songs.drain(self.current + 1..);
        }
    }

    /// Queue content is set to a playlist. Previous content is
    /// removed. The `index` parameter indicates what's the song
    /// that should be the 'current' one after the queue update.
    /// If the value excedes playlist size, 'current' is set to
    /// the latter, so the queue should look like fully already played.
    ///
    /// #### NOTE
    /// A playlist reference is received, and every song has
    /// to be cloned to be pushed in the queue.
    pub fn set_on_playlist(&mut self, playlist: &Playlist, index: usize) {
        self.clear();
        let song_iter = playlist.songs().as_slice();
        for song in &song_iter[index..playlist.songs().len()] {
            self.push(song.clone());
        }
        for song in &song_iter[..index] {
            self.push(song.clone());
        }

        if index < playlist.songs().len() {
            self.current = 0;
        } else {
            self.current = playlist.songs().len();
        }
    }
}
