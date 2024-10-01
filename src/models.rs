#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Joke {
    pub setup: String,
    pub punchline: String,
}