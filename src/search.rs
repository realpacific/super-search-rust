use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    pub url: String,
    pub description: String,
    pub keyword: String,
}

impl Display for Search {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Search(url:{}, keyword:{}, description: {})", self.url, self.keyword, self.description)
    }
}

#[allow(dead_code)]
impl Search {
    pub fn from_json(s: String) -> Self {
        return serde_json::from_str(s.as_str()).unwrap();
    }

    pub fn to_json(&self) -> String {
        json!({
            "url": self.url,
            "keyword": self.keyword,
            "description":self.description
        }).to_string()
    }

    pub fn initialize() -> Vec<Self> {
        let mut searches = Vec::new();
        searches.push(Search::new("https://www.youtube.com/results?search_query=", "Youtube", "yt"));
        searches.push(Search::new("https://medium.com/search?q=", "Medium", "md"));
        searches.push(Search::new("https://stackoverflow.com/search?q=", "StackOverflow", "so"));
        searches.push(Search::new("https://github.com/search?q=", "GitHub", "gh"));
        searches
    }

    pub fn new(url: &str, description: &str, keyword: &str) -> Search {
        Search {
            url: url.to_string(),
            description: description.to_string(),
            keyword: keyword.to_string(),
        }
    }

    pub fn convert_to_map(vector: &Vec<Self>) -> HashMap<&String, &String> {
        let mut map = HashMap::new();
        vector.iter().for_each(|x| {
            map.insert(&x.keyword, &x.url);
        });
        return map;
    }

    pub fn map_keyword_to_self(vector: &Vec<Self>) -> HashMap<&String, &Search> {
        let mut map: HashMap<&String, &Self> = HashMap::new();
        vector.iter().for_each(|x| {
            map.insert(&x.keyword, &x);
        });
        return map;
    }
}
