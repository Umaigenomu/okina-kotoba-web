use std::collections::HashSet;

use crate::commands::env_variables::{RANK_NAMES, RANK_ROLES, QUIZ_COMMANDS};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::{prelude::*};

use super::env_variables::get_command_phrases;

#[command]
pub async fn nivel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let parse_level = args.rest().trim().parse::<u16>();
    if let Ok(level) = parse_level {
        if level > 0 && level < 7 {
            let _ = msg.reply(&ctx, &format!("`{}`", QUIZ_COMMANDS[(level - 1) as usize])).await;
        } else {
            let _ = msg.reply(&ctx, "Os níveis são de 1 a 6.").await;
        }
    }

    Ok(())
}

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
        let mem_count = guild.member_count as f64;
        let chunks = std::cmp::max((mem_count / 1000f64).ceil() as i64 - 1, 0);
        
        if let Ok(mut members) = guild.members(&ctx.http, Some(1000), None).await {
            for _ in 0..chunks {
                if let Ok(mut members_nested) = guild.members(&ctx.http, Some(1000), members.last().unwrap().user.id).await {
                    members.append(&mut members_nested);
                }
            }


            for member in members.iter() {
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
