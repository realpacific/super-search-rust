use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

use serde_json;

use crate::search::Search;

static FILE_NAME: &str = "data.json";

pub trait Persistence<T> {
    fn write(t: &Vec<T>);
    fn load() -> Vec<T>;
    fn is_already_exits() -> bool;
    fn update(t: Search) -> bool;
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

    fn update(t: Search) -> bool {
        let contents = SearchPersistence::load();
        let mut map = Search::map_keyword_to_self(&contents);
        map.insert(&t.keyword, &t);
        let file = OpenOptions::new().read(true).truncate(true).write(true).open(FILE_NAME);
        let vec = map.values().collect::<Vec<_>>();
        serde_json::to_writer(&file.unwrap(), &vec).unwrap();
        return true;
    }
}
