use actix_web::{
    error, 
    http::{header::ContentType, StatusCode},
    HttpResponse
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display("Internal Server Error")]
    InternalError,
    #[display("Bad Request:")]
    Unauthorized,
    #[display("Forbidden:")]
    Unauthenticated,
    #[display("Not Found:")]
    Timeout,
    #[display("Conflict:")]
    InvalidCredentials,
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())}

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::Unauthorized => StatusCode::UNAUTHORIZED,
            HttpError::Unauthenticated => StatusCode::FORBIDDEN,
            HttpError::Timeout => StatusCode::REQUEST_TIMEOUT,
            HttpError::InvalidCredentials => StatusCode::CONFLICT,
            
        }
    }
}
