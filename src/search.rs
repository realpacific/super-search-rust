use serde_json;
use std::fmt::{Display, Formatter, Error};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;

#[derive(Serialize, Deserialize)]
pub struct Search {
    pub url: String,
    pub description: String,
    pub keyword: String,
}

impl Display for Search {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Search(url:{}, keyword:{})", self.url, self.keyword)
    }
}

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

    pub fn initialize() -> Vec<Box<Self>> {
        let mut searches = Vec::new();
        searches.push(Box::new(
            Search {
                url: "https://www.youtube.com/results?search_query=".to_string(),
                description: "Youtube".to_string(),
                keyword: "yt".to_string(),
            }));
        searches.push(Box::new(Search {
            url: "https://medium.com/search?q=".to_string(),
            description: "Medium".to_string(),
            keyword: "md".to_string(),
        }));
        searches.push(Box::new(Search {
            url: "https://github.com/search?q=".to_string(),
            description: "GitHub".to_string(),
            keyword: "gh".to_string(),
        }));
        searches
    }
}
