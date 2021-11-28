use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    data: String,
}

async fn json(data: Json<Payload>) -> Result<Json<Payload>> {
    let data = data.into_inner();
    debug!("json data: {:?}", data);
    Ok(Json(data))
}

async fn form(data: Form<Payload>) -> Result<Json<Payload>> {
    Ok(Json(data.into_inner()))
}

async fn path(data: Path<Payload>) -> Result<Json<Payload>> {
    Ok(Json(data.into_inner()))
}

async fn query(data: Query<Payload>) -> Result<Json<Payload>> {
    Ok(Json(data.into_inner()))
}

async fn error() -> Result<Json<()>> {
    Err(anyhow!("error")).with_code(StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn service() -> actix_web::Scope {
    web::scope("/example")
        .route("/form", web::route().to(form))
        .route("/json", web::route().to(json))
        .route("/path/{data}", web::route().to(path))
        .route("/query", web::route().to(query))
        .route("/error", web::route().to(error))
}
