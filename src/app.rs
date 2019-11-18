use std::collections::HashMap;

use crate::persistence::Persistence;

use super::persistence::SearchPersistence;

use self::super::search::Search;
use prettytable::{Table, Row, Cell};

pub fn run() {
    let input = String::from("so");
    let query = String::from("How to use option in rust");
    if !SearchPersistence::is_already_exits() {
        SearchPersistence::write(&Search::initialize());
    }
    let searches = SearchPersistence::load();
    let mut map = HashMap::new();
    searches.iter().for_each(|x| {
        map.insert(&x.keyword, &x.url);
    });
    if map.contains_key(&input) {
        println!("Search using {}{}", map[&input], query.replace(" ", "+"));
    } else {
        println!("{}", "lol what?");
    }
}

pub fn print() {
    let searches = SearchPersistence::load();
    let mut table = Table::new();
    table.add_row(row!["Keyword", "Description", "Url"]);
    searches.iter().for_each(|x| {
        table.add_row(Row::new(vec![
            Cell::new(&x.keyword),
            Cell::new(&x.description),
            Cell::new(&x.url)
        ]));
    });
    table.printstd();
}
