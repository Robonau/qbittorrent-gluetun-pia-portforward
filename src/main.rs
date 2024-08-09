use dotenv::dotenv;
extern crate reqwest;
use reqwest::header;
use std::{env, fs::read_to_string};
use tokio::time::{sleep, Duration};
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let Ok(qbittorrent_url) = env::var("QBITTORRENT_URL") else {
        panic!("qbittorrent_url is not set");
    };
    let Ok(qbittorrent_username) = env::var("QBITTORRENT_USERNAME") else {
        panic!("qbittorrent_username is not set");
    };
    let Ok(qbittorrent_password) = env::var("QBITTORRENT_PASSWORD") else {
        panic!("qbittorrent_password is not set");
    };
    let forwarded_port = env::var("FORWARDED_PORT_DIR");
    let forwarded_port_dir: String = match forwarded_port {
        Ok(dir) => dir,
        Err(_) => "/forwarded_port".to_string(),
    };
    let client: reqwest::Client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();
    loop {
        println!("{}", forwarded_port_dir);
        let Ok(port) = read_to_string(&forwarded_port_dir) else {
            panic!("failed to read forwarded_port_dir");
        };
        println!("{}", port);
        let Some(logged_in) = login(
            &client,
            &qbittorrent_url,
            &qbittorrent_username,
            &qbittorrent_password,
        )
        .await
        else {
            panic!("failed to login");
        };
        println!("{}", logged_in);
        let Some(preferences) = get_listen_port(&client, &qbittorrent_url).await else {
            panic!("failed to get listen_port");
        };
        println!("{}", preferences.listen_port);

        if port.trim() != preferences.listen_port.to_string() {
            let Some(set_it) =
                set_listen_port(&client, &qbittorrent_url, port.trim().parse().unwrap()).await
            else {
                panic!("failed to set listen_port");
            };
            println!("?{}?", set_it);
        }
        sleep(Duration::from_secs(15 * 60)).await;
    }
}

async fn login(
    client: &reqwest::Client,
    qbittorrent_url: &str,
    qbittorrent_username: &str,
    qbittorrent_password: &str,
) -> Option<String> {
    let mut url = Url::parse(qbittorrent_url).ok()?;
    url.set_path("/api/v2/auth/login");

    let mut headers = header::HeaderMap::new();
    headers.insert("Referer", qbittorrent_url.parse().unwrap());
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    println!("{}", url);
    client
        .post(url)
        .headers(headers)
        .body(format!(
            "username={}&password={}",
            qbittorrent_username, qbittorrent_password
        ))
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()
}

#[derive(serde::Deserialize)]
struct Preferences {
    listen_port: u32,
}

async fn get_listen_port(client: &reqwest::Client, qbittorrent_url: &str) -> Option<Preferences> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Referer", qbittorrent_url.parse().unwrap());
    let mut url = Url::parse(qbittorrent_url).ok()?;
    url.set_path("/api/v2/app/preferences");
    println!("{}", url);
    client
        .get(url)
        .headers(headers)
        .send()
        .await
        .ok()?
        .json::<Preferences>()
        .await
        .ok()
}

async fn set_listen_port(
    client: &reqwest::Client,
    qbittorrent_url: &str,
    listen_port: u32,
) -> Option<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Referer", qbittorrent_url.parse().unwrap());
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    let mut url = Url::parse(qbittorrent_url).ok()?;
    url.set_path("/api/v2/app/setPreferences");
    println!("{}", url);
    client
        .post(url)
        .headers(headers)
        .body(format!("json={{\"listen_port\": {}}}", listen_port))
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()
}
