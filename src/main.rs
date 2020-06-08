use serenity::client::Client;
#[allow(unused_imports)]
use serenity::model::{
    channel::{Message, Reaction, ReactionType},
    gateway::{Ready, Activity},
    id::{MessageId, GuildId, RoleId}
};
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    Args, CommandResult,
    macros::{
        command,
        group
    }
};
extern crate dotenv;
use std::time::Duration;
use std::thread;

#[group]
#[commands(ping, track_message, untrack_message)]
struct General;

use dotenv::dotenv;
use std::env;
// use botdb;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{}#{} is connected!", ready.user.name, ready.user.discriminator);
        diss_status(ctx);
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Some(guild) = reaction.guild_id {
            if reaction.guild_id != 667553378607431681 { return; }
            if reaction.message_id != 712566183257440266 { return; }
            if reaction.emoji != ReactionType::Unicode("✅".to_string()) { return; }
            let mut member = guild.member(&ctx, &reaction.user_id).expect("Member not found");
            member.add_role(&ctx, 687460972960415771).expect("Role addition failed");
        }
        println!("Reaction {} was added to message {} by user {}", reaction.emoji, reaction.message_id, reaction.user_id);
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

#[command]
fn track_message(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let message_id = match args.single::<u64>() {
        Ok(id) => MessageId(id),
        Err(_) => {
            msg.reply(&ctx, "Requires a valid message ID to be given").expect("Message failed");

            return Ok(());
        },
    };

    let emoji = match args.single::<String>() {
        Ok(id) => id.to_string(),
        Err(_) => {
            msg.reply(&ctx, "Requires a valid emoji to be given").expect("Message failed");

            return Ok(());
        },
    };

    let role_id = match args.single::<u64>() {
        Ok(id) => RoleId(id),
        Err(_) => {
            msg.reply(&ctx, "Requires a valid role ID to be given").expect("Message failed");

            return Ok(());
        },
    };

    msg.reply(ctx, &format!("Reaction listener added for message {}, watching for emoji {}, and giving role {}", message_id, emoji, role_id))?;

    Ok(())
}

#[command]
fn untrack_message(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let message_id = match args.single::<u64>() {
        Ok(id) => MessageId(id),
        Err(_) => {
            msg.reply(&ctx, "Requires a valid message ID to be given").expect("Message failed");

            return Ok(());
        },
    };

    let emoji = match args.single::<String>() {
        Ok(id) => id.to_string(),
        Err(_) => {
            msg.reply(&ctx, "Requires a valid emoji to be given").expect("Message failed");

            return Ok(());
        },
    };

    let role_id = match args.single::<u64>() {
        Ok(id) => RoleId(id),
        Err(_) => {
            msg.reply(&ctx, "Requires a valid role ID to be given").expect("Message failed");

            return Ok(());
        },
    };

    msg.reply(ctx, &format!("Reaction listener removed for message {}, no longer watching for emoji {}, and giving role {}", message_id, emoji, role_id))?;

    Ok(())
}

fn diss_status(ctx: Context) {
    loop {
        &ctx.set_activity(Activity::playing("todo el tiempo!"));
        thread::sleep(Duration::from_secs(15));
        &ctx.set_activity(Activity::playing("unlike Reaction Roles!"));
        thread::sleep(Duration::from_secs(15));
    }
}
