use okina_bot_kotoba_web::{
    RankCommands, RankQuizzes,
    commands::{
        levelup::*,
        metadata::*,
        env_variables::{
            get_rank_commands,
            get_rank_quizzes,
            ENV_VARS,
        }
    }
};

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::utils::Colour;
use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{group, help},
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
    collections::HashSet,
    sync::Arc,
};

#[group]
#[commands(levelup, nivel, niveis, tabela)]
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
    let token = ENV_VARS["DISCORD_TOKEN"].as_str().expect("A valid 'DISCORD_TOKEN' value was not found in env.json.");

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
        .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS)
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
    _args: Args,
    _help_options: &'static HelpOptions,
    _groups: &[&'static CommandGroup],
    _owners: HashSet<UserId>,
) -> CommandResult {
    // let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    let _ = msg
        .channel_id
        .send_message(&context.http, |m| {
            m.content("おお、よくぞ来られた。お前が二童子の言っていた新米だな~");
            m.embed(|e| {
                e.title("Resumo dos comandos");
                e.color(Colour::ORANGE);
                e.description(
                    "Lembretes:\n".to_owned()
                        + " - Cada quiz deve ser feito solo\n"
                        + " - Os quizzes devem ser feitos na ordem correta\n"
                        + " - Por favor, fazer os quizzes nos canais corretos",
                );
                e.field(
                    "`%levelup`",
                    "Envia uma DM com o próximo quiz a ser feito pelo usuário",
                    false,
                );
                e.field(
                    "`%niveis`",
                    "Envia uma DM com todos os nomes e quizzes de cada nível",
                    false,
                );
                e.field(
                    "`%nivel [numero]`",
                    "Envia o quiz cujo nível é igual a [numero] (1 a 6)",
                    false,
                );
                e.field(
                    "`%tabela`",
                    "Mostra a distribuição dos cargos de nível",
                    false,
                );
                e.field("`%help`", "Mostra essa mensagem", false);
                e.field(
                    "`隠岐奈画像出典`",
                    "[幻想郷幽玄庵 DL -> 天空璋](https://gensoukyou.1000.tv/dl.html)",
                    false,
                );
                e.field(
                    "\u{200B}",
                    "[repo](https://gitlab.com/uemi/okina-kotoba/-/tree/development)",
                    false,
                );
                e
            });
            m
        })
        .await;
    Ok(())
}
