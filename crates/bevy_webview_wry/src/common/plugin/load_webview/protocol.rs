use std::path::PathBuf;

use wry::http::header::{CONTENT_SECURITY_POLICY, CONTENT_TYPE};
use wry::http::Response;
use wry::{http, WebViewBuilder};

use crate::prelude::{Csp, Webview};
use crate::WryLocalRoot;

pub fn feed_uri<'a>(
    builder: WebViewBuilder<'a>,
    webview: &Webview,
    local_root: &WryLocalRoot,
    csp: Option<Csp>,
) -> WebViewBuilder<'a> {
    let builder = match webview {
        Webview::Uri(uri) => builder.with_url(&uri.0),
        Webview::Html(html) => builder.with_html(html),
    };
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
    let mimetype = if path.ends_with(".htm") || path.ends_with(".html") || path == "/" {
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
    } else if path.ends_with(".aac") {
        "audio/aac"
    } else if path.ends_with(".abw") {
        "application/x-abiword"
    } else if path.ends_with(".arc") {
        "application/x-freearc"
    } else if path.ends_with(".avi") {
        "video/m-msvideo"
    } else if path.ends_with(".azw") {
        "application/vnd.amazon.ebook"
    } else if path.ends_with(".bin") {
        "application/octet-stream"
    } else if path.ends_with(".bz") {
        "application/x-bzip"
    } else if path.ends_with("bz2") {
        "application/x-bzip2"
    } else if path.ends_with(".csh") {
        "application/x-csh"
    } else if path.ends_with(".doc") {
        "application/msword"
    } else if path.ends_with(".docx") {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    } else if path.ends_with(".eot") {
        "application/vnd.ms-fontobject"
    } else if path.ends_with(".epub") {
        "application/epub+zip"
    } else if path.ends_with(".gz") {
        "application/gzip"
    } else if path.ends_with(".ico") {
        "image/vnd.microsoft.icon"
    } else if path.ends_with(".ics") {
        "text/calendar"
    } else if path.ends_with(".jar") {
        "application/java-archive"
    } else if path.ends_with(".jpeg") || path.ends_with(".jpg") {
        "image/jpeg"
    } else if path.ends_with(".mid") || path.ends_with(".midi") {
        "audio/midi"
    } else if path.ends_with(".mpkg") {
        "application/vnd.apple.installer+xml"
    } else if path.ends_with(".odp") {
        "application/vnd.oasis.opendocument.presentation"
    } else if path.ends_with(".ods") {
        "application/vnd.oasis.opendocument.spreadsheet"
    } else if path.ends_with(".odt") {
        "application/vnd.oasis.opendocument.text"
    } else if path.ends_with(".oga") {
        "audio/ogg"
    } else if path.ends_with(".ogv") {
        "video/ogg"
    } else if path.ends_with(".ogx") {
        "application/ogg"
    } else if path.ends_with(".otf") {
        "font/otf"
    } else if path.ends_with(".ppt") {
        "application/vnd.ms-powerpoint"
    } else if path.ends_with(".pptx") {
        "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    } else if path.ends_with(".rar") {
        "application/vnd.rar"
    } else if path.ends_with(".rtf") {
        "application/rtf"
    } else if path.ends_with(".sh") {
        "application/x-sh"
    } else if path.ends_with(".swf") {
        "application/x-shockwave-flash"
    } else if path.ends_with(".tif") || path.ends_with(".tiff") {
        "image/tiff"
    } else if path.ends_with(".ttf") {
        "font/ttf"
    } else if path.ends_with(".vsd") {
        "application/vnd.visio"
    } else if path.ends_with(".wav") {
        "audio/wav"
    } else if path.ends_with(".weba") {
        "audio/webm"
    } else if path.ends_with(".webm") {
        "video/web"
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
