use crate::graphics::messages::StatEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modifier {
    pub stat: Option<StatEnum>,
    pub skill: Option<String>,
    pub modifier: isize,
    pub has_condition: bool,
    pub origin: String,
}

impl Modifier {
    pub fn value(&self) -> isize {
        if !self.has_condition {
            self.modifier
        } else {
            0
        }
    }
    pub fn skill_points() -> Self {
        Self {
            stat: None,
            skill: None,
            modifier: 0,
            has_condition: false,
            origin: "Skill points".to_string(),
        }
    }
    pub fn stat_bonus(governing_stat: StatEnum) -> Self {
        Self {
            stat: Some(governing_stat.clone()),
            skill: None,
            modifier: 0,
            has_condition: false,
            origin: governing_stat.to_string(),
        }
    }
}
