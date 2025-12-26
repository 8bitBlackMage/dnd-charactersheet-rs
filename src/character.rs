use serde::{Deserialize, Serialize};
use std::{fs::File, io::{self, BufReader, BufWriter, Write}, path};
use crate::{level, statblocks::StatBlock};

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct Character {
        pub name: String,
        pub class: String,
        pub subclass: String, 
        pub species: String,
        
        pub level: level::Level,
   

        pub strength : StatBlock,
        pub intellegence : StatBlock
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
    }
