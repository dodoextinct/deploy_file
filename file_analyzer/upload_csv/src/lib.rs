use spin_sdk::http::{IntoResponse, Response, Request};
use spin_sdk::http_component;
use anyhow::Result;

#[http_component]
pub fn process_csv(req: Request) -> Result<impl IntoResponse> {
    let data = req.into_body();
    let string_data = String::from_utf8(data.to_vec())?;

    print!("{:?}", string_data);
    
    //let response = http::Response::builder()
    //.status(200)
    //.header("content-type", "text/plain")
    //.body(string_data)?;
    
    let res = Response::new(200, string_data);
    Ok(res)
}
