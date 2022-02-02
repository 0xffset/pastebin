use std::{collections::HashMap, fs::{self, File}, io::{Read, Write}, path::Path};

use uuid::Uuid;

#[derive(Clone)]
pub struct FileMaster {
    // <name, path>
    pub file_map: HashMap<String, String>,
}

impl FileMaster {
    pub fn new() -> std::io::Result<Self> {
        let mut fm = FileMaster {
            file_map: HashMap::new(),
        };

        // make sure content directory tree is created else create it
        if !Path::new("content").exists() {
            fs::create_dir("content")?;
        }
        if !Path::new("content/public").exists() {
            fs::create_dir("content/public")?;
        }
        if !Path::new("content/users").exists() {
            fs::create_dir("content/users")?;
        }

        // read all public pastes names
        let public = std::fs::read_dir("content/public")?;
        for entry in public {
            let entry = entry?;
            // get filename and save it into map with "public" as value
            if let Some(file_stem) = entry.path().file_stem() {
                fm.file_map.insert(
                    file_stem.to_string_lossy().to_string(),
                    "public".to_string(),
                );
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Error reading public paste file: {:#?}", entry),
                ));
            }
        }

        // read all users pastes names
        let users = std::fs::read_dir("content/users")?;
        for entry in users {
            let entry = entry?;
            // check if entry is really directory else skip
            if entry.path().is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                let user = std::fs::read_dir(format!("content/users/{}", name))?;

                // get filenames and save it into map with user name as value
                for entry in user {
                    let entry = entry?;
                    if let Some(file_stem) = entry.path().file_stem() {
                        fm.file_map
                            .insert(file_stem.to_string_lossy().to_string(), name.clone());
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Unsupported,
                            format!("Error reading {} paste file: {:#?}", name, entry),
                        ));
                    }
                }
            }
        }

        Ok(fm)
    }

    pub fn store_file(&mut self, content: &String, path: &str) -> std::io::Result<String> {
        let filename = Uuid::new_v4().to_string().replace("-", "");
        let mut file;
        if path == "public" {
            file = File::create(format!("content/public/{}", filename))?;
        } else {
            // create user folder if not exists
            let user_path = format!("content/users/{}", path);
            if !Path::new(&user_path).exists() {
                fs::create_dir(user_path)?;
            }
            
            file = File::create(format!("content/users/{}/{}", path, filename))?;
        }
        file.write_all(content.as_bytes())?;

        self.file_map.insert(filename.clone(), path.to_string());

        Ok(filename)
    }

    pub fn read_file(&self, name: &String) -> std::io::Result<String> {
        if let Some(path) = self.file_map.get(name) {
            let mut file;
            if *path == "public".to_string() {
                file = File::open(format!("content/public/{}", name))?;
            } else {
                file = File::open(format!("content/users/{}/{}", path, name))?;
            }

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            return Ok(contents);
        }

        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }
}
