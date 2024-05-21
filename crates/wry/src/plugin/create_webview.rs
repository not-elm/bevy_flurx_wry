use std::path::PathBuf;

use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::prelude::{Commands, Entity, In, NonSend, NonSendMut, Query, Res, Window, With};
use bevy::winit::WinitWindows;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Reactor;
use wry::{http, WebView, WebViewBuilder, WebViewBuilderExtWindows};
use wry::http::header::CONTENT_TYPE;
use wry::http::Response;
use bevy_flurx_ipc::ipc_command_queue::{IpcCommand, IpcCommandQueue};

use crate::bundle::{AutoPlay, Background, EnableClipboard, Theme, Uri, UseDevtools, Visible, WebviewUninitialized};
use crate::plugin::on_page_load::{OnPageArgs, PageLoadEventQueue};
use crate::plugin::WebviewMap;

pub struct CreateWebviewPlugin;

impl Plugin for CreateWebviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, setup_new_windows);
    }
}

fn setup_new_windows(
    mut commands: Commands,
    views: Query<(
        Entity,
        &Uri,
        &UseDevtools,
        &AutoPlay,
        &EnableClipboard,
        &Visible,
        &Background,
        &Theme,
    ), (With<WebviewUninitialized>, With<Window>)>,
    ipc_queue: Res<IpcCommandQueue>,
    load_queue: Res<PageLoadEventQueue>,
    windows: NonSend<WinitWindows>,
) {
    for (
        entity,
        uri,
        use_devtools,
        auto_play,
        enable_clipboard,
        visible,
        background,
        theme,
    ) in views.iter() {
        let builder = {
            let Some(window) = windows.get_window(entity) else {
                continue;
            };
            let ipc_queue = ipc_queue.clone();
            let load_queue = load_queue.0.clone();
            wry::WebViewBuilder::new(window)
                .with_initialization_script(include_str!("./initialize.js"))
                .with_devtools(use_devtools.0)
                .with_autoplay(auto_play.0)
                .with_clipboard(enable_clipboard.0)
                .with_visible(visible.0)
                .with_theme(theme.as_wry_theme())
                .with_on_page_load_handler(move |event, uri| {
                    load_queue.lock().unwrap().push(OnPageArgs {
                        event,
                        uri,
                        entity,
                    });
                })
                .with_ipc_handler(move |request| {
                    ipc_queue.push(IpcCommand {
                        entity,
                        body: serde_json::from_str(request.body()).unwrap(),
                    });
                })
        };

        let builder = feed_background(builder, background);

        let builder = match uri {
            Uri::LocalRoot(content_root_dir) => {
                let content_root_dir = content_root_dir.clone();
                builder
                    .with_url("wry://localhost/".to_string())
                    .with_custom_protocol("wry".to_string(), move |request| {
                        match get_wry_response(request, &content_root_dir) {
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
        };

        let webview = builder.build().unwrap();
     
        commands.entity(entity).remove::<WebviewUninitialized>();
        commands.spawn(Reactor::schedule(move |task| async move {
            task.will(Update, once::run(insert_webview).with((entity, webview))).await;
        }));
    }
}

fn feed_background<'a>(builder: WebViewBuilder<'a>, background: &Background) -> WebViewBuilder<'a> {
    match background {
        Background::Unspecified => builder,
        Background::Transparent => builder.with_transparent(true),
        Background::Color(color) => {
            let rgba = color.as_rgba_u8();
            builder.with_background_color((rgba[0], rgba[1], rgba[2], rgba[3]))
        }
    }
}

fn get_wry_response(
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

fn insert_webview(
    In((entity, webview)): In<(Entity, WebView)>,
    mut view_map: NonSendMut<WebviewMap>,
) {
    view_map.0.insert(entity, webview);
}