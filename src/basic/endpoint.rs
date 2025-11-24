use http::{Request, Response};
use kinetics::macros::endpoint;
use kinetics::tools::config::Config as KineticsConfig;
use kinetics::tools::http::Body;
use serde_json::json;
use std::collections::HashMap;

/// REST API endpoint which responds with JSON {"success": true}
///
/// Test locally with the following command:
/// kinetics invoke BasicEndpointEndpoint
#[endpoint(url_path = "/endpoint")]
pub async fn endpoint(
    _event: Request<Body>,
    _secrets: &HashMap<String, String>,
    _config: &KineticsConfig,
) -> Result<Response<Body>, tower::BoxError> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!({"success": true}).to_string().into())
        .map_err(Box::new)?;

    Ok(resp)
}
