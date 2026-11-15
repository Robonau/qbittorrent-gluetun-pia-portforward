use dotenv::dotenv;
extern crate reqwest;
use reqwest::header;
use std::{
    env,
    fs::{read_dir, read_to_string},
};
use tokio::time::{sleep, Duration};
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let Ok(qbittorrent_url) = env::var("QBITTORRENT_URL") else {
        panic!("qbittorrent_url is not set");
    };

    let (api_key, qbittorrent_username, qbittorrent_password) =
        match env::var("QBITTORRENT_API_KEY") {
            Ok(api_key) => (Some(api_key), None, None),
            Err(_) => {
                let username =
                    env::var("QBITTORRENT_USERNAME").expect("qbittorrent_username is not set");

                let password =
                    env::var("QBITTORRENT_PASSWORD").expect("qbittorrent_password is not set");

                (None, Some(username), Some(password))
            }
        };

    let forwarded_port = env::var("FORWARDED_PORT_DIR");
    let forwarded_port_dir: String = match forwarded_port {
        Ok(dir) => dir + "/forwarded_port",
        Err(_) => "/pia/forwarded_port".to_string(),
    };

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(api_key.is_none())
        .build()
        .unwrap();

    loop {
        if let Ok(tmp) = read_dir(&forwarded_port_dir.replace("/forwarded_port", "")) {
            tmp.for_each(|x| println!("read_dir {:?}", x.unwrap().path()));
        }

        println!("{}", forwarded_port_dir);

        let Ok(port) = read_to_string(&forwarded_port_dir) else {
            println!("{}", forwarded_port_dir);
            panic!("failed to read forwarded_port_dir");
        };

        println!("{}", port.trim());

        if api_key.is_none() {
            let Some(logged_in) = login(
                &client,
                &qbittorrent_url,
                qbittorrent_username.as_deref().unwrap(),
                qbittorrent_password.as_deref().unwrap(),
            )
            .await
            else {
                panic!("failed to login");
            };

            println!("{}", logged_in);
        }

        let Some(preferences) =
            get_listen_port(&client, &qbittorrent_url, api_key.as_deref()).await
        else {
            panic!("failed to get listen_port");
        };

        println!("{}", preferences.listen_port);

        if port.trim() != preferences.listen_port.to_string() {
            let Some(set_it) = set_listen_port(
                &client,
                &qbittorrent_url,
                port.trim().parse().unwrap(),
                api_key.as_deref(),
            )
            .await
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

    println!("{}", url);

    client
        .post(url)
        .headers(headers)
        .form(&[
            ("username", qbittorrent_username),
            ("password", qbittorrent_password),
        ])
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

async fn get_listen_port(
    client: &reqwest::Client,
    qbittorrent_url: &str,
    api_key: Option<&str>,
) -> Option<Preferences> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Referer", qbittorrent_url.parse().unwrap());

    if let Some(api_key) = api_key {
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", api_key).parse().unwrap(),
        );
    }

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
    api_key: Option<&str>,
) -> Option<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Referer", qbittorrent_url.parse().unwrap());

    if let Some(api_key) = api_key {
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", api_key).parse().ok()?,
        );
    }

    let mut url = Url::parse(qbittorrent_url).ok()?;
    url.set_path("/api/v2/app/setPreferences");

    println!("{}", url);

    client
        .post(url)
        .headers(headers)
        .form(&[("json", format!("{{\"listen_port\": {}}}", listen_port))])
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()
}
