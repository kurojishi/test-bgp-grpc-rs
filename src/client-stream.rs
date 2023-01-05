pub mod gobgp {
    tonic::include_proto! {"apipb"}
}
use gobgp::gobgp_api_client::GobgpApiClient;
use gobgp::{
    family, AddPathStreamRequest, ExtendedCommunitiesAttribute, Family, FlowSpecIpPrefix,
    FlowSpecNlri, MpReachNlriAttribute, OriginAttribute, Path, RedirectIPv4AddressSpecificExtended,
    TableType,
};
use std::time::SystemTime;
use tonic::Request;
use std::time::Instant;

use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use ipnet::Ipv4Net;

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

fn create_path(ip: &str) -> Path {
    let nlri = to_any(
        FlowSpecNlri {
            rules: prost::alloc::vec![to_any(
                FlowSpecIpPrefix {
                    r#type: 2, //type 2 is source prefix
                    prefix: ip.to_string(),
                    prefix_len: 32,
                    offset: 0,
                },
                "FlowSpecIPPrefix"
            )],
        },
        "FlowSpecNLRI",
    );

    Path {
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
    }
}

fn generate_ip_set_from_file(
    ipfile: File,
) -> Result<Vec<AddPathStreamRequest>, Box<dyn std::error::Error>> {
    let mut requests = vec![];
    let lines = BufReader::new(ipfile).lines();
    println!("Mapping");
    for net in lines {
        let network = Ipv4Net::from_str(&(net?))?;
        let request = AddPathStreamRequest {
            table_type: TableType::Global as i32,
            paths: Vec::from_iter(network.hosts().map(|ip| create_path(&ip.to_string()))),
            vrf_id: String::new(),
        };
        requests.push(request);
    };

    Ok(requests)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GobgpApiClient::connect("http://127.0.0.1:50051").await?;

    let (tx, rx) = mpsc::channel(4);
    tokio::spawn(async move {
        let requests = generate_ip_set_from_file(File::open("freespace-prefix.txt").unwrap()).unwrap();
        for r in requests {
            tx.send(r).await.unwrap();
        }

    });

    let stream = ReceiverStream::new(rx);

    println!("Sending");
    let start = Instant::now();
    match client.add_path_stream(Request::new(stream)).await {
        Ok(response) => {
            response.into_inner()
        }
        Err(err) => panic!("ERROR = {:#?}", err),
    };
    println!("Time elapsed in sending messages is: {:?}", start.elapsed());
    Ok(())
}
