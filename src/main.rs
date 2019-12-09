#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;

use clap::{App as Clap, Arg, ArgMatches, ErrorKind, SubCommand};
use prettytable::{Cell, Row, Table};
use webbrowser;

use url::{ParseError as UrlError, Url};

use crate::persistence::{Persistence, SearchPersistence, PersistenceResult};
use crate::search::Search;

mod search;
mod persistence;

fn main() {
    let mut clap = Clap::new("Super Search")
        .about("A CLI tool for searching on your favorites websites written in Rust.")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("list")
                .about("List all stored keywords.")
                .alias("ls"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add new query")
                .alias("a")
                .arg(
                    Arg::with_name("kw")
                        .short("k")
                        .help("Keyword to be used when searching")
                        .takes_value(true)
                        .max_values(1)
                        .required(true)
                )
                .arg(Arg::with_name("q")
                    .short("q")
                    .help("Query to be used for building links")
                    .takes_value(true)
                    .max_values(1)
                    .required(true)
                )
                .arg(Arg::with_name("d")
                    .short("d")
                    .help("Describe what the query will do")
                    .takes_value(true)
                    .max_values(100)
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("del")
                .about("Delete Keyword")
                .alias("del")
                .arg(
                    Arg::with_name("kw")
                        .short("k")
                        .help("Keyword to be deleted")
                        .takes_value(true)
                        .max_values(1)
                        .required(true)
                )
        );

    let searches = load_records_from_storage();
    if searches.len() == 0 {
        println!("Keywords are empty. Use `add` to start adding.");
    }
    for i in 0..searches.len() {
        clap = clap.arg(Arg::with_name(&searches[i].keyword)
            .long(&searches[i].keyword)
            .help(&searches[i].description)
            .max_values(1000)
            .takes_value(true))
    }

    let matches = clap.get_matches_safe();

    // Built-in commands like --help returns Error of kind `ErrorKind::HelpDisplayed` so handle this
    match matches.as_ref() {
        Ok(_) => {}
        Err(error) => {
            if error.kind == ErrorKind::VersionDisplayed || error.kind == ErrorKind::HelpDisplayed {
                // message contains the information in case of --help & -V
                println!("{}", error.message);
                return;
            }
        }
    }
    // Invalid if there are no matches of args or sub-command or is empty
    let is_valid = (matches.as_ref().is_ok()) &&
        (!matches.as_ref().unwrap().args.is_empty() || !matches.as_ref().unwrap().subcommand.is_none());
    if is_valid {
        let matched_keyword = &matches.unwrap();
        match matched_keyword.subcommand() {
            ("list", _) => {
                display_table();
                return;
            }
            ("add", Some(sub)) => {
                let url = sub.value_of("q").unwrap().trim();
                if Url::parse(&url) == Err(UrlError::RelativeUrlWithoutBase) {
                    println!("Invalid query url {} provided.", &url);
                    return;
                }
                let keyword = sub.value_of("kw").unwrap().trim();
                if keyword.len() != 2 {
                    println!("Keyword must be exactly of length 2. Provided keyword: {}", &keyword);
                    return;
                }
                let description = sub.values_of("d").unwrap().collect::<Vec<_>>().join(" ");
                let search = Search::new(url, description.as_str(), keyword);
                let result = SearchPersistence::update(search);
                if result == PersistenceResult::Updated {
                    println!("Updated keyword {}.", keyword);
                } else if result == PersistenceResult::Created {
                    println!("Added new keyword {}.", keyword);
                }
                return;
            }
            ("del", Some(sub)) => {
                let keyword = sub.value_of("kw").unwrap().trim().to_string();
                let result = SearchPersistence::remove(keyword.clone());
                if result == PersistenceResult::Deleted {
                    println!("Deleted keyword {}.", keyword);
                } else if result == PersistenceResult::Nothing {
                    println!("No such keyword {} found.", keyword);
                }
                return;
            }
            _ => {}
        }
        match matched_keyword {
            _ => {
                let search_keyword = get_first_keyword(&matched_keyword);
                let query_to_be_searched: Vec<&str> =
                    (&matched_keyword).values_of(&search_keyword).unwrap().collect();
                let link =
                    build_search_query_link(&search_keyword, &query_to_be_searched.join(" "), searches);
                launch_browser(&link);
            }
        }
    } else {
        println!("lol, what?");
    }
}

fn get_first_keyword(matches: &ArgMatches) -> String {
    String::from(matches.args.keys().take(1).map(|s| &**s).collect::<Vec<_>>().join(","))
}


fn load_records_from_storage() -> Vec<Search> {
    if !SearchPersistence::is_already_exits() {
        SearchPersistence::write(&Search::initialize());
    }
    return SearchPersistence::load();
}

fn build_search_query_link(input: &String, query: &String, searches: Vec<Search>) -> String {
    let map = Search::convert_to_map(&searches);
    let link = format!("{}{}", map[&input], query.replace(" ", "+"));
    println!("Launching... {}", &link);
    return link;
}

fn display_table() {
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


fn launch_browser(link: &String) {
    if !webbrowser::open(link.as_str()).is_ok() {
        println!("Failed to open but here is the link tho... {}", &link);
    }
}
