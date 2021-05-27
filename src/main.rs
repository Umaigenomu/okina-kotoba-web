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
    utils::MessageBuilder,
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
                Some(Activity::playing("Use %help para listar os comandos")),
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
// #[individual_command_tip =
// "おお、よくぞ来られた。お前が艦長の言っていた新米だな~\n\n\
// Para obter mais detalhes sobre um commando, passe o mesmo como um argumento."]
async fn my_help(
    context: &Context,
    msg: &Message,
    _args: Args,
    _help_options: &'static HelpOptions,
    _groups: &[&'static CommandGroup],
    _owners: HashSet<UserId>,
) -> CommandResult {
    // let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    let _ = msg.channel_id.send_message(&context.http, |m| {
        m.content("おお、よくぞ来られた。お前が二童子の言っていた新米だな~");
        m.embed(|e| {
            e.title("Resumo dos comandos");
            e.description("Lembretes:\n - Cada quiz deve ser feito solo\n - - Os quizes devem ser feitos na ordem correta");
            e.field("`%levelup`", "Envia uma DM com o comando da kotoba-web do próximo quiz a ser feito pelo usuário", false);
            e.field("`%quizzes`", "Envia uma DM com todos os quizzes de cada nível", false);
            e.field("`%tabela`", "Mostra a distribuição dos cargos de nível", false);
            e.field("`%help`", "Mostra essa mensagem", false);
            e.field("`隠岐奈画像出典`", "[幻想郷幽玄庵 DL -> 天空璋](https://gensoukyou.1000.tv/dl.html)", false);
            e.field("　​", "[repo](https://gitlab.com/uemi/okina-kotoba/-/tree/development)", false);
            e
        });
        m
    }).await;
    Ok(())
}
