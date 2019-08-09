# Good Reads Search Bot
A Telegram inline bot to search good reads books, available at [@goodreads_search_bot](https://t.me/goodreads_search_bot)

## Usage
To use it within Telegram, just type ```@goodreads_search_bot <book>``` and the bot will show the list of books found.

## Build and run

- Clone the repository: ```git clone https://github.com/jxs/goodreads_search_bot.git && cd goodreads_search_bot```
- ```export TELEGRAM_BOT_TOKEN=<INSERT YOUR BOT TOKEN HERE>``` with your own bot token (you can get it from [@BotFather](https://t.me/BotFather)
- ```export GOODREADS_API_KEY=<INSERT API KEY HERE>``` with your own good reads api key (you can get it from [here](https://www.goodreads.com/api/keys) )
- Finally, build the project and run it: ```cargo run```

_N.B. Remember to set the bot as an inline bot by issuing the command_ ```/setinline``` _to BotFather_
