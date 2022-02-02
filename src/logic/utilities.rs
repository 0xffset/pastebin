use std::{fs::File, io::Read};

use actix_web::{cookie::Cookie, HttpResponse};

use super::AppState;

pub fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn return_page_or_error(path: &str) -> HttpResponse {
    if let Ok(page) = read_file_to_string(path) {
        HttpResponse::Ok().content_type("text/html").body(page)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub fn token_exists_and_valid(token: &Option<Cookie>, data: &AppState) -> bool {
    match token {
        Some(token) => {
            if data.session_exists(&token.value().to_string()) {
                return true;
            } else {
                return false;
            }
        }
        None => return false,
    }
}

lazy_static! {
    pub static ref ILLEGAL_CHARS: Vec<char> = {
        let mut vec = Vec::new();
        vec.push('\n');
        vec.push('\r');
        vec.push('\t');
        vec.push(':');
        vec.push('\0');

        vec
    };
}

pub fn is_valid_chars(input: String) -> bool {
    for c in &*ILLEGAL_CHARS {
        if input.contains(*c) {
            return false;
        }
    }

    return true;
}
