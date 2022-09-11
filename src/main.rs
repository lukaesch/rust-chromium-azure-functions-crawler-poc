use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

use headless_chrome::{Browser, LaunchOptions};

#[tokio::main]
async fn main() {
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get-prerender"))
        .map(|| Response::builder().body(prerender()));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}

fn prerender() -> String {
    let options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = match Browser::new(options) {
        Ok(it) => it,
        Err(err) => return err.to_string().to_owned(),
    };
    let tab = match browser.wait_for_initial_tab() {
        Ok(it) => it,
        Err(err) => return err.to_string().to_owned(),
    };
    let content = match tab.navigate_to("https://www.sotrusty.com/").unwrap().wait_until_navigated().unwrap().get_content() {
        Ok(it) => it,
        Err(err) => return err.to_string().to_owned(),
    };
    return content;
}