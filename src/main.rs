use iced::{
        Alignment::Center, Element, Length, alignment::Horizontal::Left, widget::{button, column, container, row, text, text_input}};

mod statblocks;
mod character;
mod level;
mod messages;

mod gui;



use crate::character::Character;
use crate::messages::Message;
#[derive(Default)]
struct Application {
    character: Character
}

impl Application {

    fn new() -> Self { Self{
        character: Character{   name: "Tav".to_string(), 
                                class:"Warrior".to_string(), 
                                subclass:"".to_string(), 
                                species: "Half Elf".to_string()
                                
                            }
    } }
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
       row![
       container( gui::namepanel::view(&self.character)).width(300)
       ].height(300).into()

    }
}



fn main() -> iced::Result {
 
    iced::application(Application::new, Application::update, Application::view).run()
    
}
