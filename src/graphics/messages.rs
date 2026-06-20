use serde::{Deserialize, Serialize};
use strum_macros::Display;
#[derive(Debug, Clone)]
pub enum Message {
    //skill,stat,value
    SetSkill(String, StatEnum, String),
    SetStat(isize, StatEnum),
    StatIncreased(StatEnum),
    StatDecreased(StatEnum),
    SkillDecreased(String, StatEnum),
    SkillIncreased(String, StatEnum),
    SaveJson,
    NewCharacter,
    LoadCharacter,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, Copy, PartialEq, Eq, Hash)]
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
    SpecialAbility,
}
