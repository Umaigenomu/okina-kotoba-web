use std::{collections::HashMap, sync::Arc};

use crate::{
    commands::env_variables::{
        ANNOUNCEMENT_CHANNEL_ID, KOTOBA_API_URL, KOTOBA_BOT_ID, QUIZ_IDS, RANK_NAMES, RANK_ROLES,
        SERVER_ID,
    },
    RankCommands, RankQuizzes,
};

use regex::Regex;
use serde_json::Value;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use super::env_variables::QuizSettings;

fn get_current_next_rank(role_ids: &[u64]) -> (u64, u64) {
    let mut current_rank = RANK_ROLES[0];
    let mut next_rank = RANK_ROLES[1];

    RANK_ROLES.iter().enumerate().for_each(|(i, rank_id)| {
        if role_ids.contains(rank_id) {
            current_rank = RANK_ROLES[i];
            next_rank = {
                if i < RANK_ROLES.len() - 1 {
                    RANK_ROLES[i + 1]
                } else {
                    RANK_ROLES[i]
                }
            };
        }
    });
    (current_rank, next_rank)
}

fn get_next_command(current_rank: &u64, rank_commands: &Arc<HashMap<u64, String>>) -> String {
    let text = rank_commands
        .get(&current_rank)
        .expect("failed to retrieve command for rank");
    let command_regex = Regex::new(r"`(.*)`").unwrap();
    let capture = command_regex.captures(text);
    if let Some(cap) = capture {
        cap.get(1).map_or("", |m| m.as_str()).to_owned()
    } else {
        text.to_owned()
    }
}

#[command]
pub async fn levelup(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut current_rank = RANK_ROLES[0];

    // let roleids: Vec<u64> = ....await.unwrap()       : Guild
    //     .member(&ctx, msg.author.id).await.unwrap()  : Member
    //     .roles(&ctx.cache).await.unwrap()            : Vec<Role>
    //     .iter().map(|role| role.id.0).collect();     : Vec<u64>

    // This is equivalent to the above ^
    if let Ok(guild) = Guild::get(&ctx.http, *SERVER_ID).await {
        if let Ok(user) = guild.member(&ctx.http, msg.author.id).await {
            let role_ids: Vec<u64> = user.roles.iter().map(|roleid| roleid.0).collect();

            let (actual_cur_rank, _) = get_current_next_rank(&role_ids);
            current_rank = actual_cur_rank;
        }
    }

    let rank_commands = {
        let client_read_lock = ctx.data.read().await;
        client_read_lock
            .get::<RankCommands>()
            .expect("Fail to retrieve RankCommands")
            .clone()
    };

    let next_command = get_next_command(&current_rank, &rank_commands);

    let msg_content = format!(
        "O próximo comando para subir de nível é:\n`{}`",
        next_command
    );

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

fn get_quiz_predicates(
    kotoba_report: &Value,
    settings: &QuizSettings,
) -> (bool, bool, bool, bool, bool, bool) {
    // Quiz stats and metadata
    let main_partic_score = kotoba_report["scores"][0]["score"].as_u64().unwrap();
    let participant_count = kotoba_report["participants"].as_array().unwrap().len();

    let question_count = kotoba_report["questions"].as_array().unwrap().len();
    let score_limit = kotoba_report["settings"]["scoreLimit"].as_u64().unwrap();
    let question_score_delta = question_count as i64 - score_limit as i64;

    let answer_time_limit = kotoba_report["settings"]["answerTimeLimitInMs"]
        .as_u64()
        .unwrap();
    let font_size = kotoba_report["settings"]["fontSize"].as_u64().unwrap();
    let font = kotoba_report["settings"]["font"].as_str().unwrap();
    let is_shuffle = kotoba_report["settings"]["shuffle"].as_bool().unwrap();
    let is_loaded = kotoba_report["isLoaded"].as_bool().unwrap();

    let mut start_index = 0;
    let mut end_index = 0;
    if let Some(ind) = kotoba_report["decks"][0].get("startIndex") {
        start_index = ind.as_u64().unwrap();
    }
    if let Some(ind) = kotoba_report["decks"][0].get("endIndex") {
        end_index = ind.as_u64().unwrap();
    }
    let is_mc = kotoba_report["decks"][0]["mc"].as_bool().unwrap();

    // Requirements according to RankQuizzes
    let (
        score_limit_setting,
        answer_time_limit_setting,
        font_size_setting,
        font_setting,
        _next_rankrole_id,
        allowed_failed_question_count,
    ) = *settings;

    // Predicates
    let invalid_settings = start_index != 0 || end_index != 0 || is_mc || !is_shuffle || is_loaded;
    let score_doesnt_match =
        main_partic_score != score_limit || score_limit < score_limit_setting as u64;
    let more_than_one_user = participant_count > 1;
    let time_limit_too_low = answer_time_limit > answer_time_limit_setting as u64;
    let wrong_font_settings =
        font_setting != "any" && (font_setting != font || font_size_setting as u64 != font_size);
    let failed_too_many =
        question_score_delta < 0i64 || question_score_delta > allowed_failed_question_count as i64;

    (
        invalid_settings,
        score_doesnt_match,
        more_than_one_user,
        time_limit_too_low,
        wrong_font_settings,
        failed_too_many,
    )
}

pub fn get_quiz_key(decks: &Value) -> String {
    decks
        .as_array()
        .unwrap()
        .iter()
        .map(|deck| {
            let deck_id = deck["uniqueId"].as_str().unwrap().to_owned();
            if let Some(start) = deck["startIndex"].as_u64() {
                if let Some(end) = deck["endIndex"].as_u64() {
                    return format!("{}({}-{})", &deck_id, start, end);
                }
            } 
            deck_id
        })
        .fold("".to_owned(), |acc, next_deck| {
            if acc.is_empty() {
                next_deck
            } else {
                format!("{}+{}", &acc, &next_deck)
            }
        })
}

pub async fn on_kotoba_msg(args: (Context, Message)) -> (Context, Message) {
    let (ctx, msg) = args;
    if msg.author.id.0 != KOTOBA_BOT_ID {
        return (ctx, msg);
    }

    let game_report_regex = Regex::new(r"game_reports/([^)]*)\)").unwrap();

    let useful_embed_fields = msg
        .embeds
        .iter()
        .filter(|embed| {
            embed
                .title
                .as_ref()
                .unwrap_or(&"_".to_owned())
                .contains("Ended")
        })
        .flat_map(|embed| {
            embed
                .fields
                .iter()
                .filter(|field| field.value.contains("[View a report for this game]"))
                .collect::<Vec<&EmbedField>>()
        })
        .collect::<Vec<&EmbedField>>();

    for &field in useful_embed_fields.iter() {
        let quiz_id = game_report_regex
            .captures(&field.value)
            .unwrap_or_else(|| panic!("Report id not found for field value {}", &field.value))
            .get(1)
            .expect("Regex didn't capture anything")
            .as_str();

        // API request
        let api_url = format!("{}{}", KOTOBA_API_URL, quiz_id);

        let kotoba_report = reqwest::get(&api_url)
            .await
            .unwrap_or_else(|_| panic!("Request to url: {} has failed.", &api_url))
            .json::<Value>()
            .await
            .unwrap();

        // Get quiz settings from RankQuizzes (global data) and then predicates
        let quiz_key: String = get_quiz_key(&kotoba_report["decks"]);

        let settings = {
            let client_read_lock = ctx.data.read().await;
            client_read_lock
                .get::<RankQuizzes>()
                .expect("Failed to retrieve RankQuizzes")
                .clone()
        };
        let settings = {
            let settings_opt = settings.get(&quiz_key);
            if let Some(s) = settings_opt {
                s
            } else {
                continue;
            }
        };

        let (
            invalid_settings,
            score_doesnt_match,
            more_than_one_user,
            time_limit_too_low,
            wrong_font_settings,
            failed_too_many,
        ) = get_quiz_predicates(&kotoba_report, &settings);

        // Checking if quiz was valid and if so give new role
        if invalid_settings
            || score_doesnt_match
            || more_than_one_user
            || time_limit_too_low
            || wrong_font_settings
            || failed_too_many
        {
            if &quiz_key == QUIZ_IDS.last().unwrap() {
                let _ = msg.channel_id.say(&ctx.http, "馬鹿だねぇ。断言しよう！　今のお前が私に勝つことは不可能だ。\nお前の背中に四季の扉がある限り、勝負など茶番でしか無い。").await;
            }
            continue;
        } else {
            // according to quiz results
            let next_calculated_rank = settings.4;
            // according to member's current role
            let participant_id = kotoba_report["participants"][0]["discordUser"]["id"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let mut member = Guild::get(&ctx.http, *SERVER_ID)
                .await
                .unwrap()
                .member(&ctx.http, participant_id)
                .await
                .unwrap();
            let role_ids: Vec<u64> = member.roles.iter().map(|roleid| roleid.0).collect();
            let (current_rank, next_rank) = get_current_next_rank(&role_ids);

            if next_rank == next_calculated_rank && current_rank != next_rank {
                let quiz_name = kotoba_report["decks"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|deck| deck["name"].as_str().unwrap())
                    .fold("".to_owned(), |acc, next_deck| {
                        if acc.is_empty() {
                            next_deck.to_owned()
                        } else {
                            format!("{}, {}", &acc, next_deck)
                        }
                    });

                let _ = member.remove_role(&ctx.http, current_rank).await;
                let adr = member.add_role(&ctx.http, next_rank).await;
                if adr.is_err() {
                    let _ = msg.channel_id.say(
                        &ctx.http,
                         "Não possuo permissão para te dar o cargo do próximo nível. Entre em contato com um moderador."
                    ).await;
                }

                let ann_channel = ChannelId(*ANNOUNCEMENT_CHANNEL_ID);

                if next_rank == *RANK_ROLES.last().unwrap() {
                    let _ = ann_channel.say(&ctx.http, format!(
                        "<@!{}> passou no(s) quiz(es): {}!\nやられたー！　さすが伯刺西爾の大将ね。この私の攻撃を避けきるなんて。", 
                        participant_id, &quiz_name
                    )).await;
                } else {
                    let next_rank_name = RANK_NAMES[RANK_ROLES[1..]
                        .iter()
                        .position(|&x| x == next_rank)
                        .unwrap()];

                    let _ = ann_channel.say(&ctx.http, format!(
                        "<@!{}> passou no(s) quiz(es): {}, e agora é um(a) {}! Parabéns!\nO próximo nível pode ser verificado através do comando `%levelup`.", 
                        participant_id, &quiz_name, next_rank_name
                    )).await;
                }
            }
        }
    }

    (ctx, msg)
}
