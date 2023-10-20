use std::io;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn format_useraname(&self) -> () {
        println!("Viewing @{}...", self.username);
    }

    pub fn validate_password(&self) -> () {
        if self.password.len() < 12 {
            println!("Password length: {}", self.password.len());
            println!("Weak password!");
        } else {
            println!("Password length: {}", self.password.len());
        }
    }

    pub fn format_password(&self) -> () {
        if self.password.len() > 0 {
            let password_length = self.password.len();
            let mut counter = 0;

            loop {
                print!("*");
                counter = counter + 1;
                if counter == password_length {
                    break;
                }
            }
        }
    }
}

fn main() -> () {
    println!("Please provide your credentials...");
    println!("Enter a valid username e.g johndoe, peterpan...>");
    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Error::Failed to read line...");

    println!("Enter a valid password, more than 12 characters >");
    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Error::Failed to read line...");

    let credentials: Credentials = Credentials {
        username: username.clone(),
        password: password.clone(),
    };

    credentials.format_useraname();
    credentials.validate_password();
    credentials.format_password();
}
