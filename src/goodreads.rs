use failure::{format_err, Error};

use futures::TryStreamExt;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use telegram_bot::{InlineQueryResult, InlineQueryResultArticle, InputTextMessageContent};

#[derive(Deserialize, Debug)]
struct GoodreadsResponse {
    search: Search,
}

#[derive(Deserialize, Debug)]
struct Search {
    #[serde(rename = "total-results", default)]
    total_results: String,
    results: Results,
}

#[derive(Deserialize, Debug)]
struct Results {
    #[serde(rename = "work", default)]
    works: Vec<Work>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Work {
    #[serde(rename = "best_book", default)]
    pub book: Book,
}

#[derive(Deserialize, Debug, Default)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub small_image_url: String,
}

#[derive(Clone, Debug, Default)]
pub struct Api {
    token: String,
}

impl Api {
    pub fn new(token: &str) -> Api {
        Api {
            token: token.to_string(),
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<InlineQueryResult>, Error> {
        goodreads_search(self.token.clone(), query.to_string()).await
    }
}

pub async fn goodreads_search(
    token: String,
    query: String,
) -> Result<Vec<InlineQueryResult>, Error> {
    let https = HttpsConnector::new(1)?;
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = format!(
        "https://www.goodreads.com/search/index.xml?key={}&q={}",
        &token,
        urlencoding::encode(&query)
    )
    .parse::<hyper::Uri>()
    .unwrap();

    let res = client.get(url).await?;
    let body = res.into_body().try_concat().await?;

    let response: GoodreadsResponse = serde_xml_rs::from_reader(body.as_ref())
        .map_err(|err| format_err!("error parsing good reads response: {}", err))?;

    let results = response
        .search
        .results
        .works
        .into_iter()
        .map(work_to_article)
        .map(From::from)
        .collect();
    Ok(results)
}

fn work_to_article(work: Work) -> InlineQueryResultArticle {
    let message = InputTextMessageContent {
        message_text: format!("http://www.goodreads.com/book/show/{}", work.book.id),
        parse_mode: Some(telegram_bot::ParseMode::Html),
        disable_web_page_preview: false,
    };

    let mut article = InlineQueryResultArticle::new(work.book.id, work.book.title, message);
    article.thumb_url(work.book.small_image_url);
    article
}
