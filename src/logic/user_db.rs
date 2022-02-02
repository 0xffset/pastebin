use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone)]
pub struct UserDB {
    // <username, password>
    pub users: HashMap<String, String>,
}

impl UserDB {
    pub fn new() -> std::io::Result<Self> {
        let mut udb = UserDB {
            users: HashMap::new(),
        };

        if !Path::new("content/public/user_db").exists() {
            File::create("content/public/user_db")?;
        } else { // skip if file didnt exist before since no users will be present
            let users = File::open("content/public/user_db")?;

            let reader = BufReader::new(users);

            // extract credentials separated by ':' and store in hashmap
            for line in reader.lines() {
                let line = line?;
                let credentials = line.splitn(2, ":").collect::<Vec<&str>>();
                if credentials.len() == 2 {
                    udb.add_user(&credentials[0], &credentials[1]);
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        format!("Invalid credentials: {}", line),
                    ));
                }
            }
        }

        Ok(udb)
    }

    pub fn get_user<S: AsRef<str>>(&self, username: &S) -> Option<String> {
        match self.users.get(username.as_ref()) {
            Some(username) => Some(username.clone()),
            None => None,
        }
    }

    pub fn add_user<S: AsRef<str>, T: AsRef<str>>(&mut self, username: &S, password: &T) {
        self.users
            .insert(username.as_ref().to_string(), password.as_ref().to_string());
    }
}
