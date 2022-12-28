pub mod gobgp {
    tonic::include_proto!{"apipb"}
}
use gobgp::gobgp_api_client::GobgpApiClient;
use gobgp::{
    family, AddPathRequest, ExtendedCommunitiesAttribute, Family, FlowSpecIpPrefix, FlowSpecNlri,
    MpReachNlriAttribute, OriginAttribute, Path, RedirectIPv4AddressSpecificExtended, TableType,
};
use std::time::SystemTime;
use tonic::Request;

const TYPE_URL_PREFACE: &str = "type.googleapis.com/apipb";

pub(crate) fn to_any<T: prost::Message>(m: T, name: &str) -> prost_types::Any {
    prost_types::Any {
        type_url: format!("{}.{}", TYPE_URL_PREFACE, name),
        value: m.encode_to_vec(),
    }
}

pub(crate) trait ToApi<T: prost::Message> {
    fn to_api(&self) -> T;
}

impl ToApi<prost_types::Timestamp> for SystemTime {
    fn to_api(&self) -> prost_types::Timestamp {
        let unix = self.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        prost_types::Timestamp {
            seconds: unix.as_secs() as i64,
            nanos: unix.subsec_nanos() as i32,
        }
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
                    prefix_len: 24,
                    offset: 0,
                },
                "FlowSpecIPPrefix"
            )],
        },
        "FlowSpecNLRI",
    );

    let path = Path {
        nlri: Some(nlri.clone()),
        family: Some(Family {
            afi: family::Afi::Ip as i32,
            safi: family::Safi::FlowSpecUnicast as i32,
        }),
        pattrs: prost::alloc::vec![
            to_any(
                ExtendedCommunitiesAttribute {
                    communities: prost::alloc::vec![to_any(
                        RedirectIPv4AddressSpecificExtended {
                            address: "192.167.5.3".to_string(),
                            local_admin: 24,
                        },
                        "RedirectIPv4AddressSpecificExtended",
                    )]
                },
                "ExtendedCommunitiesAttribute"
            ),
            to_any(
                MpReachNlriAttribute {
                    family: Some(Family {
                        afi: family::Afi::Ip as i32,
                        safi: family::Safi::FlowSpecUnicast as i32,
                    }),
                    next_hops: prost::alloc::vec![],
                    nlris: prost::alloc::vec![nlri],
                },
                "MpReachNLRIAttribute"
            ),
            to_any(OriginAttribute { origin: 2 }, "OriginAttribute")
        ],
        age: Some(SystemTime::now().to_api()),
        ..Path::default()
    };

    let request = Request::new(AddPathRequest {
        table_type: TableType::Global as i32,
        path: Some(path),
        vrf_id: String::new(),
    });

    let stream_response = match client.add_path(request).await {
        Ok(response) => response.into_inner(),
        Err(err) => panic!("ERROR = {:#?}", err),
    };

    println!("NOTE = {:#?}", stream_response);

    Ok(())
}
