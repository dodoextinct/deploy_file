use spin_sdk::http::{IntoResponse, Response, Request};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component(route ="/")]
async fn handle_request(_req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::new(200, "/stream for streaming :: /upload for uploading"))
}
