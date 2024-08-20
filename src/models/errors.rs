use actix_web::ResponseError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Error Response: {}", message)]
pub struct HttpError {
    pub message: String,
}

impl ResponseError for HttpError {}
