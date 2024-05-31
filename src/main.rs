mod countdown;

use dotenvy::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{ChatId, MessageId};
use tokio::time::{interval, Duration};
use crate::countdown::get_time_remaining;
use std::thread;
use std::env;

pub struct Config {
    pub token: String,
    pub cooldown: u64,
    pub chat_id: i64,
    pub message_id: i32,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        dotenv().expect("nie ma .env");

        let token = env::var("TOKEN").unwrap_or_default();
        let cooldown = env::var("COOLDOWN").unwrap_or_default().parse::<u64>().unwrap_or_default();
        let chat_id = env::var("CHAT_ID").unwrap_or_default().parse::<i64>().unwrap_or_default();
        let message_id = env::var("MESSAGE_ID").unwrap_or_default().parse::<i32>().unwrap_or_default();

        Ok(Config{
            token,
            cooldown,
            chat_id,
            message_id,
        })
    }
}

#[tokio::main]
async fn main() {
    let config = Config::build().expect("config robi bum");
    let bot = Bot::new(config.token);

    println!("dziala i nie wybuchlo");

    let bot_clone = bot.clone();
    let editing_thread = thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let mut interval = interval(Duration::from_secs(config.cooldown));
            loop {
                interval.tick().await;
                let (days, hours, minutes, seconds, to_end) = get_time_remaining().await;
                
                let message = if to_end {
                    format!("{days} dni, {hours} godz, {minutes} min, {seconds} sek do końca wakacji 🍂")
                } else {
                    format!("{days} dni, {hours} godz, {minutes} min i {seconds} sek do wakacji 🌴☀️")
                };

                let result = bot_clone.edit_message_text(ChatId(config.chat_id), MessageId(config.message_id), message).await;
        
                if let Err(err) = result {
                    eprintln!("jednak wybuchlo: {}", err);
                }
            }
        });
    });

    Command::repl(bot, answer).await;
    editing_thread.join().unwrap();
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    //#[command(description = "display this text.")]
    //Help,
    #[command(description = "wyświetla id wiadomości, jej zawartość i id grupy.")]
    MsgInfo,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        //Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::MsgInfo => {
            let msg_id = msg.reply_to_message().unwrap().id;
            let text = msg.reply_to_message().and_then(|m| m.text());
            let text_formatted = match text {
                Some(t) => t.to_string(),
                None => String::from("<No text>")
            };
            let group_id = msg.chat.id;
            println!("{}", format!("id wiadomosci: {msg_id}\ntekst: {text_formatted}\nid czatu: {group_id}"));
            bot.send_message(msg.chat.id, format!("id wiadomosci: {msg_id}\ntekst: {text_formatted}\nid czatu: {group_id}")).await?
        }
    };

    Ok(())
}