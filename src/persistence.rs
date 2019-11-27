use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json;

use crate::search::Search;

static FILE_NAME: &str = "data.json";

pub trait Persistence<T> {
    fn write(t: &Vec<T>);
    fn load() -> Vec<T>;
    fn is_already_exits() -> bool;
}

pub struct SearchPersistence;

impl Persistence<Search> for SearchPersistence {
    fn write(t: &Vec<Search>) {
        let file = File::create(FILE_NAME);
        serde_json::to_writer(&file.unwrap(), t).unwrap();
    }

    fn load() -> Vec<Search> {
        let file = File::open(FILE_NAME).unwrap();
        let read = serde_json::from_reader(BufReader::new(file)).unwrap();
        return serde_json::from_value(read).unwrap();
    }

    fn is_already_exits() -> bool {
        return Path::exists(Path::new(FILE_NAME));
    }
}
