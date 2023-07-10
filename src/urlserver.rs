use actix_web::{cookie::time::format_description::well_known::Rfc3339, web::Query, HttpResponse};
use lazy_static::lazy_static;
use log::{debug, error, info};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use time::OffsetDateTime;
use utoipa::IntoParams;
use webpage::{Webpage, WebpageOptions};

fn extract_urls(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref URL_REGEX: Regex = Regex::new(r#"(?i)\b((?:[a-z][\w-]+:(?:/{1,3}|[a-z0-9%])|www\d{0,3}[.]|[a-z0-9.\-]+[.][a-z]{2,4}/)(?:[^\s()<>]+|\(([^\s()<>]+|(\([^\s()<>]+\)))*\))+(?:\(([^\s()<>]+|(\([^\s()<>]+\)))*\)|[^\s`!()\[\]{};:'".,<>?«»“”‘’]))"#).unwrap();
    }
    let urls = URL_REGEX.find_iter(text).map(|mat| mat.as_str()).collect();
    info!("{:?} ➡ Found URLs: {:?}", text, urls);
    return urls;
}

fn read_pool_json(pool: &str) -> Value {
    let poolfile = format!("urlpools/{}.json", pool);
    let poolfile_exists = std::path::Path::new(&poolfile).exists();

    // Check/read pool.json
    if poolfile_exists {
        debug!("Reading poolfile: {}", poolfile);
        let json_string = {
            let text = std::fs::read_to_string(&poolfile).unwrap();
            serde_json::from_str::<Value>(&text).unwrap()
        };
        return json_string;
    } else {
        debug!("Creating poolfile: {}", poolfile);
        let json_string = serde_json::json!({"unrequested":[], "requested":[]});
        return json_string;
    }
}

fn write_pool_json(json_str: &Value, pool: &String) {
    match std::fs::create_dir_all("urlpools/") {
        Err(e) => error!("{:?}", e),
        _ => (),
    }
    let poolfile = format!("urlpools/{}.json", pool);
    debug!("Writing poolfile: {}", poolfile);
    std::fs::write(poolfile, serde_json::to_string_pretty(&json_str).unwrap()).unwrap();
}

#[derive(Deserialize, IntoParams)]
pub struct UrlPush {
    pool: String,
    url_text: String,
}

struct WebpageInfo {
    url: String,
    title: String,
    push_time: String,
}

#[utoipa::path(post, path="/add", context_path="/urls",
    responses(
        (status = OK, description = "OK - added URL(s) to pool", content_type="text/plain"),
    ),
    tag = "URL Server",
    params(UrlPush)
)]
pub async fn add(urlpush: Query<UrlPush>) -> String {
    let urls = extract_urls(&urlpush.url_text);
    let mut return_string = format!("Added to pool {}:\n", urlpush.pool);
    let mut json_string = read_pool_json(&urlpush.pool);

    for url in urls.iter() {
        let info =
            Webpage::from_url(url, WebpageOptions::default()).expect("Could not read from URL");
        let webpage = WebpageInfo {
            url: info.http.url.to_string(),
            title: info
                .html
                .title
                .unwrap_or("#NoTitle".to_string())
                .to_string(),
            push_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
        };

        return_string.push_str(format!("{}\n", webpage.url).as_str());
        json_string["unrequested"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({"url": webpage.url, "title": webpage.title, "push_time": webpage.push_time}));
    }
    write_pool_json(&json_string, &urlpush.pool);

    return return_string;
}

#[derive(Deserialize, IntoParams)]
pub struct UrlPull {
    pool: String,
    include_seen: bool,
}

#[utoipa::path(get, path="/get", context_path="/urls",
    responses(
        (status = OK, description = "OK - returns URLs", content_type="application/json" ),
        (status = FORBIDDEN, description = "Something failed.", content_type="application/json" ),
    ),
    tag = "URL Server",
    params(UrlPull)
)]
pub async fn get(urlpull: Query<UrlPull>) -> HttpResponse {
    //TODO: read urls from json_string["unrequested"] & move to requested
    let mut json_string: Value = read_pool_json(&urlpull.pool);
    let mut return_json: Value = serde_json::json!({"unrequested": [], "requested": []});
    let mut new_json: Value = serde_json::json!({"unrequested": [], "requested": []});

    for url in json_string["requested"].as_array_mut().unwrap().iter() {
        if urlpull.include_seen {
            return_json["requested"]
                .as_array_mut()
                .unwrap()
                .push(url.clone());
        }
        new_json["requested"]
            .as_array_mut()
            .unwrap()
            .push(url.clone());
    }
    for url in json_string["unrequested"].as_array_mut().unwrap().iter() {
        return_json["unrequested"]
            .as_array_mut()
            .unwrap()
            .push(url.clone());
        new_json["requested"]
            .as_array_mut()
            .unwrap()
            .push(url.clone());
    }

    write_pool_json(&new_json, &urlpull.pool);
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(return_json.to_string());
}
