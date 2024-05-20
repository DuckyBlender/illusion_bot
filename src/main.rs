use dotenv::dotenv;
use log::info;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    info!("Starting command bot...");

    let bot = Bot::from_env();

    bot.set_my_commands(Command::bot_commands()).await.unwrap();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "generate an ilusion for the image reply with the specified prompt")]
    Illusion(String),
    #[command(description = "help command")]
    Help,
    #[command(description = "start command")]
    Start,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome to the Illusion Bot!")
                .await?;
        }
        Command::Illusion(prompt) => {
            // Invoke the python script with the argument prompt
            let output = if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .arg("/C")
                    .arg("python")
                    .arg("python\\illusion.py")
                    .arg(&prompt)
                    .output()
                    .expect("Failed to execute command")
            } else {
                std::process::Command::new("python3")
                    .arg("python/illusion.py")
                    .arg(&prompt)
                    .output()
                    .expect("Failed to execute command")
            };

            info!("Status: {:?}", output.status);
            // If the status is 0, then the command was successful and the output is a local path to the image
            match output.status.code() {
                Some(0) => {
                    let output_str = String::from_utf8(output.stdout).unwrap();
                    info!("Output: {:?}", output_str);
                    // Load the file into memory
                    let image = std::fs::read(output_str.trim()).unwrap();
                    // Send the image as a photo
                    bot.send_photo(msg.chat.id, InputFile::memory(image))
                        .caption(&prompt)
                        .await?;
                }
                Some(code) => {
                    info!("Command failed with code: {}", code);
                    bot.send_message(msg.chat.id, format!("Command failed with code: {}", code))
                        .await?;
                }
                None => {
                    info!("Command failed with no code");
                    bot.send_message(msg.chat.id, "Command failed with no code")
                        .await?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn text_script() {
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .arg("/C")
                .arg("python")
                .arg("python\\illusion.py")
                .arg("--status")
                .arg("dummy")
                .output()
                .expect("Failed to execute command")
        } else {
            std::process::Command::new("python3")
                .arg("python/illusion.py")
                .arg("--status")
                .arg("dummy")
                .output()
                .expect("Failed to execute command")
        };

        println!("Status: {:?}", output.status);
        println!("Output: {:?}", String::from_utf8(output.stdout).unwrap());
        println!("Error: {:?}", String::from_utf8(output.stderr).unwrap());
        assert_eq!(output.status.code(), Some(0));
    }
}
