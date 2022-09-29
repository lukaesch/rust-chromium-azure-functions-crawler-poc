use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

use headless_chrome::{Browser, LaunchOptions};

#[tokio::main]
async fn main() {
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get-prerender"))
        .map(|| Response::builder().body( match prerender() {
            Ok(it) => it,
            Err(err) => err.to_string().to_owned(),
        }));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}

fn prerender() -> Result<String, anyhow::Error>  {
    let options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    let content = tab.navigate_to("https://www.sotrusty.com/").unwrap().wait_until_navigated().unwrap().get_content()?;
    return Ok(content);
}