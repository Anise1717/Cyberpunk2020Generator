mod skills {

    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    struct CustomSkill {
        name: String,
        value: isize,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct SpecialAbilities {
        authority: isize,
        charismatic_leadership: isize,
        combat_sense: isize,
        credibility: isize,
        family: isize,
        interface: isize,
        jury_rig: isize,
        medical_tech: isize,
        resources: isize,
        street_deal: isize,
        custom_skills: Vec<CustomSkill>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct AttractionSkills {
        personal_grooming: isize,
        wardrobe_and_style: isize,
        custom_skills: Vec<CustomSkill>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct BodySkills {
        endurance: isize,
        strength_feat: isize,
        swimming: isize,
        custom_skills: Vec<CustomSkill>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CoolWillSkills {
        interrogation: isize,
        intimidate: isize,
        oratory: isize,
        resist_torture_drugs: isize,
        streetwise: isize,
        custom_skills: Vec<CustomSkill>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct EmpathySkills {
        human_perception: isize,
        interview: isize,
        leadership: isize,
        seduction: isize,
        social: isize,
        persuasion_and_fast_talk: isize,
        perform: isize,
        custom_skills: Vec<CustomSkill>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct IntellegenceSkills {
        accounting: isize,
        anthropology: isize,
        awareness_notice: isize,
        biology: isize,
        botany: isize,
        chemistry: isize,
        composition: isize,
        diagnose_illness: isize,
        education_and_gen_know: isize,
        expert: Vec<CustomSkill>,
        gamble: isize,
        geology: isize,
        hide_evade: isize,
        history: isize,
        languages: Vec<CustomSkill>,
        library_search: isize,
        mathematics: isize,
        physics: isize,
        programming: isize,
        shadow_track: isize,
        stock_market: isize,
        system_knowledge: isize,
        teaching: isize,
        wilderness_survival: isize,
        zoology: isize,
        custom_skills: Vec<CustomSkill>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ReflexSkills {
        archery: isize,
        athletics: isize,
        brawling: isize,
        dance: isize,
        dodge_and_escape: isize,
        driving: isize,
        fencing: isize,
        handgun: isize,
        heavy_weapons: isize,
        martial_arts: Vec<CustomSkill>,
        melee: isize,
        motorcycle: isize,
        operate_hvy_machinery: isize,
        pilot_gyro: isize,
        pilot_fixed_wing: isize,
        pilot_dirigible: isize,
        pilot_vect_thrust_vehicle: isize,
        rifle: isize,
        stealth: isize,
        submachinegun: isize,
        custom_skills: Vec<CustomSkill>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TechSkills {
        aero_tech: isize,
        av_tech: isize,
        basic_tech: isize,
        cryotank_operation: isize,
        cyberdeck_design: isize,
        cyber_tech: isize,
        demolitions: isize,
        disguise: isize,
        electronics: isize,
        elect_security: isize,
        first_aid: isize,
        forgery: isize,
        gyro_tech: isize,
        paint_or_draw: isize,
        photo_and_film: isize,
        pharmaceuticals: isize,
        pick_lock: isize,
        pick_pocket: isize,
        play_instrument: isize,
        weaponsmith: isize,
        custom_skills: Vec<CustomSkill>,
    }
    // i did NOT want to implement CEL or switch to a dictionary
    impl SpecialAbilities {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "authority" => Some(&mut self.authority),
                "charismatic_leadership" => Some(&mut self.charismatic_leadership),
                "combat_sense" => Some(&mut self.combat_sense),
                "credibility" => Some(&mut self.credibility),
                "family" => Some(&mut self.family),
                "interface" => Some(&mut self.interface),
                "jury_rig" => Some(&mut self.jury_rig),
                "medical_tech" => Some(&mut self.medical_tech),
                "resources" => Some(&mut self.resources),
                "street_deal" => Some(&mut self.street_deal),
                _ => self
                    .custom_skills
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value),
            }
        }
    }

    impl AttractionSkills {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "personal_grooming" => Some(&mut self.personal_grooming),
                "wardrobe_and_style" => Some(&mut self.wardrobe_and_style),
                _ => self
                    .custom_skills
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value),
            }
        }
    }

    impl BodySkills {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "endurance" => Some(&mut self.endurance),
                "strength_feat" => Some(&mut self.strength_feat),
                "swimming" => Some(&mut self.swimming),
                _ => self
                    .custom_skills
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value),
            }
        }
    }

    impl CoolWillSkills {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "interrogation" => Some(&mut self.interrogation),
                "intimidate" => Some(&mut self.intimidate),
                "oratory" => Some(&mut self.oratory),
                "resist_torture_drugs" => Some(&mut self.resist_torture_drugs),
                "streetwise" => Some(&mut self.streetwise),
                _ => self
                    .custom_skills
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value),
            }
        }
    }

    impl EmpathySkills {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "human_perception" => Some(&mut self.human_perception),
                "interview" => Some(&mut self.interview),
                "leadership" => Some(&mut self.leadership),
                "seduction" => Some(&mut self.seduction),
                "social" => Some(&mut self.social),
                "persuasion_and_fast_talk" => Some(&mut self.persuasion_and_fast_talk),
                "perform" => Some(&mut self.perform),
                _ => self
                    .custom_skills
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value),
            }
        }
    }

    impl IntellegenceSkills {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "accounting" => Some(&mut self.accounting),
                "anthropology" => Some(&mut self.anthropology),
                "awareness_notice" => Some(&mut self.awareness_notice),
                "biology" => Some(&mut self.biology),
                "botany" => Some(&mut self.botany),
                "chemistry" => Some(&mut self.chemistry),
                "composition" => Some(&mut self.composition),
                "diagnose_illness" => Some(&mut self.diagnose_illness),
                "education_and_gen_know" => Some(&mut self.education_and_gen_know),
                "expert" => Some(&mut self.expert.value),
                "gamble" => Some(&mut self.gamble),
                "geology" => Some(&mut self.geology),
                "hide_evade" => Some(&mut self.hide_evade),
                "history" => Some(&mut self.history),
                "library_search" => Some(&mut self.library_search),
                "mathematics" => Some(&mut self.mathematics),
                "physics" => Some(&mut self.physics),
                "programming" => Some(&mut self.programming),
                "shadow_track" => Some(&mut self.shadow_track),
                "stock_market" => Some(&mut self.stock_market),
                "system_knowledge" => Some(&mut self.system_knowledge),
                "teaching" => Some(&mut self.teaching),
                "wilderness_survival" => Some(&mut self.wilderness_survival),
                "zoology" => Some(&mut self.zoology),
                _ => self
                    .languages
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value)
                    .or_else(|| {
                        self.custom_skills
                            .iter_mut()
                            .find(|s| s.name == field)
                            .map(|s| &mut s.value)
                    }),
            }
        }
    }

    impl ReflexSkills {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "archery" => Some(&mut self.archery),
                "athletics" => Some(&mut self.athletics),
                "brawling" => Some(&mut self.brawling),
                "dance" => Some(&mut self.dance),
                "dodge_and_escape" => Some(&mut self.dodge_and_escape),
                "driving" => Some(&mut self.driving),
                "fencing" => Some(&mut self.fencing),
                "handgun" => Some(&mut self.handgun),
                "heavy_weapons" => Some(&mut self.heavy_weapons),
                "melee" => Some(&mut self.melee),
                "motorcycle" => Some(&mut self.motorcycle),
                "operate_hvy_machinery" => Some(&mut self.operate_hvy_machinery),
                "pilot_gyro" => Some(&mut self.pilot_gyro),
                "pilot_fixed_wing" => Some(&mut self.pilot_fixed_wing),
                "pilot_dirigible" => Some(&mut self.pilot_dirigible),
                "pilot_vect_thrust_vehicle" => Some(&mut self.pilot_vect_thrust_vehicle),
                "rifle" => Some(&mut self.rifle),
                "stealth" => Some(&mut self.stealth),
                "submachinegun" => Some(&mut self.submachinegun),
                _ => self
                    .martial_arts
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value)
                    .or_else(|| {
                        self.custom_skills
                            .iter_mut()
                            .find(|s| s.name == field)
                            .map(|s| &mut s.value)
                    }),
            }
        }
    }

    impl TechSkills {
        pub fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
            match field {
                "aero_tech" => Some(&mut self.aero_tech),
                "av_tech" => Some(&mut self.av_tech),
                "basic_tech" => Some(&mut self.basic_tech),
                "cryotank_operation" => Some(&mut self.cryotank_operation),
                "cyberdeck_design" => Some(&mut self.cyberdeck_design),
                "cyber_tech" => Some(&mut self.cyber_tech),
                "demolitions" => Some(&mut self.demolitions),
                "disguise" => Some(&mut self.disguise),
                "electronics" => Some(&mut self.electronics),
                "elect_security" => Some(&mut self.elect_security),
                "first_aid" => Some(&mut self.first_aid),
                "forgery" => Some(&mut self.forgery),
                "gyro_tech" => Some(&mut self.gyro_tech),
                "paint_or_draw" => Some(&mut self.paint_or_draw),
                "photo_and_film" => Some(&mut self.photo_and_film),
                "pharmaceuticals" => Some(&mut self.pharmaceuticals),
                "pick_lock" => Some(&mut self.pick_lock),
                "pick_pocket" => Some(&mut self.pick_pocket),
                "play_instrument" => Some(&mut self.play_instrument),
                "weaponsmith" => Some(&mut self.weaponsmith),
                _ => self
                    .custom_skills
                    .iter_mut()
                    .find(|s| s.name == field)
                    .map(|s| &mut s.value),
            }
        }
    }
}
