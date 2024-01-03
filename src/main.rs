use std::env;

use rand::seq::SliceRandom;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

const HELP_MESSAGE: &str = "
    This is a bot created for the purpose of assisting with the class taught by Alvin Ashiatey.
    The following commands are available:
    `!help` - Displays this message.
    `!randomize` - Randomizes the order of the students in the class.
";
const HELP_COMMAND: &str = "!help";
const RANDOMIZE_COMMAND: &str = "!randomize";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            HELP_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            RANDOMIZE_COMMAND => {
                let mut members = msg
                    .guild_id
                    .unwrap()
                    .members(&ctx.http, None, None)
                    .await
                    .unwrap();

                members.retain(|member| !member.user.bot);
                members.shuffle(&mut rand::thread_rng());

                let mut message = MessageBuilder::new();

                for member in members {
                    message.push_bold_line_safe(member.user.name);
                }

                if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                    println!("Error sending message: {:?}", why);
                }
            }

            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
