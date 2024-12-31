//! Allows you to use http apis.

use crate::error::ApiResult;
use crate::macros::api_plugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::{In, PluginGroup, Update};
use bevy::utils::HashMap;
use bevy_flurx::prelude::effect;
use bevy_flurx::task::ReactorTask;
use bevy_flurx_ipc::command;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Allows you to use all http plugins.
///
/// ## Plugins
/// - [HttpFetchApi]
pub struct AllHttpPlugins;
impl PluginGroup for AllHttpPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(HttpFetchApi)
    }
}

api_plugin!(
    /// You'll be able to use fetch api.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const response = await window.__FLURX__.http.fetch("https://example.com");
    /// ```
    HttpFetchApi,
    command: fetch
);

#[derive(Deserialize, Debug, Default)]
struct Args {
    url: String,
    body: Option<Vec<u8>>,
    method: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl Args {
    async fn fetch(self) -> ApiResult<Response> {
        let mut client = reqwest::Client::new().request(self.method()?, &self.url);
        if let Some(headers) = self.headers {
            client = client.headers(to_header_map(&headers)?);
        }
        if let Some(body) = self.body {
            client = client.body(body);
        }
        Ok(client.send().await?)
    }

    fn method(&self) -> ApiResult<Method> {
        match self.method.as_ref() {
            Some(method) => Ok(Method::from_str(method.as_str())?),
            None => Ok(Method::GET),
        }
    }
}

#[derive(Serialize)]
struct Output {
    body: Vec<u8>,
    headers: HashMap<String, String>,
    status: u16,
    #[serde(rename = "statusText")]
    status_text: String,
}

#[command(id = "FLURX|http::fetch", internal)]
async fn fetch(In(args): In<Args>, task: ReactorTask) -> ApiResult<Output> {
    task.will(
        Update,
        effect::tokio::spawn(|args: Args| async move {
            println!("args: {args:?}");
            let response = args.fetch().await?;
            let status = response.status();
            Ok(Output {
                headers: to_hash_map(response.headers()),
                body: response.bytes().await?.to_vec(),
                status: status.as_u16(),
                status_text: status.to_string(),
            })
        })
        .with(args),
    )
    .await
}

fn to_header_map(headers: &HashMap<String, String>) -> ApiResult<HeaderMap> {
    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        header_map.insert(HeaderName::from_str(name)?, value.parse()?);
    }
    Ok(header_map)
}

fn to_hash_map(headers: &HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .filter_map(|(k, v)| Some((k.to_string(), v.to_str().ok()?.to_string())))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::error::ApiResult;
    use crate::http::Args;
    use bevy::utils::default;
    use reqwest::Method;

    #[test]
    fn create_methods() -> ApiResult {
        assert_eq!(
            Args {
                method: None,
                ..default()
            }
            .method()?,
            Method::GET
        );
        assert_eq!(
            Args {
                method: Some("GET".to_string()),
                ..default()
            }
            .method()?,
            Method::GET
        );
        assert_eq!(
            Args {
                method: Some("POST".to_string()),
                ..default()
            }
            .method()?,
            Method::POST
        );
        assert_eq!(
            Args {
                method: Some("PUT".to_string()),
                ..default()
            }
            .method()?,
            Method::PUT
        );
        assert_eq!(
            Args {
                method: Some("DELETE".to_string()),
                ..default()
            }
            .method()?,
            Method::DELETE
        );
        Ok(())
    }
}
