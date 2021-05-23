use crate::commands::env_variables::{
    SERVER_ID,
    ANNOUNCEMENT_CHANNEL_ID,
    RANK_ROLES,
    get_rank_commands,
    get_rank_quizzes
};

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use regex::Regex;


// let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
// let text = "2012-03-14, 2013-01-01 and 2014-07-05";
// for cap in re.captures_iter(text) {
//     println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
// }
// Output:
// Month: 03 Day: 14 Year: 2012
// Month: 01 Day: 01 Year: 2013
// Month: 07 Day: 05 Year: 2014

#[command]
pub async fn levelup(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    Ok(())
}
