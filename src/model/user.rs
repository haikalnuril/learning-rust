use crate::third::say_hello as say_hello_third;

pub struct User {
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub email: String,
        pub age: u8,
}

impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {}, my name is {}", name, self.first_name);

            say_hello_third();
        }

        pub fn say_bye(){
            println!("Bye-bye")
        }
}

pub mod second {
    pub fn say_bye(){
        super::User::say_bye();
    }
}

