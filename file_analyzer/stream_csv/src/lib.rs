use spin_sdk::{
    http::{IntoResponse, OutgoingRequest, Response, Method, Scheme, Headers, IncomingResponse, send, Request},
    http_component
};
use anyhow::{Result, bail};
use csv::ReaderBuilder;
use url::Url;

#[http_component]
pub async fn stream_csv(_req:Request)-> Result<impl IntoResponse>{
    let csv_url = "https://support.staffbase.com/hc/en-us/article_attachments/360009197031/username.csv";
    let url = Url::parse(csv_url).unwrap();
    let outgoing_request = OutgoingRequest::new(&Method::Get, Some(url.path()),
                                            Some(&match url.scheme() {
                                                "http" => Scheme::Http,
                                                "https" => Scheme::Https,
                                                scheme => Scheme::Other(scheme.into()),
                                            }),
                                Some(url.authority()),
                                &Headers::new(&[]),);

    let response:IncomingResponse = send(outgoing_request).await?;

    let status = response.status();

    if !(200..300).contains(&status) {
        bail!("unexpected status: {status}");
    }

    let body = response.into_body().await?;
    
    let reader = ReaderBuilder::new().has_headers(true).from_reader(body.as_slice());

    let mut total_cases = 0;

    for result in reader.into_records() {
        match result {
            Ok(record) => {
                let csv_record = record.iter().collect::<Vec<&str>>().join("; ");
                println!("CSV Record: {}", csv_record);
                total_cases+=1;
            }
            Err(err) => {
                eprintln!("CSV Parsing Error: {:?}", err);
            }
        }
    }

    let response_body = format!("Total cases: {}", total_cases);
    let response = Response::new(200, response_body);

    Ok(response)
}
