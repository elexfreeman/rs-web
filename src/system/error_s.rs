use actix_web::{http, Error, HttpResponse};

pub fn response_error(e: Error) -> HttpResponse {
    log::error!("Error {:?}", e);
    HttpResponse::Ok()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(e.to_string())
}

