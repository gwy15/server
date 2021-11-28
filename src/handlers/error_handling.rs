use actix_web::{
    error::{JsonPayloadError, PathError, QueryPayloadError, UrlencodedError},
    HttpRequest,
};

use crate::prelude::*;

pub async fn not_found(req: HttpRequest) -> Result<&'static str> {
    let path = req.path();
    Err(web_error!(
        StatusCode::NOT_FOUND,
        "The route \"{}\"is not found.",
        path
    ))
}

pub fn form_error(e: UrlencodedError, _req: &HttpRequest) -> ActixError {
    web_error!(StatusCode::BAD_REQUEST, e).into()
}

pub fn json_error(e: JsonPayloadError, _req: &HttpRequest) -> ActixError {
    web_error!(StatusCode::BAD_REQUEST, e).into()
}

pub fn path_error(e: PathError, _req: &HttpRequest) -> ActixError {
    web_error!(StatusCode::BAD_REQUEST, e).into()
}

pub fn query_error(e: QueryPayloadError, _req: &HttpRequest) -> ActixError {
    web_error!(StatusCode::BAD_REQUEST, e).into()
}
