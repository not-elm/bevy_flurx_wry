use std::path::PathBuf;

use wry::{http, WebViewBuilder};
use wry::http::header::CONTENT_TYPE;
use wry::http::Response;

use crate::bundle::Uri;

pub fn set_protocol<'a>(builder: WebViewBuilder<'a>, uri: &Uri) -> WebViewBuilder<'a> {
    match uri {
        Uri::LocalRoot(content_root_dir) => {
            let content_root_dir = content_root_dir.clone();
            builder
                .with_url("flurx://localhost/".to_string())
                .with_custom_protocol("flurx".to_string(), move |request| {
                    match get_response(request, &content_root_dir) {
                        Ok(r) => r.map(Into::into),
                        Err(e) => http::Response::builder()
                            .header(CONTENT_TYPE, "text/plain")
                            .status(500)
                            .body(e.to_string().as_bytes().to_vec())
                            .unwrap()
                            .map(Into::into),
                    }
                })
        }
        Uri::Remote(uri) => {
            builder.with_url(uri)
        }
    }
}

fn get_response(
    request: wry::http::Request<Vec<u8>>,
    content_root_dir: &str,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let path = request.uri().path();
    let root = PathBuf::from("assets/ui").join(content_root_dir);
    let path = if path == "/" {
        "index.html"
    } else {
        &path[1..]
    };
    let content = std::fs::read(std::fs::canonicalize(root.join(path))?)?;

    let mimetype = if path.ends_with(".html") || path == "/" {
        "text/html"
    } else if path.ends_with(".txt") {
        "text/plain"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".csv") {
        "text/csv"
    } else if path.ends_with(".js") {
        "text/javascript"
    } else if path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".gif") {
        "image/gif"
    } else if path.ends_with(".bmp") {
        "image/bmp"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".pdf") {
        "application/pdf"
    } else if path.ends_with(".zip") {
        "application/zip"
    } else if path.ends_with(".lzh") {
        "application/x-lzh"
    } else if path.ends_with(".tar") {
        "application/x-tar"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".mp3") {
        "audio/mp3g"
    } else if path.ends_with(".mp4") {
        "video/mp4"
    } else if path.ends_with(".mpeg") {
        "video/mpeg"
    } else {
        panic!("not implemented content type {path}");
    };

    Response::builder()
        .header(CONTENT_TYPE, mimetype)
        .body(content)
        .map_err(Into::into)
}
