pub mod commands;
use commands::env_variables::QuizSettings;
use serenity::prelude::*;

use std::{collections::HashMap, sync::Arc};

pub struct RankCommands;
impl TypeMapKey for RankCommands {
    type Value = Arc<HashMap<u64, String>>;
}

pub struct RankQuizzes;
impl TypeMapKey for RankQuizzes {
    type Value = Arc<HashMap<String, QuizSettings>>;
}

pub mod utils {
    use std::ops::Shr;
    pub struct Pipe<T>(T);

    impl<T> Pipe<T> {
        pub fn new(content: T) -> Pipe<T> {
            Pipe(content)
        }
    }

    impl<A, B, F> Shr<F> for Pipe<A>
    where
        F: FnOnce(A) -> B,
    {
        type Output = Pipe<B>;

        fn shr(self, func: F) -> Self::Output {
            Pipe(func(self.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::commands::env_variables::{
        load_quiz_commands, load_quiz_ids, load_quiz_settings, load_rank_names, load_rank_roles,
    };
    use super::commands::levelup::get_quiz_key;
    use lazy_static::lazy_static;
    use serde_json as json;
    use serde_json::Value;

    lazy_static! {
        static ref ENV_VARS_TEST: Value = {
            let mut env_dir = std::env::current_dir().expect("Can't find path to executable");
            env_dir.push("env_example.json");
            let env_data =
                std::fs::read_to_string(env_dir).expect("Failed to read env_example.json");
            json::from_str(&env_data).expect("Failed to parse json")
        };
    }

    #[test]
    fn test_env_var_loaders() {
        let rank_names = load_rank_names(&ENV_VARS_TEST);
        assert_eq!(rank_names[0], "新米少佐");

        let rank_roles = load_rank_roles(&ENV_VARS_TEST);
        assert_eq!(rank_roles[0], 0);
        assert_eq!(rank_roles[1], 847610868232618015);

        let quiz_commands = load_quiz_commands(&ENV_VARS_TEST);
        assert_eq!(quiz_commands[0], "k!quiz n5 nd atl=16 12 size=60 mmq=1");

        let quiz_settings = load_quiz_settings(&ENV_VARS_TEST, &rank_roles);
        assert_eq!(quiz_settings[0], (12, 16001, 60, "any", rank_roles[1], 0));

        let quiz_ids = load_quiz_ids(&ENV_VARS_TEST);
        assert_eq!(quiz_ids[0], "JLPT N5");
    }

    #[test]
    fn test_get_quiz_key() {
        let sample_kotoba_report: Value = json::from_str("{
            \"decks\":[
                {
                    \"name\":\"Kanken 2級 Reading Quiz\",
                    \"shortName\":\"2k\",
                    \"uniqueId\":\"kanken_2k\",
                    \"mc\":false
                },
                {
                    \"name\":\"YYYY's New Quiz\",
                    \"shortName\":\"new_con_book\",
                    \"uniqueId\":\"0e50357b-59a5-44c9-9de3-02120f5bcbf3\",
                    \"startIndex\":2000,
                    \"endIndex\":3300,
                    \"mc\":false,
                    \"internetDeck\":true
                },
                {
                    \"name\":\"JLPT N1 Listening Quiz\",
                    \"shortName\":\"ln1\",
                    \"uniqueId\":\"ln1\",
                    \"mc\":false
                }
            ],
            \"sessionName\":\"Multiple Deck Quiz\",
            \"startTime\":\"2021-07-29T02:59:53.964Z\",
            \"endTime\":\"2021-07-29T03:00:07.537Z\",
            \"settings\":{
                \"scoreLimit\":30,
                \"unansweredQuestionLimit\":5,
                \"answerTimeLimitInMs\":25000,
                \"newQuestionDelayAfterUnansweredInMs\":0,
                \"newQuestionDelayAfterAnsweredInMs\":0,
                \"additionalAnswerWaitTimeInMs\":0,
                \"fontSize\":40,
                \"fontColor\":\"rgb(0, 0, 0)\",
                \"backgroundColor\":\"rgb(255, 255, 255)\",
                \"font\":\"AC Gyousho\",
                \"maxMissedQuestions\":1,
                \"shuffle\":true,
                \"serverSettings\":{
                    \"bgColor\":\"rgb(255, 255, 255)\",
                    \"fontFamily\":\"Noto Sans CJK\",
                    \"color\":\"rgb(0, 0, 0)\",
                    \"size\":92,
                    \"additionalAnswerWaitWindow\":2.1,
                    \"answerTimeLimit\":16,
                    \"conquestAndInfernoEnabled\":true,
                    \"internetDecksEnabled\":true,
                    \"delayAfterAnsweredQuestion\":2.2,
                    \"delayAfterUnansweredQuestion\":3,
                    \"scoreLimit\":10,
                    \"unansweredQuestionLimit\":5,
                    \"maxMissedQuestions\":0,
                    \"shuffle\":true
                },
                \"inlineSettings\":{
                    \"fontFamily\":\"AC Gyousho\",
                    \"size\":40,
                    \"delayAfterUnansweredQuestion\":0,
                    \"delayAfterAnsweredQuestion\":0,
                    \"additionalAnswerWaitWindow\":0,
                    \"aliases\":[\"nodelay\",\"nd\"],
                    \"answerTimeLimit\":25,
                    \"maxMissedQuestions\":1,
                    \"scoreLimit\":30
                }
            },
            \"isLoaded\":false
        }").unwrap();
        let quiz_ids = load_quiz_ids(&ENV_VARS_TEST);
        let quiz_key = get_quiz_key(&sample_kotoba_report["decks"]);
        assert_eq!(&quiz_key, quiz_ids[5]);
    }
}
