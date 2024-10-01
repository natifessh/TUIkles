use crate::models::Joke;
use reqwest::Result;

pub async fn get_joke() -> Result<Joke> {
    let request = reqwest::get("http://www.official-joke-api.appspot.com/jokes/random").await?;
    let joke = request.json::<Joke>().await?;
    Ok(joke)
}