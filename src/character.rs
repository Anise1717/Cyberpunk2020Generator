use iced::{application::IntoBoot, futures::stream::TrySkipWhile};

use crate::{
    graphics::messages::{Message, StatEnum},
    skills::{self, Modifier},
    stats::Stats,
    weapons,
};

use std::{collections::HashMap, iter::Skip};

fn skill_map(skills: &[&str], governing_stat: StatEnum) -> HashMap<String, Vec<Modifier>> {
    skills
        .iter()
        .map(|skill| {
            (
                skill.to_string(),
                vec![
                    Modifier::skill_points(),
                    Modifier::stat_bonus(governing_stat),
                ],
            )
        })
        .collect()
}

fn ability_map(abilities: &[&str]) -> HashMap<String, Vec<Modifier>> {
    abilities
        .iter()
        .map(|ability| (ability.to_string(), vec![Modifier::skill_points()]))
        .collect()
}
// make all skills one hashmap and load new characters from json template
#[derive(Default)]
pub struct Character {
    handle: String,
    role: Option<Role>,
    pub character_points: isize,
    stats: Stats,

    pub special_abilities: HashMap<String, Vec<Modifier>>,
    attraction_skills: HashMap<String, Vec<Modifier>>,
    body_skills: HashMap<String, Vec<Modifier>>,
    cool_skills: HashMap<String, Vec<Modifier>>,
    empathy_skills: HashMap<String, Vec<Modifier>>,
    pub intelligence_skills: HashMap<String, Vec<Modifier>>,
    reflex_skills: HashMap<String, Vec<Modifier>>,
    tech_skill: HashMap<String, Vec<Modifier>>,
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
                let temp = self.skill_helper(stat).get_mut(&skill);
                match temp {
                    Some(skill) => {
                        skill
                            .iter_mut()
                            .filter(|x| x.origin == "Skill points")
                            .for_each(|x| x.modifier += 1);
                    }
                    _ => {
                        eprintln!("fuck");
                    }
                }
            }
            Message::SkillDecreased(skill, stat) => {
                let temp = self.skill_helper(stat).get_mut(&skill);
                match temp {
                    Some(skill) => {
                        skill
                            .iter_mut()
                            .filter(|x| x.origin == "Skill points")
                            .for_each(|x| x.modifier -= 1);
                    }
                    _ => {
                        eprintln!("fuck");
                    }
                }
            }

            Message::SetSkill(skill, stat, value) => {
                if let Ok(parsed) = value.parse::<isize>() {
                    let temp = self.skill_helper(stat).get_mut(&skill);
                    match temp {
                        Some(skill) => {
                            skill
                                .iter_mut()
                                .filter(|x| x.origin == "Skill points")
                                .for_each(|x| x.modifier = parsed);
                        }
                        _ => {
                            eprintln!("fuck");
                        }
                    }
                }
            }
            _ => {}
        }
    }
    fn skill_helper(&mut self, stat: StatEnum) -> &mut HashMap<String, Vec<Modifier>> {
        match stat {
            StatEnum::Body => &mut self.body_skills,
            StatEnum::Tech => &mut self.tech_skill,
            StatEnum::Intelligence => &mut self.intelligence_skills,
            StatEnum::Reflex => &mut self.reflex_skills,
            StatEnum::Cool => &mut self.cool_skills,
            StatEnum::Attractiveness => &mut self.attraction_skills,
            StatEnum::Empathy => &mut self.empathy_skills,
            _ => &mut self.special_abilities,
        }
    }

    pub fn new(name: String, points: isize) -> Self {
        Self {
            handle: name,
            role: None,
            character_points: points,
            stats: Stats::new(points),

            special_abilities: ability_map(&[
                "Authority",
                "Charismatic Leadership",
                "Combat Sense",
                "Credibility",
                "Family",
                "Interface",
                "Jury Rig",
                "Medical Tech",
                "Resources",
                "Streetdeal",
            ]),

            attraction_skills: skill_map(
                &["Personal Grooming", "Wardrobe & Style"],
                StatEnum::Attractiveness,
            ),

            body_skills: skill_map(&["Endurance", "Strength Feat", "Swimming"], StatEnum::Body),

            cool_skills: skill_map(
                &[
                    "Interrogation",
                    "Intimidate",
                    "Oratory",
                    "Resist Torture/Drugs",
                    "Streetwise",
                ],
                StatEnum::Cool,
            ),

            empathy_skills: skill_map(
                &[
                    "Human Perception",
                    "Interview",
                    "Leadership",
                    "Seduction",
                    "Social",
                    "Persuasion & Fast Talk",
                    "Perform",
                ],
                StatEnum::Empathy,
            ),

            intelligence_skills: skill_map(
                &[
                    "Accounting",
                    "Anthropology",
                    "Awareness/Notice",
                    "Biology",
                    "Botany",
                    "Chemistry",
                    "Composition",
                    "Diagnose Illness",
                    "Education & Gen. Know",
                    "Gamble",
                    "Geology",
                    "Hide/Evade",
                    "History",
                    "Library Search",
                    "Mathematics",
                    "Physics",
                    "Programming",
                    "Shadow/Track",
                    "Stock Market",
                    "System Knowledge",
                    "Teaching",
                    "Wilderness Survival",
                    "Zoology",
                ],
                StatEnum::Intelligence,
            ),

            reflex_skills: skill_map(
                &[
                    "Archery",
                    "Athletics",
                    "Brawling",
                    "Dance",
                    "Dodge & Escape",
                    "Driving",
                    "Fencing",
                    "Handgun",
                    "Heavy Weapons",
                    "Melee",
                    "Motorcycle",
                    "Operate Hvy. Machinery",
                    "Pilot (Gyro)",
                    "Pilot (Fixed Wing)",
                    "Pilot (Dirigible)",
                    "Pilot (Vect. Thrust Vehicle)",
                    "Rifle",
                    "Stealth",
                    "Submachinegun",
                ],
                StatEnum::Reflex,
            ),

            tech_skill: skill_map(
                &[
                    "Aero Tech",
                    "AV Tech",
                    "Basic Tech",
                    "Cryotank Operation",
                    "Cyberdeck Design",
                    "CyberTech",
                    "Demolitions",
                    "Disguise",
                    "Electronics",
                    "Elect. Security",
                    "First Aid",
                    "Forgery",
                    "Gyro Tech",
                    "Paint or Draw",
                    "Photo & Film",
                    "Pharmaceuticals",
                    "Pick Lock",
                    "Pick Pocket",
                    "Play Instrument",
                    "Weaponsmith",
                ],
                StatEnum::Tech,
            ),
            // gear: vec![],
            // weapons: vec![],
        }
    }
}

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
