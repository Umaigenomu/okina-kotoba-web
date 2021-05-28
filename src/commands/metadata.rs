use std::collections::HashSet;

use crate::commands::env_variables::{RANK_NAMES, RANK_ROLES};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::{client::bridge::gateway::ChunkGuildFilter, prelude::*};

use super::env_variables::get_command_phrases;

#[command]
pub async fn niveis(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let rank_commands = get_command_phrases();
    let rank_commands = &rank_commands[..&rank_commands.len() - 1];
    let msg_content = rank_commands.iter().fold("".to_owned(), |acc, command| {
        if acc.len() == 0 {
            command.to_owned()
        } else {
            format!("{}\n{}", acc, command)
        }
    });

    let dm = msg
        .author
        .direct_message(&ctx, |msg| msg.content(&msg_content))
        .await;
    match dm {
        Ok(_) => {}
        Err(why) => {
            println!("Err sending DM: {:?}", why);
            let _ = msg.reply(&ctx, &msg_content).await;
        }
    };

    Ok(())
}

#[command]
pub async fn tabela(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let rank_roles = &RANK_ROLES[1..];
    let rank_roles_set: HashSet<u64> = rank_roles.iter().map(|&u| u).collect();
    let mut role_count = vec![0u16; rank_roles.len()];

    if let Some(guild) = msg.guild(&ctx.cache).await {
        let guild_id = guild.id;
        ctx.shard
            .chunk_guild(guild_id, Some(10000), ChunkGuildFilter::None, None);
        
        if let Some(members) = ctx.cache.guild_field(guild_id, |g| g.members.to_owned()).await {
            let members: Vec<&Member> = members.iter().map(|entry| entry.1).collect();
            for &member in members.iter() {
                member.roles.iter().for_each(|roleid| {
                    if rank_roles_set.contains(&roleid.0) {
                        let role_ind = rank_roles.iter().position(|&x| x == roleid.0).unwrap();
                        role_count[role_ind] += 1;
                    }
                });
            }

            let dist_msg =
                role_count
                    .iter()
                    .enumerate()
                    .fold("".to_owned(), |acc, (i, next_count)| {
                        let count_str = format!("{}: {}", &RANK_NAMES[i], next_count);
                        if acc.len() == 0 {
                            count_str
                        } else {
                            format!("{}\n{}", &acc, &count_str)
                        }
                    });
            let _ = msg.channel_id.say(&ctx.http, dist_msg).await;
        }
    } else {
        let _ = msg
            .author
            .direct_message(&ctx, |msg| {
                msg.content("Este comando funciona apenas quando invocado no servidor.")
            })
            .await;
    }

    Ok(())
}
