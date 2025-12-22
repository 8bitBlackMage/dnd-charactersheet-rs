use iced::{
        Alignment::Center, alignment::Horizontal::Left,
             widget::{Column, button, column, row, text, text_input}};


mod character;
use crate::character::Character;

#[derive(Clone)]
enum Message{
    NameChanged(String),
    ClassChanged(String),
    SubclassChanged(String),
    SpeciesChanged(String),
    SaveToFile,
}

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
    fn update(&mut self, message: Message)
    {
        match message
        {
            Message::NameChanged(name) => {self.character.name = name;}
            Message::ClassChanged(class) => {self.character.class = class; }
            Message::SubclassChanged(sub_class) => {self.character.subclass = sub_class;}
            Message::SpeciesChanged(species) => {self.character.species = species;}
            Message::SaveToFile => {self.character.save();}
        }
    }
    fn view(&'_  self) -> Column<'_, Message>{
        column![
            row![
                text("name: ").size(30),

                text_input("",&self.character.name).on_input(Message::NameChanged)
                ].align_y(Center), 

            row![
                text("Class: ").size(30), 
                text_input("",&self.character.class).on_input(Message::ClassChanged)
                ].align_y(Center), 

            row![
                text("SubClass: ").size(30), 
                text_input("",&self.character.subclass).on_input(Message::SubclassChanged)
                ].align_y(Center), 
            row![
                text("Species: ").size(30), 
                text_input("",&self.character.species).on_input(Message::SpeciesChanged)
                ].align_y(Center), 

            button("save").on_press(Message::SaveToFile)
        ].align_x(Left)
    }
}



fn main() -> iced::Result {
 
    iced::application(Application::new, Application::update, Application::view).run()
    
}
