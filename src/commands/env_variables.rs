// EVERY VARIABLE IN THIS FILE MUST BE SET BY THE SERVER'S MODERATOR

use std::collections::HashMap;

pub const KOTOBA_API_URL: &'static str = "https://kotobaweb.com/api/game_reports/";

pub const SERVER_ID: u64 = 336676820176863233;

pub const KOTOBA_BOT_ID: u64 = 251239170058616833;

pub const ANNOUNCEMENT_CHANNEL_ID: u64 = 838580882620809248;

const RANK_NAMES: [&str; 5] = ["新米少佐", "少佐", "中佐", "大佐", "大将"];
pub const RANK_ROLES: [u64; 6] = [
    0,                  // No role
    845821942821158952, // 新米少佐
    845822538978295819, // 少佐
    845822662014140446, // 中佐
    845822770499289099, // 大佐
    845822934730932254, // 大将
];

// By accessing kotoba-web's api, you are able to see each of the decks' unique ids for a quiz report
// for multiple deck quizzes, the unique ids were merged with '+'
pub const QUIZ_IDS: [&str; 5] = [
    "JLPT N4",
    "JLPT N3",
    "JLPT N2+gn2.json",
    "JLPT N1+gn1.json",
    "kanken_2k+kanken_j1k+57cbb7f8-72b0-4361-a0a8-9020441e1d0c",
];
// VALUES STORED ARE:
// score_limit, answer_time_limit_in_ms, fontsize, font, rankrole_obtained, allowed_failed_question_count
pub type QuizSettings = (u32, u32, u32, &'static str, u64, u8);
pub const QUIZ_SETTINGS: [QuizSettings; 5] = [
    (14, 10001, 60, "any", RANK_ROLES[1], 0),
    (18, 10001, 60, "any", RANK_ROLES[2], 0),
    (20, 16001, 40, "AC Gyousho", RANK_ROLES[3], 0),
    (24, 16001, 40, "AC Gyousho", RANK_ROLES[4], 1),
    (30, 12001, 40, "AC Gyousho", RANK_ROLES[5], 1),
];
// Kotoba-web quiz commands built upon the settings above^
const QUIZ_COMMANDS: [&str; 5] = [
    "k!quiz n4 nd atl=10 14 size=60 mmq=1",
    "k!quiz n3 nd atl=10 18 size=60 mmq=1",
    "k!quiz n2+gn2 nd atl=16 20 font=10 size=40 mmq=1",
    "k!quiz n1+gn1 nd atl=16 24 font=10 size=40 mmq=2",
    "k!quiz 2k+j1k+cope nd atl=12 30 font=10 size=40 mmq=2",
];

pub fn get_rank_quizzes() -> HashMap<String, QuizSettings> {
    QUIZ_IDS
        .iter()
        .zip(QUIZ_SETTINGS.iter())
        .map(|(&id, settings)| (id.to_owned(), settings.clone()))
        .collect()
}

// Command messages for each level are defined here
pub fn get_rank_commands() -> HashMap<u64, String> {
    [
        format!("Nível 1 ({}):\n`{}`", RANK_NAMES[0], QUIZ_COMMANDS[0]),
        format!("Nível 2 ({}):\n`{}`", RANK_NAMES[1], QUIZ_COMMANDS[1]),
        format!("Nível 3 ({}):\n`{}`", RANK_NAMES[2], QUIZ_COMMANDS[2]),
        format!("Nível 4 ({}):\n`{}`", RANK_NAMES[3], QUIZ_COMMANDS[3]),
        format!("Nível 5 ({}):\n`{}`", RANK_NAMES[4], QUIZ_COMMANDS[4]),
        "Você já está no nível mais alto. (Parabéns)".to_owned(),
    ]
    .iter()
    .zip(RANK_ROLES.iter())
    .map(|(command, &role_id)| (role_id, command.clone()))
    .collect::<HashMap<u64, String>>()
}
