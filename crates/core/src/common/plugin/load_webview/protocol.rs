use std::path::PathBuf;

use wry::http::header::{CONTENT_SECURITY_POLICY, CONTENT_TYPE};
use wry::http::Response;
use wry::{http, WebViewBuilder};

use crate::common::bundle::WebviewUri;
use crate::prelude::csp::Csp;
use crate::WryLocalRoot;

pub fn feed_uri<'a>(
    builder: WebViewBuilder<'a>,
    uri: &WebviewUri,
    local_root: &WryLocalRoot,
    csp: Option<Csp>,
) -> WebViewBuilder<'a> {
    let builder = builder.with_url(uri.0.to_string());
    feed_custom_protocol(builder, local_root.clone(), csp)
}

fn feed_custom_protocol(
    builder: WebViewBuilder,
    local_root: WryLocalRoot,
    csp: Option<Csp>,
) -> WebViewBuilder {
    let local_root = local_root.0;
    builder.with_custom_protocol("flurx".to_string(), move |_, request| {
        match get_response(request, &local_root, &csp) {
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

fn get_response(
    request: wry::http::Request<Vec<u8>>,
    local_root: &PathBuf,
    csp: &Option<Csp>,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let path = request.uri().path();
    let root = PathBuf::from("assets").join(local_root);
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

    let mut response_builder = Response::builder();
    if let Some(csp) = csp {
        response_builder = response_builder.header(CONTENT_SECURITY_POLICY, csp.0.as_str());
    }
    response_builder
        .header(CONTENT_TYPE, mimetype)
        .body(content)
        .map_err(Into::into)
}
