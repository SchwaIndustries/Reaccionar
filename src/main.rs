use serenity::client::Client;
use serenity::model::{channel::Message, channel::Reaction, gateway::Ready};
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
extern crate dotenv;

#[group]
#[commands(ping)]
struct General;

use dotenv::dotenv;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        println!("Reaction {} was added to message {} by user {}", reaction.emoji, reaction.message_id, reaction.user_id)
    }
}

fn main() {
    // Login with a bot token from the environment
    dotenv().ok();
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
