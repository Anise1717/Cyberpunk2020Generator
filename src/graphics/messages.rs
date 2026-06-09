use serde::{Deserialize, Serialize};
use strum_macros::Display;
#[derive(Debug, Clone)]
pub enum Message {
    SetSkill(String, StatEnum, isize),
    SetStat(isize, StatEnum),
    StatIncreased(StatEnum),
    StatDecreased(StatEnum),
    SkillDecreased(String, StatEnum),
    SkillIncreased(String, StatEnum),
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, Copy)]
pub enum StatEnum {
    Body,
    Luck,
    Intelligence,
    Movement,
    Reflex,
    Tech,
    Cool,
    Attractiveness,
    Empathy,
}
