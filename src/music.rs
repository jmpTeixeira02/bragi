use std::{collections::HashMap, fs, io, path::PathBuf};

enum Genre {
    Rock,
    Metal,
    Pop,
}

struct Artist {
    name: String,
    songs: Vec<Song>,
}

struct Song {
    title: String,
    artist: Box<Artist>,
    genre: Genre,
    year: u8,

    path: String,
}

struct Library {
    songs: HashMap<Artist, Vec<Song>>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            songs: HashMap::new(),
        }
    }

    pub fn load(dir: PathBuf) -> io::Result<()> {
        // Should contain dir with the artist name and files inside for each song
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
        }
    }
}
