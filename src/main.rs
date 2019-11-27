#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;

use clap::{App as Clap, Arg, ArgMatches, SubCommand};
use prettytable::{Cell, Row, Table};
use webbrowser;

use crate::persistence::{Persistence, SearchPersistence};
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
        );
    let searches = load_records_from_storage();
    for i in 0..searches.len() {
        clap = clap.arg(Arg::with_name(&searches[i].keyword)
            .long(&searches[i].keyword)
            .help(&searches[i].description)
            .max_values(1000)
            .takes_value(true))
    }
    let matches = clap.get_matches_safe();
    // Invalid if there are no matches of args or sub-command or is empty
    let is_valid = matches.is_ok() &&
        (!((&matches).as_ref().unwrap().args).is_empty() || !(&matches).as_ref().unwrap().subcommand.is_none());
    if is_valid {
        let matched_keyword = &matches.unwrap();
        match matched_keyword.subcommand() {
            ("list", _) => {
                display_table();
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
