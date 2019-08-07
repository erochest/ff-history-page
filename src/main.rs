#![recursion_limit = "256"]

#[macro_use]
extern crate diesel;

use std::collections::HashMap;
use std::env;
use std::hash::Hash;

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use typed_html::dom::DOMTree;
use typed_html::{html, text};

use crate::errors::Result;
use crate::models::{HistoryVisit, Place};

mod errors;
mod models;
mod schema;

fn main() -> Result<()> {
    use crate::schema::{moz_historyvisits, moz_places};

    let connection = connect()?;
    let mut results: Vec<(Place, HistoryVisit)> = moz_places::table
        .inner_join(moz_historyvisits::table.on(moz_places::id.eq(moz_historyvisits::place_id)))
        .limit(1000)
        .load(&connection)?;
    results.sort_unstable_by_key(|r| r.0.last_visited_date);
    let mut index = groupby(results, |r| {
        let week = r.0.get_last_visited_date().unwrap_or_else(Utc::now).date().iso_week();
        (week.year(), week.week())
    })
        .into_iter()
        .collect::<Vec<_>>();
    index.sort_unstable_by_key(|pair| pair.0);
    index.reverse();

    let doc: DOMTree<String> = html!(
        <html>
            <head><title>"Firefox History"</title></head>
            <body>
                <h1>"FireFox History"</h1>
                {index.into_iter().
                    // These are unsorted. Need to sort before here.
                    map(|((year, week), entries)| {
                        html!(<div>
                            <h2>{ text!("{} - week {}", year, week) }</h2>
                            <ul>
                                {entries.into_iter()
                                    .map(|(place, _)| html!(
                                            <li>
                                                { place.to_link() }
                                                { text!(": {}", place.get_last_visited_date().unwrap_or_else(Utc::now)) }
                                            </li>
                                    ))
                                }
                            </ul>
                        </div>)
                    })
                }
            </body>
        </html>
    );

    println!("{}", doc);

    Ok(())
}

fn connect() -> Result<SqliteConnection> {
    dotenv()?;
    let database_url = env::var("DATABASE_URL")?;
    SqliteConnection::establish(&database_url).map_err(|e| e.into())
}

fn groupby<I, K: Eq + Hash, F: Fn(&I) -> K>(input: Vec<I>, key_fn: F) -> HashMap<K, Vec<I>> {
    input.into_iter()
        .map(|i| (key_fn(&i), i))
        .fold(HashMap::new(), |mut m, pair| {
            let (key, item) = pair;
            let entry = m.entry(key);
            entry.or_default().push(item);
            m
        })
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::groupby;

    #[test]
    fn test_groupby_returns_items_grouped_by_key() {
        let input = vec![
            "aaa".to_string(),
            "bbb".to_string(),
            "ccc".to_string(),
            "abc".to_string(),
        ];
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["aaa".to_string(), "abc".to_string()]);
        expected.insert("b".to_string(), vec!["bbb".to_string()]);
        expected.insert("c".to_string(), vec!["ccc".to_string()]);

        let actual = groupby(input, |s: &String| s[0..1].to_string());

        assert_eq!(expected, actual);
    }
}
