//Paragonii's part here
#[get("/{user}")]
async fn user_repos() -> impo Responder {
    HttpResponse::build(http::StatusCode::OK)
    SomeScraperCode(".pinned-item-list-item.public")
}

let client = reqwest::blocking::Client::builder().cookie_store(true).build()?;
client.post("https://www.lycamobile.es/wp-admin/admin-ajax.php")
    .form(&[
            ("action", "lyca_login_ajax"),
            ("method", "login"),
            ("mobile_no", "<MOBILE_PHONE_NUMBER>"),
            ("pass", "<SUPER_SECRET_PASSWORD>")
    ])
    .send()?;