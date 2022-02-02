use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::logic::{
    read_file_to_string, return_page_or_error, token_exists_and_valid, AppStateType,
};

#[get("/")]
async fn index(req: HttpRequest, data: AppStateType<'_>) -> impl Responder {
    let data = data.lock().unwrap();

    // if request has a cookie with a valid token, hide login/register and show name and logout
    if token_exists_and_valid(&req.cookie("token"), &data) {
        // user and cookie exist guaranteed because of previous check
        let username = data
            .get_username_from_session(&req.cookie("token").unwrap().value().to_string())
            .unwrap();

        if let Ok(mut website) = read_file_to_string("public/html/index_user.html") {
            website = website.replace("[USERNAME]", &username);
            return HttpResponse::Ok().body(website);
        } else {
            return HttpResponse::InternalServerError().finish();
        }
    }

    return_page_or_error("public/html/index.html")
}
