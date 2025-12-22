    #[derive(Default, Debug)]
    pub struct Character {
        pub name: String,
        pub class: String,
        pub subclass: String, 
        pub species: String,
    }

    impl Character {

        pub fn save(&mut self) {
            println!("{:?}", self);
        }
    }
