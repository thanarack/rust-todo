use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoRequest {
    pub todo: String,
}
