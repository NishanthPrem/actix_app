use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}