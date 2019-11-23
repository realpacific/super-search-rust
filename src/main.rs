#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;

use clap::{App as Clap, Arg, ArgMatches, SubCommand};

use crate::app::{fetch_inputs, print};

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
    let mut usage_string = String::from("");
    for i in 0..searches.len() {
        usage_string = if i == 0 {
            String::from(format!("--{} '{}'", searches[i].keyword, searches[i].description))
        } else {
            String::from(format!(
                "{}\n--{} '{}'", usage_string, searches[i].keyword, searches[i].description))
        }
    }
    println!("{}", &usage_string);
    clap = clap.arg(
        Arg::from_usage("-c, --config=[FILE] 'Sets a custom config file'
                              <INPUT>              'Sets the input file to use'
                              -v...                'Sets the level of verbosity'").takes_value(true).max_values(100)
    );
    let matches = clap.get_matches();
    let keyword = get_first_keyword(&matches);
    let vals: Vec<&str> = matches.values_of(&keyword).unwrap().collect();

    app::search(&keyword, &vals.join(" "));
}

fn get_first_keyword(matches: &ArgMatches) -> String {
    String::from(matches.args.keys().take(1).map(|s| &**s).collect::<Vec<_>>().join(","))
}
