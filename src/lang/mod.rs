use rand::seq::SliceRandom;
use serde::Deserialize;
use serde_json::from_reader;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Language {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    size: u32,
    words: Vec<String>,
}

impl Language {
    pub fn new(path: &str) -> Self {
        read_language_from_file(path).unwrap()
    }

    pub fn get_random(&self, num: usize) -> Vec<String> {
        let mut rng = &mut rand::thread_rng();

        self.words.choose_multiple(&mut rng, num).cloned().collect()
    }
}

fn read_language_from_file<P: AsRef<Path>>(path: P) -> Result<Language, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Language`.
    let lang = from_reader(reader)?;

    Ok(lang)
}
