use std::{
    cell::RefCell,
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    rc::Rc,
};

const AUDIO_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a", "aac", "wma", "opus"];

struct Artist {
    name: String,
    songs: Vec<Rc<RefCell<Song>>>,
}

struct Song {
    title: String,
    artist: Rc<RefCell<Artist>>,

    path: PathBuf,
}

struct Library {
    collection: HashMap<String, Rc<RefCell<Artist>>>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            collection: HashMap::new(),
        }
    }

    pub fn load(&mut self, dir: PathBuf) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let artist_name = path
                .file_name()
                .and_then(|e| e.to_str())
                .unwrap_or("UNKNOWN")
                .to_string();

            let artist = Rc::new(RefCell::new(Artist {
                name: artist_name.clone(),
                songs: Vec::new(),
            }));

            for song in fs::read_dir(&path)? {
                let song = song?;
                let song_path = song.path();

                if !is_audio_file(&song_path) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("expected an audio file, found: {}", song_path.display()),
                    ));
                }

                let song = Rc::new(RefCell::new(Song {
                    title: song_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("UNKNOWN")
                        .to_string(),
                    artist: Rc::clone(&artist),
                    path: song_path,
                }));

                artist.borrow_mut().songs.push(Rc::clone(&song));
            }

            self.collection.insert(artist_name, artist);
        }
        Ok(())
    }
}

fn is_audio_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| AUDIO_EXTENSIONS.iter().any(|e1| e.eq_ignore_ascii_case(e1)))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    fn setup() -> PathBuf {
        let dir = std::env::temp_dir();
        fs::remove_dir_all(&dir);

        let artist_one = dir.join("Artist One");
        let artist_two = dir.join("Artist Two");
        fs::create_dir_all(&artist_one).unwrap();
        fs::create_dir_all(&artist_two).unwrap();

        File::create(artist_one.join("song1.mp3")).unwrap();
        File::create(artist_one.join("song2.mp3")).unwrap();
        File::create(artist_two.join("song3.mp3")).unwrap();
        dir
    }

    #[test]
    fn should_load_artist_and_song() {
        let dir = setup();
        let mut lib = Library::new();
        lib.load(dir);
        assert_eq!(lib.collection.len(), 2)
    }

    #[test]
    fn should_error_on_non_file_inside_artist() {}
}
