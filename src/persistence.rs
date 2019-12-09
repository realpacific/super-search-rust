use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

use serde_json;

use crate::search::Search;

static FILE_NAME: &str = "data.json";

pub trait Persistence<T> {
    fn write(t: &[T]) -> PersistenceResult;
    fn load() -> Vec<T>;
    fn is_already_exits() -> bool;
    fn update(t: Search) -> PersistenceResult;
    fn remove(kw: String) -> PersistenceResult;
}

pub struct SearchPersistence;

#[derive(PartialEq)]
pub enum PersistenceResult {
    Created,
    Updated,
    Deleted,
    Loaded,
    Error,
    Nothing,
}

impl Persistence<Search> for SearchPersistence {
    fn write(t: &[Search]) -> PersistenceResult {
        let file = OpenOptions::new().create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(FILE_NAME);
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
        let file = File::open(FILE_NAME).unwrap();
        let read = serde_json::from_reader(BufReader::new(file)).unwrap();
        return serde_json::from_value(read).unwrap();
    }

    fn is_already_exits() -> bool {
        return Path::exists(Path::new(FILE_NAME));
    }

    fn update(t: Search) -> PersistenceResult {
        let mut contents = SearchPersistence::load();
        let original_size = contents.len();
        contents.retain(|x| *x != t);
        if contents.len() < original_size {
            return PersistenceResult::Updated;
        }
        contents.push(t);
        SearchPersistence::write(&contents);
        return PersistenceResult::Created;
    }

    fn remove(kw: String) -> PersistenceResult {
        let mut contents = SearchPersistence::load();
        let original_size = contents.len();
        contents.retain(|x| x.keyword != kw);
        if original_size == contents.len() {
            return PersistenceResult::Nothing;
        }
        SearchPersistence::write(&contents);
        return PersistenceResult::Deleted;
    }
}
