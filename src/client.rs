use rsdb::rsdb_client::RsdbClient;
use rsdb::SetStringRequest;
pub mod rsdb {
    tonic::include_proto!("rsdb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RsdbClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SetStringRequest {
        key: "Tonic".into(),
        val: "val".into(),
    });

    let response = client.set_string(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
