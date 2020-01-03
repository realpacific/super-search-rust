use std::fs::{File, OpenOptions};
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use serde_json;

use crate::search::Search;

static FILE_NAME: &str = "ssearch.json";

fn get_location() -> Box<PathBuf> {
    Box::new(dirs::home_dir().unwrap().join(".ssearch").join(FILE_NAME))
}

pub trait Persistence<T> {
    fn write(t: &[T]) -> PersistenceResult;
    fn load() -> Vec<T>;
    fn is_already_exits() -> bool;
    fn update(t: Search) -> PersistenceResult;
    fn remove(kw: String) -> PersistenceResult;
}

pub struct FileBasedSearchPersistence;

#[derive(PartialEq)]
pub enum PersistenceResult {
    Created,
    Updated,
    Deleted,
    Error,
    Nothing,
}

impl Persistence<Search> for FileBasedSearchPersistence {
    fn write(t: &[Search]) -> PersistenceResult {
        fs::create_dir(get_location().as_path().parent().unwrap());
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(get_location().as_path());
        if file.is_err() {
            return PersistenceResult::Error;
        }
        let result = serde_json::to_writer(&file.unwrap(), t);
        if result.is_ok() {
            result.unwrap();
            return PersistenceResult::Created;
        }
        return PersistenceResult::Error;
    }

    fn load() -> Vec<Search> {
        let file = File::open(get_location().as_path()).unwrap();
        let read = serde_json::from_reader(BufReader::new(file)).unwrap();
        return serde_json::from_value(read).unwrap();
    }

    fn is_already_exits() -> bool {
        return Path::exists(get_location().as_path().parent().unwrap());
    }

    fn update(t: Search) -> PersistenceResult {
        let mut contents = FileBasedSearchPersistence::load();
        let original_size = contents.len();
        contents.retain(|x| *x != t);
        if contents.len() < original_size {
            return PersistenceResult::Updated;
        }
        contents.push(t);
        FileBasedSearchPersistence::write(&contents);
        return PersistenceResult::Created;
    }

    fn remove(kw: String) -> PersistenceResult {
        let mut contents = FileBasedSearchPersistence::load();
        let original_size = contents.len();
        contents.retain(|x| x.keyword != kw);
        if original_size == contents.len() {
            return PersistenceResult::Nothing;
        }
        FileBasedSearchPersistence::write(&contents);
        return PersistenceResult::Deleted;
    }
}
