extern crate dotenv;
mod commands;
use commands::levelup;

use std::{collections::HashSet, env};
use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::{
            Args, CommandGroup, CommandResult, HelpOptions, help_commands,
            macros::{command, group, help, check, hook}
        }
    },
    http::Http, 
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*
};
use dotenv::dotenv;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    
    let http = Http::new_with_token(&token);
    // Fetch bot's owner and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access app info: {:?}", why)
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("%"))
        .help(&MY_HELP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
