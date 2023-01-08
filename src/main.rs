use headless_chrome::{Browser, LaunchOptions};
use std::{env, net::Ipv4Addr};
use warp::{http::Response, Filter};

struct Config {
    port: u16,
    is_headless: bool,
}

#[tokio::main]
async fn main() -> () {
    let config = load_config();
    let app = warp::get()
        .and(warp::path!("api" / "prerender" / String))
        .map(move |path| render_path(path, "html".to_owned(), false, config.is_headless))
        .or(
            warp::path!("api" / "prerender" / String / String / bool).map(
                move |url, element, is_http| render_path(url, element, is_http, config.is_headless),
            ),
        );
    warp::serve(app)
        .run((Ipv4Addr::LOCALHOST, config.port))
        .await;
    println!("Server started on port {}", config.port)
}

fn load_config() -> Config {
    let headless_key = "FUNCTIONS_HEADLESS_MODE";
    let is_headless: bool = match env::var(headless_key) {
        Ok(val) => val
            .parse()
            .expect("Headless mode not set. Defaulting to true"),
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

fn prerender(
    path: String,
    element: String,
    is_http: bool,
    is_headless: bool,
) -> Result<String, anyhow::Error> {
    let options = LaunchOptions::default_builder()
        .headless(is_headless)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let url = &(if is_http { "http://" } else { "https://" }.to_owned() + path.as_str());
    println!("Fetching {}", &url);
    let browser = Browser::new(options)?;
    let tab = &browser.wait_for_initial_tab()?;
    tab.navigate_to(&url).expect("failed to navigate");
    println!("Waiting for element {}", &element);
    tab.wait_for_element(&element).expect("failed to wait for element");
    let content = &tab.get_content()?;
    println!("Done. Content size: {}kb", content.len() / 1024);
    Ok(content.to_string())
}
