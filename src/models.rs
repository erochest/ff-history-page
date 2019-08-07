use chrono::prelude::*;
use typed_html::elements::*;
use typed_html::{html, text};

#[derive(Debug, Queryable)]
pub struct Place {
    pub id: Option<i32>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub rev_host: Option<String>,
    pub visit_count: Option<i32>,
    pub hidden: i32,
    pub typed: i32,
    pub favicon: Option<i32>,
    pub frecency: i32,
    pub last_visited_date: Option<i64>,
    pub guid: Option<String>,
    pub foreign_count: i32,
    pub url_hash: i32,
    pub description: Option<String>,
    pub preview_image_url: Option<String>,
    pub origin_id: Option<i32>,
}

impl Place {
    pub fn to_link(&self) -> Option<Box<a<String>>> {
        self.url.clone().map(|url| {
            let title = self.title.clone().unwrap_or_else(|| url.clone());
            html!(
                <a href=url>{ text!("{}", title) }</a>
            )
        })
    }

    pub fn get_last_visited_date(&self) -> Option<DateTime<Utc>> {
        self.last_visited_date
            .map(|timestamp| Utc.timestamp(timestamp / 1000000, (timestamp % 1000000) as u32))
    }
}

#[derive(Debug, Queryable)]
pub struct HistoryVisit {
    pub id: Option<i32>,
    pub from_visit: Option<i32>,
    pub place_id: Option<i32>,
    pub visit_date: Option<i64>,
    pub visit_type: Option<i32>,
    pub session: Option<i32>,
}
