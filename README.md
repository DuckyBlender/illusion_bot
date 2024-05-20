**Illusion Bot**
================

A Telegram bot that generates illusions from user-provided images and prompts.

**Features**
-----------

* Responds to user commands: ~~`/illusion`~~, `/trollface`, `/help`, and `/start`
* Generates illusions using a Python script and sends the result as a photo
~~* Uploads images to an S3 bucket and generates a presigned URL for the API~~
* Supports throttling to prevent abuse

**Setup**
--------

1. Create a Telegram bot and obtain a bot token
2. Install AWS CLI and configure it with `aws configure`
3. Set up an S3 bucket and configure the `BUCKET_NAME` environment variable
4. Run the bot using `cargo run --release`

**Commands**
------------

- [ ] `/illusion <prompt>`: Generate an illusion from a user-provided image and prompt
- [x] `/trollface <prompt>`: Generate a trollface with a user-provided prompt
- [x] `/help`: Display available commands
- [x] `/start`: Welcome message

**Technical Details**
--------------------

* The bot uses the `teloxide` library to interact with the Telegram API
* The `dotenv` library is used to load environment variables from a `.env` file
* The `log` and `pretty_env_logger` libraries are used for logging

**Testing**
----------

* A test script is provided in the `tests` module to verify the Python script execution

**Note**
----

This README file is a brief summary of the project. For more information, please refer to the code comments and documentation.