#[path = "./gobgp/apipb.rs"]
pub mod gobgp;

use gobgp::gobgp_api_client::GobgpApiClient;
use gobgp::{
    family, AddPathRequest, Family, FlowSpecIpPrefix, FlowSpecNlri, Path,
    RedirectIPv4AddressSpecificExtended, TableType,
};
use prost::Message;
use tonic::Request;

const TYPE_URL_PREFACE: &str = "type.googleapis.com/apipb";

pub(crate) fn to_any<T: prost::Message>(m: T, name: &str) -> prost_types::Any {
    prost_types::Any {
        type_url: format!("{}.{}", TYPE_URL_PREFACE, name),
        value: m.encode_to_vec(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = GobgpApiClient::connect("http://127.0.0.1:50051").await?;
    let nlri = to_any(
        FlowSpecNlri {
            rules: prost::alloc::vec![to_any(
                FlowSpecIpPrefix {
                    r#type: 2, //type 2 is source prefix
                    prefix: "10.64.0.54".to_string(),
                    prefix_len: 32,
                    offset: 0,
                },
                "FlowSpecIpPrefix"
            )],
        },
        "FlowSpecNlri",
    );

    let path = Path {
        nlri: Some(nlri),
        family: Some(Family {
            afi: family::Afi::Ip as i32,
            safi: family::Safi::FlowSpecUnicast as i32,
        }),
        pattrs: vec![prost_types::Any {
            type_url: "/apipb.RedirectIPv4AddressSpecificExtended".to_string(),
            value: RedirectIPv4AddressSpecificExtended {
                address: "192.167.5.3".to_string(),
                local_admin: 300,
            }
            .encode_to_vec(),
        }],
        ..Path::default()
    };

    let request = Request::new(AddPathRequest {
        table_type: TableType::Global as i32,
        path: Some(path),
        vrf_id: "ipv4-flowspec".to_string(),
    });

    println!("SOMETHING");
    let stream_response = match client.add_path(request).await {
        Ok(response) => response.into_inner(),
        Err(err) => panic!("ERROR = {:#?}", err),
    };
    println!("SOMETHING");

    println!("NOTE = {:#?}", stream_response);

    return Ok(());
}
