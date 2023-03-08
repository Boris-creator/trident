use dotenv::dotenv;
use std::path::Path;
use teloxide::types::InputFile;
use teloxide::{prelude::*, utils::command::BotCommands};

mod get_stats;
mod text_to_image;

use get_stats::{fetch, print_stats};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    dotenv().ok();
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Shows the fascists' army losses.")]
    Stats,
    #[command(description = "Displays the list of available commands.")]
    Help,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Stats => {
            let response = fetch().await;
            match response {
                Ok(data) => {
                    let actual_data = data.data;
                    print_stats(actual_data);
                }
                Err(err) => println!("{}", err),
            }
            let path = Path::new("assets/result.jpg");
            bot.send_photo(msg.chat.id, InputFile::file(path)).await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, "/stats\n\
            Display the up-to-date data on the Russian invasion of Ukraine according to https://russianwarship.rip/\n\
            /help\n\
            Displays this text.
            ")
                .await?
        }
    };

    Ok(())
}
