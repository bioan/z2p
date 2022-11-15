use actix_web::{post, web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscriptionData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<SubscriptionData>) -> impl Responder {
    HttpResponse::Ok()
}
