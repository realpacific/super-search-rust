use self::super::search::Search;

pub fn run() {
    let input = String::from("md");
    let query = String::from("How to rust?");
    let mut searches = vec![];
    for item in Search::initialize() {
        searches.push(Search::from_json(String::from(item.to_json())));
    }
    let option = searches.into_iter().find(|x| x.keyword == input);
    match option {
        Some(ref s) => println!("Search using {}{}", s.url, query.replace(" ", "+")),
        _ => println!("{}", "lol what?")
    }
}
