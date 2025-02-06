## Unreleased

- Release [`bevy_child_window`](https://github.com/not-elm/bevy_child_window) that helps `bevy_webview_wry` to create a
  child window.
- Support embedded html content.
- Add child window example.
- Fix the bug that a new window is created when a redirect occurs

### Breaking changes

- Rename `ParentWindow` to `EmbedWithin`
- `WebviewUri` is no longer a component. Instead, `Webview` has been added.

## v0.1.0

First release!
Please feel free to report me at [issue](https://github.com/not-elm/bevy_webview_wry/issues).