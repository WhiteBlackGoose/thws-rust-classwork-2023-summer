use hyper::{Client, StatusCode, body::HttpBody};

#[tokio::main]
async fn main() {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let uri = "https://www.fhws.de".parse().unwrap();
    let mut response = client.get(uri).await.unwrap();
    if response.status() != StatusCode::OK {
        println!("Unexpected error code {}", response.status());
        return;
    }
    let body = response.body_mut();
    let body = String::from_utf8(body.data().await.unwrap().unwrap().to_vec()).unwrap();
    println!("{}", body);
}
