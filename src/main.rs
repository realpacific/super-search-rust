#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;

mod app;
mod search;
mod persistence;

fn main() {
    app::run();
}
