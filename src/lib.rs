use aws_sdk_sqs::operation::send_message::builders::SendMessageFluentBuilder;
use kinetics_macro::endpoint;
use lambda_http::{Body, Error, Request, Response};
use std::collections::HashMap;

#[endpoint(
    url_path = "/endpoint",
    environment = {"SOME_VAR": "somevalue"},
)]
pub async fn endpoint(
    _event: Request,
    _secrets: &HashMap<String, String>,
    _queues: &HashMap<String, SendMessageFluentBuilder>,
) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;

    Ok(resp)
}
