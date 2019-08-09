#![feature(async_await)]

mod goodreads;

use futures::StreamExt;
use goodreads::Api as GoodReadsApi;
use telegram_bot::{
    Api as TelegramBotApi, CanAnswerInlineQuery, CanReplySendMessage, Update, UpdateKind,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let telegram_token =
        std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found in environment");
    let goodreads_key =
        std::env::var("GOODREADS_API_KEY").expect("GOODREADS_API_KEY not found in environment");
    let telegram_api = TelegramBotApi::new(&telegram_token);

    let goodreads_api = GoodReadsApi::new(&goodreads_key);
    let mut stream = telegram_api.stream();

    log::info!("starting good reads telegram bot!");
    while let Some(update) = stream.next().await {
        match update {
            Ok(Update {
                kind: UpdateKind::InlineQuery(query),
                ..
            }) => match goodreads_api.search(&query.query).await {
                Ok(results) => {
                    let reply = query.answer(results);
                    let res = telegram_api.send(reply).await;
                    if let Err(err) = res {
                        log::error!("telegram bot send error, {:?}", err);
                    }
                }
                Err(err) => {
                    log::error!("update error, {}", err);
                    continue;
                }
            },
            Ok(Update {
                kind: UpdateKind::Message(message),
                ..
            }) => {
                let res = telegram_api.send(message.text_reply(
                    "Hi, I am an inline Telegram Bot, I don't respond to commands, you can use me to search Good Reads books:
 start a message tagging me following the book you want to seach GoodReads ex:
@goodreads_search_bot game of thrones",
                )).await;
                if let Err(err) = res {
                    log::error!("telegram bot send error, {:?}", err);
                }
            }
            Err(err) => {
                log::error!("update error, {}", err);
            }
            _ => {}
        }
    }
}
