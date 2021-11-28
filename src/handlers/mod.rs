use actix_web::{Responder, Result};

pub async fn not_found() -> impl Responder {
    "not found"
}
