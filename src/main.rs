use dotenv::dotenv;
use log::{error, info};
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
    #[command(description = "generate a trollface with the specified prompt")]
    Trollface(String),
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
            bot.send_message(
                msg.chat.id,
                "Welcome to the Illusion Bot! Send /help to see the available commands.",
            )
            .await?;
        }
        Command::Trollface(prompt) => {
            generate_illusion(
                &bot,
                &msg,
                &prompt,
                "https://upload.wikimedia.org/wikipedia/en/9/9a/Trollface_non-free.png",
            )
            .await?;
        } // Command::Illusion(prompt) => {
          // let url = match &msg.reply_to_message() {
          //     Some(reply) => match &reply.photo() {
          //         Some(photos) => photos[0].file.id.clone(),
          //         None => {
          //             bot.send_message(
          //                 msg.chat.id,
          //                 "Please reply to an image to generate an illusion.",
          //             )
          //             .await?;
          //             return Ok(());
          //         }
          //     },
          //     None => {
          //         bot.send_message(
          //             msg.chat.id,
          //             "Please reply to an image to generate an illusion.",
          //         )
          //         .await?;
          //         return Ok(());
          //     }
          // };

          // // We can't just pass the download URL to the API because this would expose the bot token
          // // So we download the image into memory, send it to s3, and then pass the presigned s3 URL to the API
          // // Download the image
          // let img_file = bot.get_file(&url).await.unwrap();
          // let img_url = img_file.path;
          // let mut buf: Vec<u8> = Vec::new();
          // bot.download_file(&img_url, &mut buf).await.unwrap();
          // todo!();
          // }
    }
    Ok(())
}

async fn generate_illusion(
    bot: &Bot,
    msg: &Message,
    prompt: &str,
    url: &str,
) -> ResponseResult<()> {
    // Spawn a new task to send typing indicator every 5 seconds
    info!("Generating illusion for: {} with image: {}", prompt, url);

    // Send typing indicator
    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
        .await?;

    // Measure the time it takes to execute the command
    let now = std::time::Instant::now();

    // Invoke the python script with the argument prompt
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg("python")
            .arg("python\\illusion.py")
            .arg("--prompt")
            .arg(&prompt)
            .arg("--image")
            .arg(&url)
            .output()
            .expect("Failed to execute command")
    } else {
        std::process::Command::new("python3")
            .arg("python/illusion.py")
            .arg("--prompt")
            .arg(&prompt)
            .arg("--image")
            .arg(&url)
            .output()
            .expect("Failed to execute command")
    };

    info!("Time taken: {:?}", now.elapsed());

    info!("Status: {:?}", output.status);
    // If the status is 0, then the command was successful and the output is a local path to the image
    match output.status.code() {
        Some(0) => {
            // Send uploading indicator
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadPhoto)
                .await?;

            // Convert the output to a string to get the path to the image
            let output_str = String::from_utf8(output.stdout).unwrap();
            let output_str = output_str.trim();
            info!("Output: {:?}", output_str);
            // Load the file into memory
            let image = std::fs::read(output_str).unwrap();
            // Send the image as a photo
            bot.send_photo(msg.chat.id, InputFile::memory(image))
                .caption(format!(
                    "{} ({}s)",
                    prompt,
                    now.elapsed().as_millis() as f64 / 1000.0
                ))
                .reply_to_message_id(msg.id)
                .await?;
        }
        Some(code) => {
            error!("Command failed with code: {}", code);
            error!("Error: {:?}", String::from_utf8(output.stderr).unwrap());
            bot.send_message(msg.chat.id, format!("Command failed with code: {}", code))
                .reply_to_message_id(msg.id)
                .await?;
        }
        None => {
            info!("Command failed with no code");
            bot.send_message(msg.chat.id, "Command failed with no code")
                .reply_to_message_id(msg.id)
                .await?;
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
                .output()
                .expect("Failed to execute command")
        } else {
            std::process::Command::new("python3")
                .arg("python/illusion.py")
                .arg("--status")
                .output()
                .expect("Failed to execute command")
        };

        println!("Status: {:?}", output.status);
        println!("Output: {:?}", String::from_utf8(output.stdout).unwrap());
        println!("Error: {:?}", String::from_utf8(output.stderr).unwrap());
        assert_eq!(output.status.code(), Some(0));
    }
}
