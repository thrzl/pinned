use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use cached::proc_macro::cached;
use env_logger;
use microserde::json;
use reqwest::get;
use select::document::Document;
use select::predicate::{Attr, Class};
use std::collections::HashMap;

#[cached(size = 1000)]
async fn get_pinned(u: String) -> Vec<HashMap<String, String>> {
    let resp = get(format!("https://github.com/{u}")).await.unwrap();
    let document = Document::from(&resp.text().await.unwrap().to_owned()[..]);
    let mut repos: Vec<_> = Vec::new();
    document
        .find(Class("pinned-item-list-item"))
        .for_each(|node| {
            let mut repo = HashMap::new();
            node.find(Class("repo")).for_each(|node| {
                repo.insert("name".to_string(), node.text().to_string());
                repo.insert("full_name".to_string(), format!("{owner}/{name}", owner=u, name=repo["name"]));
                repo.insert("link".to_string(), format!("https://github.com/{slug}", slug=repo["full_name"]));
                repo.insert("image".to_string(), format!("https://opengraph.githubassets.com/1/{slug}", slug=repo["full_name"]));
            });
            node.find(Class("pinned-item-desc")).for_each(|node| {
                repo.insert("description".to_string(), node.text().trim().to_string());
            });
            node.find(Attr("itemprop", "programmingLanguage"))
                .for_each(|node| {
                    repo.insert("language".to_string(), node.text().to_string());
                });
            node.find(Class("repo-language-color")).for_each(|node| {
                repo.insert(
                    "language-color".to_string(),
                    node.attr("style")
                        .unwrap()
                        .to_string()
                        .replace("background-color: ", ""),
                );
            });
            node.find(Class("pinned-item-meta")).for_each(|node| {
                let l = node.attr("href").unwrap_or("");
                if !l.is_empty() {
                    if l.contains("/network/members") {
                        repo.insert("forks".to_string(), node.text().trim().to_string());
                    } else if l.contains("/stargazers") {
                        repo.insert("stars".to_string(), node.text().trim().to_string());
                    }
                };
            });
            if !repo.contains_key("stars") {
                repo.insert("stars".to_string(), "0".to_string());
            }
            if !repo.contains_key("forks") {
                repo.insert("forks".to_string(), "0".to_string());
            }
            repos.push(repo);
        });

    repos
}

#[get("/")]
async fn index() -> Result<HttpResponse, http::Error> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(r#"<iframe width="100%" height="100%" src="https://www.youtube-nocookie.com/embed/Yw6u6YkTgQ4?controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#)
}

#[get("/{user}")]
async fn user(user: web::Path<String>) -> impl Responder {
    let r: Vec<HashMap<String, String>> = get_pinned(user.to_string()).await;
    json::to_string(&r)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Starting on port {}", 8080);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(user)
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}