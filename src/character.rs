use crate::{
    graphics::messages::{Message, StatEnum},
    skills::{self, Modifier},
    stats::Stats,
    weapons,
};

use std::collections::HashMap;

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

#[derive(Default)]
pub struct Character {
    handle: String,
    role: Option<Role>,
    pub character_points: isize,
    stats: Stats,
    pub skills: HashMap<StatEnum, HashMap<String, Vec<Modifier>>>,
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
                if let Some(stat_map) = self.skills.get_mut(&stat) {
                    if let Some(modifiers) = stat_map.get_mut(&skill) {
                        modifiers
                            .iter_mut()
                            .filter(|x| x.origin == "Skill points")
                            .for_each(|x| x.modifier += 1);
                    } else {
                        eprintln!("skill not found: {}", skill);
                    }
                } else {
                    eprintln!("stat not found");
                }
            }
            Message::SkillDecreased(skill, stat) => {
                if let Some(stat_map) = self.skills.get_mut(&stat) {
                    if let Some(modifiers) = stat_map.get_mut(&skill) {
                        modifiers
                            .iter_mut()
                            .filter(|x| x.origin == "Skill points")
                            .for_each(|x| x.modifier -= 1);
                    } else {
                        eprintln!("skill not found: {}", skill);
                    }
                } else {
                    eprintln!("stat not found");
                }
            }
            Message::SetSkill(skill, stat, value) => {
                if let Ok(parsed) = value.parse::<isize>() {
                    if let Some(stat_map) = self.skills.get_mut(&stat) {
                        if let Some(modifiers) = stat_map.get_mut(&skill) {
                            modifiers
                                .iter_mut()
                                .filter(|x| x.origin == "Skill points")
                                .for_each(|x| x.modifier = parsed);
                        } else {
                            eprintln!("skill not found: {}", skill);
                        }
                    } else {
                        eprintln!("stat not found");
                    }
                }
            }
            _ => {}
        }
    }

    pub fn new(name: String, points: isize) -> Self {
        let mut skills = HashMap::new();

        skills.insert(
            StatEnum::Luck,
            ability_map(&[
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
        );

        skills.insert(
            StatEnum::Attractiveness,
            skill_map(
                &["Personal Grooming", "Wardrobe & Style"],
                StatEnum::Attractiveness,
            ),
        );

        skills.insert(
            StatEnum::Body,
            skill_map(&["Endurance", "Strength Feat", "Swimming"], StatEnum::Body),
        );

        skills.insert(
            StatEnum::Cool,
            skill_map(
                &[
                    "Interrogation",
                    "Intimidate",
                    "Oratory",
                    "Resist Torture/Drugs",
                    "Streetwise",
                ],
                StatEnum::Cool,
            ),
        );

        skills.insert(
            StatEnum::Empathy,
            skill_map(
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
        );

        skills.insert(
            StatEnum::Intelligence,
            skill_map(
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
        );

        skills.insert(
            StatEnum::Reflex,
            skill_map(
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
        );

        skills.insert(
            StatEnum::Tech,
            skill_map(
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
        );

        Self {
            handle: name,
            role: None,
            character_points: points,
            stats: Stats::new(points),
            skills,
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
