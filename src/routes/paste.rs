use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};

use serde::Deserialize;

use crate::logic::{token_exists_and_valid, AppStateType};

#[get("/paste/{paste}")]
async fn paste(data: AppStateType<'_>, web::Path(paste): web::Path<String>) -> impl Responder {
    let data = data.lock().unwrap();
    match data.read_paste(&paste) {
        Ok(paste) => HttpResponse::Ok()
            .set_header("Content-Type", "text/plain")
            .set_header("X-Content-Type-Options", "nosniff")
            .body(paste),
        Err(_) => HttpResponse::NotFound().body("<p style=\"color: red;\">Paste not found</p>"),
    }
}

#[derive(Deserialize)]
struct PasteData {
    pub paste: String,
}

#[post("/paste")]
async fn paste_upload(
    req: HttpRequest,
    data: AppStateType<'_>,
    paste_data: web::Form<PasteData>,
) -> impl Responder {
    let mut data = data.lock().unwrap();

    let path;
    // if request has a cookie with a valid token save in users directory
    if token_exists_and_valid(&req.cookie("token"), &data) {
        // it can be unwrapped savely because of the check above
        path = data
            .get_username_from_session(&req.cookie("token").unwrap().value().to_string())
            .unwrap();
    } else {
        path = "public".to_string();
    }

    match data.store_paste(&paste_data.paste, &path) {
        Ok(link) => HttpResponse::Ok()
            .set_header("Content-Type", "text/plain")
            .set_header("X-Content-Type-Options", "nosniff")
            .body(format!("{}/paste/{}", std::env::var("URL").unwrap(), link)),
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("<p style=\"color: red;\">Internal Server Error</p>")
        }
    }
}
