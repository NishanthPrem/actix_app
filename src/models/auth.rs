use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PathParameters {
    pub name: String,
    pub id: String,
    pub email: String,
}
