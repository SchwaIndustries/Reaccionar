#![feature(proc_macro_hygiene, decl_macro)]
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

#[macro_use] extern crate rocket;

#[group]
#[commands(ping, track_message, untrack_message)]
struct General;

use dotenv::dotenv;
use std::env;
// mod botdb;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{}#{} is connected!", ready.user.name, ready.user.discriminator);
        thread::spawn(|| {
            diss_status(ctx);
        });
        rocket::ignite().mount("/", routes![index]).launch();
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Some(guild) = reaction.guild_id {
            // if let guild_info = botdb::get_guild(guild) {
            //     if reaction.message_id == guild_info.message_id && reaction.emoji == ReactionType::Unicode(guild_info.emoji) {
            //         let mut member = guild.member(&ctx, &reaction.user_id).expect("Member not found");
            //         member.add_role(&ctx, guild_info.role_id).expect("Role addition failed");
            //         println!("Reaction {} was added to message {} by user {}", reaction.emoji, reaction.message_id, reaction.user_id);
            //     }
            // }
            if guild == 667553378607431681 && reaction.message_id == 712566183257440266 { 
                if reaction.emoji != ReactionType::Unicode('\u{2705}'.to_string()) { return; }
                let mut member = guild.member(&ctx, &reaction.user_id).expect("Member not found");
                member.add_role(&ctx, 687460972960415771).expect("Role addition failed");
                println!("Reaction {} was added to message {} by user {}", reaction.emoji, reaction.message_id, reaction.user_id);
            }
            if guild == 704495983542796338 && reaction.message_id == 709842698575675416 { 
                if reaction.emoji != ReactionType::Unicode('\u{1F44D}'.to_string()) { return; }
                let mut member = guild.member(&ctx, &reaction.user_id).expect("Member not found");
                member.add_role(&ctx, 704500053519368383).expect("Role addition failed");
                println!("Reaction {} was added to message {} by user {}", reaction.emoji, reaction.message_id, reaction.user_id);
            }
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "What's up? This is reaccionar. Yet another reaction roles bot, powered by the finest Rust code."
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
        thread::sleep(Duration::from_secs(60));
        &ctx.set_activity(Activity::playing("unlike Reaction Roles!"));
        thread::sleep(Duration::from_secs(60));

        reqwest::blocking::get(&env::var("DOMAIN").expect("Domain to ping not found")).expect("keep alive failed");
    }
}
