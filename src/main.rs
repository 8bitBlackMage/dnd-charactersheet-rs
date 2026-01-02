use iced::{Element, Length};
use iced::widget::{column, container, row};

mod charactersheet;
mod messages;

mod gui;



use crate::charactersheet::character::Character;
use crate::charactersheet::abilities::{AbilityScoreTypes, SkillTypes};

use crate::charactersheet::level;
use crate::messages::Message;
#[derive(Default)]
struct Application {
    character: Character
}

impl Application {

    fn new() -> Self { 
        Self{ character: Character::default()} 
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
            Message::SkillProficencyChanged(skill_id) => 
            {
                let mut skill = self.character.get_skill(skill_id);
                skill.proficient = !skill.proficient;
                self.character.set_skill(skill_id, skill);
            },
            Message::SkillExpertieseChanged(skill_id) => {
                let mut skill = self.character.get_skill(skill_id);
                skill.expert = !skill.expert;
                self.character.set_skill(skill_id, skill);
            },
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
        use SkillTypes::*;
        column![
        gui::topbar::view(),
       row![
        container(gui::namepanel::view(&self.character)).width(Length::FillPortion(1)),
        container(gui::levelpanel::view(&self.character.level)).width(Length::FillPortion(1))
       ].height(300),
       row![
        container(gui::stats::statpanel::view(&self.character, AbilityScoreTypes::Strength, Vec::from([Athletics])))
            .width(Length::FillPortion(1)),

        container(gui::stats::statpanel::view(&self.character,AbilityScoreTypes::Dexterity,Vec::from([Acrobatics, SleightOfHand, Stealth])))
            .width(Length::FillPortion(1)),

        container(gui::stats::statpanel::view(&self.character,AbilityScoreTypes::Constitution,Vec::from([])))
            .width(Length::FillPortion(1)),
        ],
       row![
        container(gui::stats::statpanel::view(&self.character, AbilityScoreTypes::Intellegence,Vec::from([Arcana,History,Investigation,Nature,Religion])))
            .width(Length::FillPortion(1)),
        container(gui::stats::statpanel::view(&self.character, AbilityScoreTypes::Wisdom, Vec::from([AnimalHandling,Insight,Medicine,Perception,Survival])))
            .width(Length::FillPortion(1)),
        container(gui::stats::statpanel::view(&self.character, AbilityScoreTypes::Charisma, Vec::from([Deception,Intimidation,Performance,Persuasion])))
            .width(Length::FillPortion(1)),
       ]
        ].into()

    }
}



fn main() -> iced::Result {
 
    iced::application(Application::new, Application::update, Application::view).run()
    
}
