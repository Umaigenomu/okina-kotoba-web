use lazy_static::lazy_static;
use serde_json as json;
use serde_json::Value;
use std::{collections::HashMap, env, fs};

fn load_env() -> Value {
    let mut env_dir = env::current_exe().expect("Can't find path to executable");
    env_dir.pop();
    env_dir.push("env.json");
    let env_data = fs::read_to_string(env_dir).expect("Failed to read env.json");
    json::from_str(&env_data).expect("Unable to parse ")
}

pub fn load_rank_names(env_vars: &'static Value) -> Vec<&'static str> {
    let rank_names = env_vars["RANK_NAMES"]
        .as_array()
        .expect("Failed to read rank_names from env.json");
    rank_names
        .iter()
        .map(|name| name.as_str().expect("Rank names must all be strings."))
        .collect()
}

pub fn load_rank_roles(env_vars: &'static Value) -> Vec<u64> {
    let roles = env_vars["RANK_ROLE_IDS"]
        .as_array()
        .expect("Failed to read rank_role_ids from env.json");
    let role_ids: Vec<u64> = roles
        .iter()
        .map(|id| {
            id.as_u64()
                .expect("Rank role ids must all be unsigned ints.")
        })
        .collect();
    // First elem is 0 => no role
    [&[0], &role_ids[..]].concat()
}

pub fn load_quiz_settings(env_vars: &'static Value, rank_roles: &[u64]) -> Vec<QuizSettings> {
    let quiz_settings = env_vars["QUIZ_SETTINGS"]
        .as_array()
        .expect("Failed to read quiz_settings from env.json");
    quiz_settings
        .iter()
        .zip(&rank_roles[1..])
        .map(|(setting, role)| {
            (
                setting["num_questions"].as_u64().unwrap() as u32,
                setting["time_limit_ms"].as_u64().unwrap() as u32,
                setting["size"].as_u64().unwrap() as u32,
                setting["font"].as_str().unwrap(),
                *role,
                setting["misses"].as_u64().unwrap() as u8,
            )
        })
        .collect()
}

pub fn load_quiz_commands(env_vars: &'static Value) -> Vec<&'static str> {
    let commands = env_vars["QUIZ_COMMANDS"]
        .as_array()
        .expect("Failed to read quiz_commands from env.json");
    commands
        .iter()
        .map(|cmd| cmd.as_str().expect("Quiz commands must be strings"))
        .collect()
}

pub fn load_quiz_ids(env_vars: &'static Value) -> Vec<&'static str> {
    let ids = env_vars["QUIZ_IDS"]
        .as_array()
        .expect("Failed to read quiz_ids from env.json");
    ids.iter()
        .map(|id| id.as_str().expect("Quiz ids must be strings"))
        .collect()
}

lazy_static! {
    pub static ref ENV_VARS: Value = load_env();
    pub static ref SERVER_ID: u64 = {
        ENV_VARS["SERVER_ID"].as_u64().expect("Failed to read server_id from env.json")
    };
    pub static ref ANNOUNCEMENT_CHANNEL_ID: u64 = {
        ENV_VARS["ANNOUNCEMENT_CHANNEL_ID"].as_u64()
        .expect("Failed to read announcement_channel_id from env.json")
    };
    pub static ref RANK_NAMES: Vec<&'static str> = load_rank_names(&ENV_VARS);
    pub static ref RANK_ROLES: Vec<u64> = load_rank_roles(&ENV_VARS);
    pub static ref QUIZ_SETTINGS: Vec<QuizSettings> = load_quiz_settings(&ENV_VARS, &RANK_ROLES);
    // Kotoba-web quiz commands built upon the settings above^
    pub static ref QUIZ_COMMANDS: Vec<&'static str> = load_quiz_commands(&ENV_VARS);
    // By accessing kotoba-web's api, you are able to see each of the decks' unique ids for a quiz report
    // for multiple deck quizzes, the unique ids were merged with '+'
    pub static ref QUIZ_IDS: Vec<&'static str> = load_quiz_ids(&ENV_VARS);
}

// score_limit, answer_time_limit_in_ms, fontsize, font, rankrole_obtained, allowed_failed_question_count
pub type QuizSettings = (u32, u32, u32, &'static str, u64, u8);

// Command messages for each level are defined here; LINE 77 AND 79 ARE DEFINED BY USER
pub fn get_command_phrases() -> Vec<String> {
    let mut phrases: Vec<String> = RANK_NAMES
        .iter()
        .enumerate()
        .zip(QUIZ_COMMANDS.iter())
        .map(|((i, &name), &command)| format!("Nível {} ({}):\n`{}`", i + 1, name, command))
        .collect();
    phrases.push("Você já está no nível mais alto. (Parabéns)".to_owned());
    phrases
}
// ========================================================================================================

// Function used by the app;
pub fn get_rank_quizzes() -> HashMap<String, QuizSettings> {
    QUIZ_IDS
        .iter()
        .zip(QUIZ_SETTINGS.iter())
        .map(|(&id, settings)| (id.to_owned(), *settings))
        .collect()
}
// Function used by the app;
pub fn get_rank_commands() -> HashMap<u64, String> {
    get_command_phrases()
        .iter()
        .zip(RANK_ROLES.iter())
        .map(|(command, &role_id)| (role_id, command.clone()))
        .collect::<HashMap<u64, String>>()
}

// CONSTANTS NOT DEFINED BY SERVER
pub const KOTOBA_API_URL: &str = "https://kotobaweb.com/api/game_reports/";
pub const KOTOBA_BOT_ID: u64 = 251239170058616833;
