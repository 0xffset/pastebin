use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::logic::AppStateType;

#[get("/logout")]
async fn logout(req: HttpRequest, data: AppStateType<'_>) -> impl Responder {
    let mut res = HttpResponse::SeeOther();

	// if cookie exists remove from client
    if let Some(cookie) = req.cookie("token") {
        res.del_cookie(&cookie);

		// if cookie was a valid session token, remove from session store
        let token = cookie.value().to_string();
        if data.lock().unwrap().session_exists(&token) {
            data.lock().unwrap().remove_session(&token);
        }
    }

	// redirect to homepage
    return res.header("Location", "/").finish();
}
