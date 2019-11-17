#[macro_use]
extern crate serde_json;

mod app;
mod search;

fn main() {
    app::run();
}
