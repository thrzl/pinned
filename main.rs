use actix_web::{get, App, web, HttpServer, Responder, HttpResponse, http, middleware::Logger};
use reqwest::get;
use std::collections::HashMap;
use miniserde::{Serialize, Deserialize, json};
use cached::proc_macro::cached;
use env_logger;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
  name: String,
  uuid: String,
}

#[cached(size=1000)]
async fn request(url: String) -> HashMap<String, String> {
    let resp = get(&url).await.unwrap();
    let resp_data = json::from_str(&resp.text().await.unwrap()).unwrap();
    resp_data
}

#[get("/")]
async fn index() -> Result<HttpResponse, http::Error> {
    Ok(HttpResponse::PermanentRedirect()
    .append_header(("Location", "https://crust.terabyteis.me")).finish())
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(r#"<iframe width="100%" height="100%" src="https://www.youtube-nocookie.com/embed/Yw6u6YkTgQ4?controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Starting on port {}", 8080);
    HttpServer::new(|| {App::new()
            .wrap(Logger::default())
            // .service(index)
            .service(hello)
            .service(user)
        }
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await        
}
//Paragonii's part here
#[get("/{user}")]
async fn user_repos() -> impo Responder {
    HttpResponse::build(http::StatusCode::OK)
    SomeScraperCode(".pinned-item-list-item.public")
}