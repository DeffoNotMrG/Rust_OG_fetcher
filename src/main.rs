#[macro_use] extern crate rocket;

use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use rocket::tokio;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

// Add these at the top with your other imports
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get API key from header
        let api_key = request.headers().get_one("x-api-key");
        
        // Get expected API key from environment
        let expected_api_key = env::var("API_KEY").expect("API_KEY must be set");
        
        match api_key {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if key == expected_api_key => Outcome::Success(ApiKey(key.to_string())),
            Some(_) => Outcome::Error((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}

#[derive(Serialize)]
struct MetaTags {
    title: Option<String>,
    description: Option<String>,
    og_image: Option<String>,
}

#[get("/fetch_meta?<url>")]
async fn fetch_meta(_api_key: ApiKey, url: String) -> String {
    // Create a reqwest client
    let client = Client::new();

    // Fetch the HTML content of the URL
    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(_) => return serde_json::to_string(&MetaTags {
            title: None,
            description: None,
            og_image: None,
        }).unwrap(),
    };

    let body = response.text().await.unwrap_or_default();
    let document = Html::parse_document(&body);

    // Selectors for meta tags
    let title_selector = Selector::parse("title").unwrap();
    let meta_selector = Selector::parse("meta").unwrap();
    let og_image_selector = Selector::parse("meta[property='og:image']").unwrap();

    // Extract the title
    let title = document.select(&title_selector).next().map(|title| title.inner_html());

    // Extract the description
    let mut description = None;
    for element in document.select(&meta_selector) {
        if let Some(name) = element.value().attr("name") {
            if name == "description" {
                description = element.value().attr("content").map(|s| s.to_string());
                break;
            }
        }
    }

    // Extract the OG image
    let og_image = document.select(&og_image_selector).next()
        .and_then(|element| element.value().attr("content").map(|s| s.to_string()));

    // Return the result as JSON
    let meta_tags = MetaTags {
        title,
        description,
        og_image,
    };

    serde_json::to_string(&meta_tags).unwrap()
}

#[catch(401)]
fn unauthorized() -> &'static str {
    "Invalid or missing API key"
}

#[catch(400)]
fn bad_request() -> &'static str {
    "API key is required"
}

#[launch]
fn rocket() -> _ {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Ensure API_KEY is set
    if env::var("API_KEY").is_err() {
        panic!("API_KEY must be set in environment variables");
    }

    rocket::build()
        .mount("/", routes![fetch_meta])
        .register("/", catchers![unauthorized, bad_request])
}