use std::error::Error;
use teloxide::{
    payloads::{self, SendMessageSetters},
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me,
    },
    utils::command::BotCommands,
};
use url::Url;

/// These commands are supported:
#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text
    Help,
    /// Start
    Start,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let bot = Bot::from_env();

    let handler =
        dptree::entry().branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("{:?}", q.id);

    let mut data = payloads::AnswerCallbackQuery::new(q.id);

    let url = Url::parse("https://telegram-bot-taupe.vercel.app/").unwrap();

    data.url = Some(url);
    println!("as");

    println!("{:?}", data);

    let d = <teloxide::Bot as teloxide::prelude::Requester>::AnswerCallbackQuery::new(
        bot.clone(),
        data,
    )
    .await;

    // let let d=d = bot.answer_callback_query(q.id).await;

    println!("asd");

    Ok(())
}
