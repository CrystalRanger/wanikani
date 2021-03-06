
extern crate reqwest;

use reqwest::{Client, StatusCode};
use serde_json::Value;

const APIENDPOINT: &str = "https://api.wanikani.com/v2/";

#[tokio::main]
async fn main() {
    //get the json data
    let reviews = get_json("assignments?immediately_available_for_review=true").await.unwrap();
    let lessons = get_json("assignments?immediately_available_for_lessons=true").await.unwrap();
    let user: Value = serde_json::from_str(get_json("user").await.unwrap().as_str()).unwrap();

    let mut username = user["data"]["username"].to_string();

    //remove quotes in a very stupid manner
    username.pop();
    username.remove(0);

    println!("Hello, {}!", username);



    let r_deserialized: Value = serde_json::from_str(reviews.as_str()).unwrap();
    let l_deserialized: Value = serde_json::from_str(lessons.as_str()).unwrap();

    println!("You have {} reviews, and {} lessons", r_deserialized["total_count"], l_deserialized["total_count"]);

}

async fn get_json(endpoint: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    match client.get(APIENDPOINT.to_owned()+endpoint)
       .header("Authorization", format!("Bearer {}", dotenv::var("API_TOKEN").unwrap()).as_str())
       .send()
       .await {
           Ok(resp) => {
               if resp.status() == StatusCode::OK {
                   Ok(resp.text().await.expect("No text!"))
               } else {
                   Err(resp.error_for_status().unwrap_err())
               }
           }
           Err(e) => {
               Err(e)
           }
       }
}