use actix_web::{post, HttpResponse, Responder};

#[post("/login")]
pub async fn signin() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users signin")
}

#[post("")]
pub async fn signup() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users signup")
}
