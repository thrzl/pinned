//Paragonii's part here
#[get("/{user}")]
async fn user_repos() -> impo Responder {
    HttpResponse::build(http::StatusCode::OK)
    SomeScraperCode(".pinned-item-list-item.public")
}

let client = reqwest::blocking::Client::builder().cookie_store(true).build()?;
client.post("https://github.com")
    .form(&[
    ])
    .send()?;

    let response = client.get("https://github.com").send()?;
    let body_response = response.text()?;
    
    let selector = &Selector::parse("p.bdl-balance > span")
    .expect("Error during the parsing using the given selector");
let span_text = parsed_html
    .select(selector)
    .flat_map(|el| el.text())
    .collect()

    span_text.split("hasta").nth(1)
    .expect("Can't get the expiration date correctly")
    .trim_start().to_string()

    let selector = &Selector::parse("div.bdl-mins")
    .expect("Error during the parsing using the given selector");
let div_text = parsed_html
    .select(selector)
    .flat_map(|el| el.text())
    .collect()

    div_text.get(2..)
    .unwrap_or_else(||"Can't get the internet balance correctly")
    .to_string()

    


