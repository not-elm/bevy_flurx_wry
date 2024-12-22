# bevy_flurx_wry

[![Crates.io](https://img.shields.io/crates/v/bevy_flurx_wry.svg)](https://crates.io/crates/bevy_flurx_wry)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/not-elm/bevy_flurx_wry#license)
[![Crates.io](https://img.shields.io/crates/d/bevy_flurx_wry.svg)](https://crates.io/crates/bevy_flurx_wry)

> [!CAUTION]
> This crate is in the early stages of development and is subject to breaking changes.

## Purpose

The purpose of this crate is integrate [bevy](https://github.com/bevyengine/bevy)
and [wry](https://github.com/tauri-apps/wry) using [bevy_flurx](https://github.com/not-elm/bevy_flurx).

## Platform Support

The operation has been confirmed on `Windows` and `MacOS`.

`Linux` is currently not supported.

## Usage

There are two ways to create a webview:

### Converts an existing window into a webview window.

![simple](examples/simple.gif)

[examples/simple.rs](examples/simple.rs)

```rust
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_flurx_wry::prelude::*;

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    // Converts the `Window` attached the entity into a webview window. 
    commands.entity(window.single()).insert(
        WryWebViewBundle {
            uri: WebviewUri::new("https://bevyengine.org/"),
            ..default()
        }
    );
}
```

### Create a webview as child inside a window.

![child_view](examples/child_view.gif)
[examples/child_view.rs](examples/child_view.rs)

```rust
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_flurx_wry::prelude::*;

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        WryWebViewBundle {
            ..default()
        },
        AsChildBundle {
            // Here, create a webview as child inside a given window.
            parent: ParentWindow(window.single()),
            bounds: Bounds {
                position: Vec2::new(100., 100.),
                size: Vec2::new(500., 500.),
                min_size: Vec2::new(100., 100.),
            },
            ..default()
        },
    ));
}
```

## Ipc

### IpcEvent

You can listen events from the webview and, conversely, emit events to the webview.

#### Webview(javascript) -> bevy

[examples/event_listen.rs](examples/event_listen.rs)

___javascript___

```javascript
// you can use any type.
const event = {
    message: "message"
};
window.__FLURX__.emit("event_id", event);
```

___rust___

```rust
use bevy::prelude::*;
use bevy_flurx_wry::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MessageFromWebview {
    message: String,
}

fn read_webview_message(
    mut er: EventReader<IpcEvent<MessageFromWebview>>
) {
    for e in er.read() {
        println!("webview message: {}", e.payload.message);
    }
}
```

#### bevy -> Webview(javascript)

[examples/event_emit.rs](examples/event_emit.rs)

___javascript___

```javascript
window.__FLURX__.listen("event_id", ({message}) => {
    console.log(message);
});
```

___rust___

```rust
use bevy::prelude::*;
use bevy_flurx_wry::prelude::*;
use serde_json::json;

fn emit_event(
    mut views: Query<&mut EventEmitter>
) {
    for mut emitter in views.iter_mut() {
        emitter.emit("event_id", &serde_json::json!({
            "message" : "hello world!"
        }));
    }
}
```

### IpcCommand

`IpcEvent` can't receive the output value from the other side.
In this case, `IpcCommand` can be used.

`IpcComamnd` can be divided into two command patterns: action-command, task-command

Please check  [examples/ipc_command.rs](examples/ipc_command.rs) for details.

## ChangeLog

Please see [here](https://github.com/not-elm/bevy_flurx_wry/blob/main/CHANGELOG.md).

## Compatible Bevy versions

| bevy_flurx_wry | bevy_flurx | bevy    |
|----------------|------------|---------|
| 0.1.0-alpha1   | 0.5.2      |  0.13.2 |

## License

This crate is licensed under the MIT License or the Apache License 2.0.

## Todo
- [ ] Bugfix
  - [ ] The webview could be moved without dragging.
  - [ ] `with_initialization_script` does not execute before `window.onload`
- [ ] Api
  - [x] fs
  - [x] clipboard
  - [x] dialog
  - [ ] http
  - [ ] mocks
  - [x] notification
  - [x] os
  - [x] path
  - [ ] shell
  - [x] monitor
  - [x] window
- [ ] Security
  - [x] csp 
  - [ ] scope
- [ ] Support Linux(X11)
- [ ] Support Linux(Wayland)