use std::{
    collections::HashMap
};

pub const SERVER_ID: u64 = 336676820176863233;

pub const ANNOUNCEMENT_CHANNEL_ID: u64 = 838580882620809248; 

pub const RANK_ROLES: [u64; 6] = [
    845821942821158952,  // 新米少佐
    845822538978295819,  // 少佐
    845822662014140446,  // 中佐
    845822770499289099,  // 大佐
    845822934730932254,   // 大将
    0 // DONE
];
const RANK_NAMES: [&str; 5] = [
    "新米少佐",
    "少佐",
    "中佐",
    "大佐",
    "大将",
];

// By accessing kotoba-web's api, you are able to see each of the decks' unique ids' for a quiz report
// for multiple deck quizzes, the unique ids were merged with '+'
// values stored are: scorelimit, answertimelimitinms, fontsize, font, nextrankroleid, allowedfailedquestioncount
pub fn get_rank_quizzes() -> HashMap<String, (u32, u32, u32, &'static str, u64, u8)> {
    let mut rank_quizzes  = HashMap::new();
    rank_quizzes.insert(
        "JLPT N4".to_owned(), (14, 10001, 80, "any", RANK_ROLES[1], 0)
    );
    rank_quizzes.insert(
        "JLPT N3".to_owned(), (18, 10001, 60, "any", RANK_ROLES[2], 0)
    );
    rank_quizzes.insert(
        "JLPT N2+gn2.json".to_owned(), (20, 16001, 40, "AC Gyousho", RANK_ROLES[3], 1)
    );
    rank_quizzes.insert(
        "JLPT N1+gn1.json".to_owned(), (24, 16001, 40, "AC Gyousho", RANK_ROLES[4], 1)
    );
    rank_quizzes.insert(
        "kanken_2k+kanken_j1k+57cbb7f8-72b0-4361-a0a8-9020441e1d0c".to_owned(),
        (30, 12001, 40, "AC Gyousho", RANK_ROLES[5], 0)
    );

    rank_quizzes
}

pub fn get_rank_commands() -> HashMap<u64, String> {
    let mut rank_commands  = HashMap::new();
    rank_commands.insert(
        RANK_ROLES[0], format!(
            "Nível 1 ({}):\n`k!quiz n4 nd atl=10 14 size=80 mmq=1`",
             RANK_NAMES[0]
        )
    );
    rank_commands.insert(
        RANK_ROLES[1], format!(
            "Nível 2 ({}):\n`k!quiz n3 nd atl=10 18 size=60 mmq=1`",
             RANK_NAMES[1]
        )
    );
    rank_commands.insert(
        RANK_ROLES[2], format!(
            "Nível 3 ({}):\n`k!quiz n2+gn2 nd atl=16 20 font=10 size=40 mmq=2`",
             RANK_NAMES[2]
        )
    );
    rank_commands.insert(
        RANK_ROLES[3], format!(
            "Nível 4 ({}):\n`k!quiz n1+gn1 nd atl=16 24 font=10 size=40 mmq=2`",
             RANK_NAMES[3]
        )
    );
    rank_commands.insert(
        RANK_ROLES[4], format!(
            "Nível 5 ({}):\n`k!quiz 2k+j1k+cope nd atl=16 30 font=10 size=40 mmq=2`",
             RANK_NAMES[4]
        )
    );
    rank_commands.insert(
        RANK_ROLES[5], "Você já está no nível mais alto. (Parabéns)".to_owned()
    );

    rank_commands
}
