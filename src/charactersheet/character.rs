use serde::{Deserialize, Serialize};
use std::{fs::File, io::{self, BufReader, BufWriter, Write}, path};
use crate::{charactersheet::{statblocks::{Skill, StatBlock}, stattypes}, level};

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct Character {
        pub name: String,
        pub class: String,
        pub subclass: String, 
        pub species: String,
        
        pub level: level::Level,
   

        pub strength : StatBlock,
        pub dexterity: StatBlock,
        pub constition: StatBlock,
        pub intellegence : StatBlock,
        pub wisdom: StatBlock,
        pub charisma: StatBlock,
    
    }

    impl Character {

        pub fn save(& self, file_path: &path::Path) -> Result<(),io::Error> {
            let file = File::create(file_path.as_os_str())?;

            let mut writer = BufWriter::new(file);

            let _ = writer.write_all( serde_json::to_string_pretty(&self)?.as_bytes());

            Ok(())
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

        pub fn get_statblock(&self, stat_type:stattypes::StatTypes) -> & StatBlock
        {
             match stat_type{
                stattypes::StatTypes::Strength(_) => &self.strength,
                stattypes::StatTypes::Dexterity(_) => &self.dexterity,
                stattypes::StatTypes::Constitution(_) => &self.constition,
                stattypes::StatTypes::Intellegence(_) => &self.intellegence,
                stattypes::StatTypes::Wisdom(_) => &self.wisdom,
                stattypes::StatTypes::Charisma(_) =>  &self.charisma,
            }
        }

        pub fn get_skill(&self,stat_type:stattypes::StatTypes) -> & Skill {
            let stat_block = self.get_statblock(stat_type);
            let skill_id:usize = stat_type.into();
            & &stat_block.skills[skill_id].1
        }

    }