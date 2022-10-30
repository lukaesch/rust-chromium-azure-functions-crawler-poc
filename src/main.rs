use std::{net::Ipv4Addr, time::Duration};

use std::env;
use warp::{http::Response, Filter};

use headless_chrome::{Browser, LaunchOptions};

#[tokio::main]
async fn main() -> () {
    let example1 = warp::get()
        .and(warp::path!("api" / "get-prerender" / String))
        .map(|path| render_path(path, "html".to_owned(), false))
        .or(
            warp::path!("api" / "get-prerender" / String / String / bool)
                .map(|param, element, is_http| render_path(param, element, is_http)),
        );

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}

fn render_path(
    param: String,
    element: String,
    is_http: bool,
) -> Result<Response<String>, warp::http::Error> {
    return match prerender(param, element, is_http) {
        Ok(it) => Response::builder().body(it),
        Err(err) => Response::builder()
            .status(500)
            .body(err.to_string().to_owned()),
    };
}

fn prerender(path: String, element: String, is_http: bool) -> Result<String, anyhow::Error> {
    let options = LaunchOptions::default_builder()
        .headless(false)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let url = &(if is_http { "http://" } else { "https://" }.to_owned() + path.as_str());
    println!("{}", &url);

    let content = Browser::new(options)?
        .wait_for_initial_tab()?
        .navigate_to(&url)?
        .wait_until_navigated()?
        .wait_for_element_with_custom_timeout(&element, Duration::from_secs(10))?
        .get_content();
    Ok(content?)
}
