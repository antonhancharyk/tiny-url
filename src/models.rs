use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUrl {
    pub url: String,
    pub tiny_url: String,
}
