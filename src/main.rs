use std::{env, net::Ipv4Addr, time::Duration};
use warp::{http::Response, Filter};
use headless_chrome::{Browser, LaunchOptions};

struct Config {
    port: u16,
    is_headless: bool,
}

#[tokio::main]
async fn main() -> () {
    let config = load_config();
    println!("Starting server on port {}", config.port);
    let app = warp::get()
        .and(warp::path!("api" / "prerender" / String))
        .map(move |path| render_path(path, "html".to_owned(), false, config.is_headless))
        .or(
            warp::path!("api" / "prerender" / String / String / bool)
                .map(move |param, element, is_http| render_path(param, element, is_http, config.is_headless)),
        );
    warp::serve(app).run((Ipv4Addr::LOCALHOST, config.port)).await
}

fn load_config() -> Config {
    let headless_key = "FUNCTIONS_HEADLESS_MODE";
    let is_headless: bool = match env::var(headless_key) {
        Ok(val) => val.parse().expect("Headless mode not set. Defaulting to true"),
        Err(_) => true,
    };
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };
    Config { port, is_headless }
}

fn render_path(
    param: String,
    element: String,
    is_http: bool,
    is_headless: bool,
) -> Result<Response<String>, warp::http::Error> {
    return match prerender(param, element, is_http, is_headless) {
        Ok(it) => Response::builder().body(it),
        Err(err) => Response::builder()
            .status(500)
            .body(err.to_string().to_owned()),
    };
}

fn prerender(path: String, element: String, is_http: bool, is_headless: bool) -> Result<String, anyhow::Error> {
    let options = LaunchOptions::default_builder()
        .headless(is_headless)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let url = &(if is_http { "http://" } else { "https://" }.to_owned() + path.as_str());
    println!("Fetching {}", &url);

    let content = Browser::new(options)?
        .wait_for_initial_tab()?
        .navigate_to(&url)?
        .wait_until_navigated()?
        .wait_for_element_with_custom_timeout(&element, Duration::from_secs(10))?
        .get_content();
    Ok(content?)
}
