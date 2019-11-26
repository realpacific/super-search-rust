#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;

use clap::{App as Clap, Arg, ArgMatches, SubCommand};

use crate::app::fetch_inputs;

mod app;
mod search;
mod persistence;

fn main() {
    let mut clap = Clap::new("ss")
        .about("A CLI tool for searching on your favorites websites written in Rust.")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("list")
                .about("List all stored keywords.")
                .alias("ls"),
        );
    let searches = fetch_inputs();
    for i in 0..searches.len() {
        clap = clap.arg(Arg::with_name(&searches[i].keyword)
            .long(&searches[i].keyword)
            .help(&searches[i].description)
            .max_values(1000)
            .takes_value(true))
    }
    let matches = clap.get_matches_safe();
    let is_valid = matches.is_ok() &&
        (!((&matches).as_ref().unwrap().args).is_empty() || !(&matches).as_ref().unwrap().subcommand.is_none());
    if is_valid {
        let matched_keyword = &matches.unwrap();
        match matched_keyword.subcommand() {
            ("list", _) => {
                app::print();
                return;
            }
            _ => {}
        }
        match matched_keyword {
            _ => {
                let search_keyword = get_first_keyword(&matched_keyword);
                let query_to_be_searched: Vec<&str> = (&matched_keyword).values_of(&search_keyword).unwrap().collect();
                app::search(&search_keyword, &query_to_be_searched.join(" "));
            }
        }
    } else {
        println!("lol, what?");
    }
}

fn get_first_keyword(matches: &ArgMatches) -> String {
    String::from(matches.args.keys().take(1).map(|s| &**s).collect::<Vec<_>>().join(","))
}
