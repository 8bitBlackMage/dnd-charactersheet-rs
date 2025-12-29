use iced::{Element, Length};
use iced::widget::{column, container, row};

mod charactersheet;
mod messages;

mod gui;



use crate::charactersheet::character::Character;
use crate::charactersheet::level;
use crate::messages::Message;
use crate::charactersheet::statblocks::StatBlock;
#[derive(Default)]
struct Application {
    character: Character
}

impl Application {

    fn new() -> Self { 
        Self{ character: Character{   name: "Tav".to_string(), 
                                class:"Fighter".to_string(), 
                                subclass:"".to_string(), 
                                species: "Half Elf".to_string(),
                                level: level::Level::new(1,false),
                                strength : StatBlock::new_strength_block(15),
                                dexterity: StatBlock::new_dexterity_block(13),
                                constition: StatBlock::new_constitution_block(14),
                                intellegence : StatBlock::new_intellegence_block(10),
                                wisdom: StatBlock::new_wisdom_block(12),
                                charisma: StatBlock::new_charisma_block(8),
    } 
    }
}
    fn update(&mut self, message: messages::Message)
    {
        match message
        {
            Message::NameChanged(name) => {self.character.name = name;}
            Message::ClassChanged(class) => {self.character.class = class; }
            Message::SubclassChanged(sub_class) => {self.character.subclass = sub_class;}
            Message::SpeciesChanged(species) => {self.character.species = species;}
            Message::SaveToFile => {self.open_save_dialog();}
            Message::LoadFromFile => {self.open_load_dialog();}
            Message::LevelChanged(level) => self.character.level.level_from_str(level),
            Message::ExperienceAdd(exp) => {self.character.level.add_experience(exp);} ,
            Message::ExperienceRemoved(_) => todo!(),
            Message::SkillProficencyChanged(stat_type) => {
                let mut skill = self.character.get_skill(stat_type);
            }
            Message::SkillExpertieseChanged(_) => {},
        }
    }

    fn open_save_dialog(& self) {

        match rfd::FileDialog::new()
        .set_file_name("character.json")
        .set_directory("~/Documents").save_file(){
            Some(path) => 
            {
                match  self.character.save(&path){
                    Ok(_) => {}
                    Err(err) => {print!("{}",err);}
                }
            }
            None => 
            {

            }
        }
    }

    fn open_load_dialog(&mut self)
    {
        match rfd::FileDialog::new()
        .set_file_name("character.json")
        .set_directory("~/Documents").pick_file(){
            Some(path) => 
            {
                self.character = Character::load(&path);
            }
            None => {}
    }
    }

    fn view(&'_  self) -> Element<'_, messages::Message>{
        column![
        gui::topbar::view(),
       row![
        container(gui::namepanel::view(&self.character)).width(Length::FillPortion(1)),
        container(gui::levelpanel::view(&self.character.level)).width(Length::FillPortion(1))
       ].height(300),
       row![
        container(gui::stats::statpanel::view("Strength".to_string(), &self.character.strength)).width(Length::FillPortion(1)),
        container(gui::stats::statpanel::view("Dexterity".to_string(), &self.character.dexterity)).width(Length::FillPortion(1)),
        container(gui::stats::statpanel::view("Constitution".to_string(), &self.character.constition)).width(Length::FillPortion(1)),
       ],
       row![
        container(gui::stats::statpanel::view("Intellegence".to_string(), &self.character.intellegence)).width(Length::FillPortion(1)),
        container(gui::stats::statpanel::view("Wisdom".to_string(), &self.character.wisdom)).width(Length::FillPortion(1)),
        container(gui::stats::statpanel::view("Charisma".to_string() ,&self.character.charisma)).width(Length::FillPortion(1)),
       ]
        ].into()

    }
}



fn main() -> iced::Result {
 
    iced::application(Application::new, Application::update, Application::view).run()
    
}
