use std::{fs, io::Write};

use actix_web::{
    cookie::Cookie, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};

use serde::Deserialize;

use crate::logic::{is_valid_chars, read_file_to_string, token_exists_and_valid, AppStateType};

#[derive(Deserialize, Debug)]
struct RegisterData {
    username: String,
    password: String,
    confirm_password: String,
}

#[get("/register")]
async fn register_get(req: HttpRequest, data: AppStateType<'_>) -> impl Responder {
    let data = data.lock().unwrap();

    // if request has a cookie with a valid token, redirect to /
    if token_exists_and_valid(&req.cookie("token"), &data) {
        return HttpResponse::SeeOther().header("Location", "/").finish();
    }

    if let Ok(mut website) = read_file_to_string("public/html/register.html") {
        website = website.replace("[ERROR]", "");
        return HttpResponse::Ok().body(website);
    } else {
        return HttpResponse::InternalServerError().finish();
    }
}

#[post("/register")]
async fn register_post(
    req: HttpRequest,
    data: AppStateType<'_>,
    credentials: web::Form<RegisterData>,
) -> impl Responder {
    let mut data = data.lock().unwrap();

    // if request has a cookie with a valid token, redirect to /
    if token_exists_and_valid(&req.cookie("token"), &data) {
        return HttpResponse::SeeOther().header("Location", "/").finish();
    }

    if credentials.username.len() < 3
        || credentials.username.len() > 16
        || !is_valid_chars(credentials.username.clone())
        || data.get_user_db(&credentials.username).is_some()
    {
        if let Ok(mut website) = read_file_to_string("public/html/register.html") {
            website = website.replace(
                "[ERROR]",
                "<p style=\"color: red;\">The username is either already taken or invalid</p>",
            );
            return HttpResponse::Ok().body(website);
        } else {
            return HttpResponse::InternalServerError().finish();
        }
    } else if credentials.password.len() < 6 || credentials.password != credentials.confirm_password {
        if let Ok(mut website) = read_file_to_string("public/html/register.html") {
            website = website.replace(
                "[ERROR]", 
                "<p style=\"color: red;\">Password must be at least 6 characters and must match!</p>"
            );
            return HttpResponse::Ok().body(website);
        } else {
            return HttpResponse::InternalServerError().finish();
        }
    } else {
        let password_hash = data.hash(&credentials.password);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("content/public/user_db")
            .unwrap();

        // if storing credentials in file fails return internal server error
        if let Err(_) =
            file.write_all(format!("{}:{}\n", credentials.username, password_hash).as_bytes())
        {
            return HttpResponse::InternalServerError().finish();
        }

        data.add_user_db(&credentials.username, &password_hash);

        let token = data.unique_string();
        data.add_session(&token.clone(), &credentials.username.clone());

        // set cookie and redirect
        return HttpResponse::SeeOther()
            .header("Location", "/")
            .cookie(
                Cookie::build("token", token)
                    .secure(true)
                    .http_only(true)
                    .finish(),
            )
            .finish();
    }
}
