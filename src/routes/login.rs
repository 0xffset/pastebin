use actix_web::{
    cookie::Cookie, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};

use crate::logic::{
    read_file_to_string, token_exists_and_valid, AppStateType,
};

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginToken {
    token: String,
}

#[get("/login")]
async fn login_get(req: HttpRequest, data: AppStateType<'_>) -> impl Responder {
    let data = data.lock().unwrap();

    // if request has a cookie with a valid token, redirect to /
    if token_exists_and_valid(&req.cookie("token"), &data) {
        return HttpResponse::SeeOther().header("Location", "/").finish();
    }

    if let Ok(mut website) = read_file_to_string("public/html/login.html") {
        website = website.replace("[ERROR]", "");
        return HttpResponse::Ok().body(website);
    } else {
        return HttpResponse::InternalServerError().finish();
    }
}

#[post("/login")]
async fn login_post(
    req: HttpRequest,
    data: AppStateType<'_>,
    credentials: web::Form<LoginData>,
) -> impl Responder {
    let mut data = data.lock().unwrap();

    // if request has a cookie with a valid token, redirect to /
    if token_exists_and_valid(&req.cookie("token"), &data) {
        return HttpResponse::SeeOther().header("Location", "/").finish();
    }

    // if user exists
    if let Some(pass) = data.get_user_db(&credentials.username) {
        // if password matches
        if data.validate_hash(&pass, &credentials.password) {
            // generate token
            let token = data.unique_string();
            data.add_session(&token, &credentials.username);

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

    // user didn't exist
    if let Ok(mut website) = read_file_to_string("public/html/login.html") {
        website = website.replace(
            "[ERROR]",
            "<p style=\"color: red\">Incorrect username or password!</p>",
        );
        return HttpResponse::Ok().body(website);
    } else {
        return HttpResponse::InternalServerError().finish();
    }
}
