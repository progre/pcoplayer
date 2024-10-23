use anyhow::{anyhow, bail, Result};
use log::warn;
use m3u::Entry;
use reqwest::header::CONTENT_TYPE;
use url::Url;

use crate::bbs::{fetch_thread_url_encoding_name, parse_bbs_url};

async fn _check_subject(url: &Url) -> Result<bool> {
    let origin = url.origin().ascii_serialization();
    let resp = reqwest::Client::new()
        .head(format!("{}/subject.txt", origin))
        .send()
        .await?
        .error_for_status()?;
    Ok(matches!(
        resp.headers()
            .get(CONTENT_TYPE)
            .and_then(|x| x.to_str().ok())
            .map(|x| x.starts_with("text/plain")),
        Some(true)
    ))
}

fn is_probably_pls(path: &str) -> bool {
    if path.starts_with("/pls/") {
        return true;
    }
    false
}

async fn resolve_pls_to_stream(url: &str) -> Result<String> {
    let resp = reqwest::get(url).await?;
    let small_content_length = resp.content_length().map_or(false, |x| x < 1024 * 1024);
    let pls_content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .map_or(false, |x| x == "audio/x-mpegurl");
    if !small_content_length || !pls_content_type {
        bail!("Not a PLS file");
    }
    let text = resp.text().await?;
    let mut pls = m3u::Reader::new_ext(text.as_bytes())?;
    Ok(
        match pls
            .entry_exts()
            .next()
            .ok_or_else(|| anyhow!("No entry"))??
            .entry
        {
            Entry::Path(path) => path.to_string_lossy().to_string(),
            Entry::Url(url) => url.to_string(),
        },
    )
}

async fn check_stream(url: &str) -> Result<bool> {
    let resp = reqwest::Client::new()
        .head(url)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp
        .headers()
        .get(CONTENT_TYPE)
        .map_or(false, |x| x == "video/x-flv"))
}

pub enum UrlType {
    Bbs {
        thread_url: Url,
        charset: String,
        thread_name: String,
    },
    Stream {
        stream_url: String,
    },
    Unknown,
}

pub async fn resolve_url(url_str: &str) -> Result<UrlType> {
    let url = Url::parse(url_str)?;
    if !matches!(url.scheme(), "http" | "https") {
        bail!("Invalid URL scheme: {}", url.scheme())
    }
    let url = match parse_bbs_url(url) {
        Err(url) => url,
        Ok(bbs_url) => match fetch_thread_url_encoding_name(&bbs_url).await {
            Ok((thread_url, encoding, thread_name)) => {
                return Ok(UrlType::Bbs {
                    thread_url,
                    charset: encoding,
                    thread_name,
                });
            }
            Err(e) => {
                log::trace!("{:?}", e);
                warn!("Failed to fetch thread URL: {}", e);
                bbs_url.into_url()
            }
        },
    };
    let path = url.path();
    if is_probably_pls(path) {
        if let Ok(url) = resolve_pls_to_stream(url_str).await {
            return Ok(UrlType::Stream { stream_url: url });
        }
    }
    if check_stream(url_str).await? {
        return Ok(UrlType::Stream {
            stream_url: url_str.to_string(),
        });
    }
    Ok(UrlType::Unknown)
}
