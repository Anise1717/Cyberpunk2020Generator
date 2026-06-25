use serde::{Deserialize, Serialize};

use crate::{
    gear,
    graphics::messages::{Message, StatEnum},
    skills::{self, Modifier},
    stats::Stats,
    weapons::Weapon,
};

use std::{collections::HashMap, io::Read};
//move to helper function file

// fn skill_map(skills: &[&str], governing_stat: StatEnum) -> HashMap<String, Vec<Modifier>> {
//     skills
//         .iter()
//         .map(|skill| {
//             (
//                 skill.to_string(),
//                 vec![
//                     Modifier::skill_points(),
//                     Modifier::stat_bonus(governing_stat),
//                 ],
//             )
//         })
//         .collect()
// }

// fn ability_map(abilities: &[&str]) -> HashMap<String, Vec<Modifier>> {
//     abilities
//         .iter()
//         .map(|ability| (ability.to_string(), vec![Modifier::skill_points()]))
//         .collect()
// }

#[derive(Default, Serialize, Deserialize)]
pub struct Character {
    handle: String,
    role: Option<Role>,
    pub character_points: isize,
    pub stats: Stats,
    pub skills: HashMap<StatEnum, HashMap<String, Vec<Modifier>>>,
    pub gear: Vec<gear::Gear>,
    pub weapons: Vec<Weapon>,
}

impl Character {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::StatIncreased(stat) => {
                *self.stats.get_mut(stat) += 1;
            }
            Message::StatDecreased(stat) => {
                *self.stats.get_mut(stat) -= 1;
            }
            Message::SkillIncreased(skill, stat) => {
                self.skill_inc_helper(&skill, stat, 1);
            }
            Message::SkillDecreased(skill, stat) => {
                self.skill_inc_helper(&skill, stat, -1);
            }
            Message::SetSkill(skill, stat, value) => {
                if let Ok(parsed) = value.parse::<isize>() {
                    self.set_skill(&skill, stat, parsed);
                }
            }
            _ => {}
        }
    }

    pub fn new(name: String, points: isize, mut json: std::fs::File) -> Self {
        let mut file = String::new();
        json.read_to_string(&mut file).expect("failed to read file");
        let mut temp: Character = serde_json::from_str(&file).expect("Invalid json");
        temp.handle = name;
        temp.character_points = points;
        temp
    }
    // helper function of augmenting skill points by 1
    fn skill_inc_helper(&mut self, skill: &str, stat: StatEnum, value: isize) {
        if let Some(stat_map) = self.skills.get_mut(&stat) {
            if let Some(modifiers) = stat_map.get_mut(skill) {
                modifiers
                    .iter_mut()
                    .filter(|x| x.origin == "Skill points")
                    .for_each(|x| x.modifier += value);
            } else {
                eprintln!("skill not found: {}", skill);
            }
        } else {
            eprintln!("stat not found");
        }
    }
    //internal setter
    fn set_skill(&mut self, skill: &str, stat: StatEnum, value: isize) {
        if let Some(stat_map) = self.skills.get_mut(&stat) {
            if let Some(modifiers) = stat_map.get_mut(skill) {
                modifiers
                    .iter_mut()
                    .filter(|x| x.origin == "Skill points")
                    .for_each(|x| x.modifier = value);
            } else {
                eprintln!("skill not found: {}", skill);
            }
        } else {
            eprintln!("stat not found");
        }
    }
}
#[derive(Serialize, Deserialize)]
enum Role {
    Solo,
    Rocker,
    Netrunner,
    Media,
    Nomad,
    Fixer,
    Cop,
    Corp,
    Techie,
    Medtech,
}
