use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::{fs::File, io::{self, BufReader, BufWriter, Write}, path};
use crate::charactersheet::abilities::*;
use crate::{charactersheet::{abilities::{Skill, Ability}}, level};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Character {
        pub name: String,
        pub class: String,
        pub subclass: String, 
        pub species: String,
        
        pub level: level::Level,
   

        pub strength : Ability,
        pub dexterity: Ability,
        pub constition: Ability,
        pub intellegence : Ability,
        pub wisdom: Ability,
        pub charisma: Ability,
        
        abilities: HashMap<AbilityScoreTypes,Ability>,

        skills: HashMap<SkillTypes, Skill>
        


    }

    impl Default for Character {
        fn default() -> Self {
        let level = level::Level::new(1,false);
        let mut  character = Character { 
            name: "Tav".to_string(), 
            class:"Fighter".to_string(), 
            subclass:"".to_string(), 
            species: "Half Elf".to_string(),
            level: level,
            strength: Default::default(), 
            dexterity: Default::default(), 
            constition: Default::default(), 
            intellegence: Default::default(), 
            wisdom: Default::default(), 
            charisma: Default::default(),
            abilities: HashMap::from([
                (AbilityScoreTypes::Strength,Ability::new(15)),
                (AbilityScoreTypes::Dexterity,Ability::new(13)),
                (AbilityScoreTypes::Constitution,Ability::new(14)),
                (AbilityScoreTypes::Intellegence,Ability::new(10)),
                (AbilityScoreTypes::Wisdom,Ability::new(12)),
                (AbilityScoreTypes::Charisma,Ability::new(8))
            ]),
            skills: HashMap::from([
                    (SkillTypes::Athletics,Skill::new(0, AbilityScoreTypes::Strength, &level)),
                    (SkillTypes::Acrobatics, Skill::new(0,AbilityScoreTypes::Dexterity, &level)),
                    (SkillTypes::SleightOfHand, Skill::new(0,AbilityScoreTypes::Dexterity, &level)),
                    (SkillTypes::Stealth, Skill::new(0,AbilityScoreTypes::Dexterity, &level)),
                    (SkillTypes::Arcana, Skill::new(0, AbilityScoreTypes::Intellegence, &level)),
                    (SkillTypes::History, Skill::new(0, AbilityScoreTypes::Intellegence, &level)),
                    (SkillTypes::Investigation, Skill::new(0, AbilityScoreTypes::Intellegence, &level)),
                    (SkillTypes::Nature, Skill::new(0, AbilityScoreTypes::Intellegence, &level)),
                    (SkillTypes::Religion, Skill::new(0, AbilityScoreTypes::Intellegence, &level)),
                    (SkillTypes::AnimalHandling, Skill::new(0, AbilityScoreTypes::Wisdom, &level)),
                    (SkillTypes::Insight, Skill::new(0, AbilityScoreTypes::Wisdom, &level)),
                    (SkillTypes::Medicine, Skill::new(0, AbilityScoreTypes::Wisdom, &level)),
                    (SkillTypes::Perception, Skill::new(0, AbilityScoreTypes::Wisdom, &level)),
                    (SkillTypes::Survival, Skill::new(0, AbilityScoreTypes::Wisdom, &level)),
                    (SkillTypes::Deception, Skill::new(0, AbilityScoreTypes::Charisma, &level)),
                    (SkillTypes::Intimidation, Skill::new(0, AbilityScoreTypes::Charisma, &level)),
                    (SkillTypes::Performance, Skill::new(0, AbilityScoreTypes::Charisma, &level)),
                    (SkillTypes::Persuasion, Skill::new(0, AbilityScoreTypes::Charisma, &level)),
            ]) 
        };
        character.propagate_skill_changes();
        return character;
    }
    }


    impl Character {

        pub fn save(& self, file_path: &path::Path) -> Result<(),io::Error> {
            let file = File::create(file_path.as_os_str())?;

            let mut writer = BufWriter::new(file);

            let _ = writer.write_all( serde_json::to_string_pretty(&self)?.as_bytes());

            Ok(())
        }

        pub fn propagate_skill_changes(&mut self){
            for (_,skill) in self.skills.iter_mut() {
                skill.set_modifier(self.abilities[&skill.ability].value, &self.level);
            }
        }

        pub fn load( file_path: &path::Path) -> Self {
            match  File::open(file_path){
                Ok(file) => {
                    let reader = BufReader::new(file);
                     match serde_json::from_reader(reader) {
                        Ok(char) => {return char},
                        Err(_) => {return Self::default()},
                    }
                }
                Err(_) => {return Self::default() },
            }
        }

        pub fn get_skill(&self ,skill_id: SkillTypes) -> Skill {
            self.skills[&skill_id]
        }
        pub fn set_skill(&mut self, skill_id: SkillTypes, skill: Skill){
            self.skills.insert(skill_id,skill);
        }

        //#TODO come back and handle skill propogation on change.
        pub fn get_ability(&self, ability_id: AbilityScoreTypes) -> Ability {
            self.abilities[&ability_id]
        }
        
        pub fn set_ability(&mut self, ability_id: AbilityScoreTypes, ability: Ability) {
            self.abilities.insert(ability_id, ability);
            self.propagate_skill_changes();
        }

    }