use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

use headless_chrome::{Browser, LaunchOptions};

#[tokio::main]
async fn main() -> () {
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get-prerender"))
        .map(|| match prerender() {
            Ok(it) => Response::builder().body(it),
            Err(err) => Response::builder()
                .status(500)
                .body(err.to_string().to_owned()),
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}

fn prerender() -> Result<String, anyhow::Error> {
    let options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let content = Browser::new(options)?
        .wait_for_initial_tab()?
        .navigate_to("https://www.sotrusty.com/")?
        .wait_until_navigated()?
        .get_content()?;
        
    return Ok(content);
}
