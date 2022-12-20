#[path = "./gobgp/apipb.rs"]
pub mod gobgp;

use gobgp::gobgp_api_client::GobgpApiClient;
use gobgp::{ListPeerResponse, ListPeerRequest};
use tonic::Request;


#[tokio::main]
async fn gobgp_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GobgpApiClient::connect("http://127.0.0.1:50051").await?;

    let request = Request::new(ListPeerRequest { address: "10.10.10.3".to_string(), enable_advertised: false});

    let response = client.list_peer(request).await?;

    println!("RESPONSE={:?}", response);

    return Ok(())
}
