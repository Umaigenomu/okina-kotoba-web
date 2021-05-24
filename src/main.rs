extern crate dotenv;
mod commands;

// use okina_bot_kotoba_web::utils::Pipe;
use commands::levelup::*;

use commands::env_variables::{get_rank_commands, get_rank_quizzes, QuizSettings};

use dotenv::dotenv;
use serenity::{
    async_trait,
    framework::{
        standard::{
            help_commands,
            macros::{check, command, group, help, hook},
            Args, CommandGroup, CommandResult, HelpOptions,
        },
        StandardFramework,
    },
    http::Http,
    model::{
        channel::Message,
        gateway::Ready,
        id::UserId,
        prelude::{Activity, OnlineStatus},
    },
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

struct RankCommands;
impl TypeMapKey for RankCommands {
    type Value = Arc<HashMap<u64, String>>;
}

struct RankQuizzes;
impl TypeMapKey for RankQuizzes {
    type Value = Arc<HashMap<String, QuizSettings>>;
}

#[group]
#[commands(levelup)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "%ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong%").await {
                println!("Error sending message: {:?}", why);
            }
        }
        let _ = on_kotoba_msg((ctx, msg)).await;
        // let _ = Pipe::new((ctx, msg)) >> on_kotoba_msg;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let _ = ctx
            .set_presence(
                Some(Activity::playing("Use |help para listar os comandos")),
                OnlineStatus::Online,
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

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
        }
        Err(why) => panic!("Could not access app info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("%")
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<RankCommands>(Arc::new(get_rank_commands()));
        data.insert::<RankQuizzes>(Arc::new(get_rank_quizzes()));
    }

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
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
