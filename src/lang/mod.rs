use rand::seq::SliceRandom;
use rust_embed::RustEmbed;
use serde::Deserialize;
use serde_json::from_str;
use std::{error::Error, ffi::OsString, fs, io, path::PathBuf};

#[derive(RustEmbed)]
#[folder = "resources/runtime"]
struct Resources;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct Language {
    name: String,

    words: Vec<String>,
}

impl Language {
    pub fn new(file_name: String) -> Self {
        read_language_from_file(format!("{}.json", file_name)).unwrap()
    }

    pub fn get_random(&self, num: usize) -> Vec<String> {
        let mut rng = &mut rand::thread_rng();

        self.words.choose_multiple(&mut rng, num).cloned().collect()
    }
}

fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("Failed to find config directory.")
        .join("ferristype")
}

fn language_dir() -> PathBuf {
    config_dir().join("words")
}

fn read_language_from_file(file_name: String) -> Result<Language, Box<dyn Error>> {
    let file = fs::read(language_dir().join(&file_name))
        .ok()
        .or_else(|| Resources::get(&format!("words/{}", &file_name)).map(|f| f.data.into_owned()))
        .unwrap();
    let file_as_str = String::from_utf8(file).expect("Unable to interpret file as a string");
    let lang = from_str(&file_as_str).expect("Unable to deserialize language json");
    Ok(lang)
}

pub fn list_installed_language() -> io::Result<Vec<OsString>> {
    Ok(language_dir()
        .read_dir()?
        .filter_map(Result::ok)
        .map(|e| e.file_name())
        .collect())
}
