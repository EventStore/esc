use esc_api::{Cluster, Topology};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ClusterAddresses {
    tcp: Vec<String>,
    grpc: String,
    ui: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EnrichedCluster {
    #[serde(flatten)]
    cluster: Cluster,
    addresses: ClusterAddresses,
}

pub fn enrich_cluster(cluster: Cluster) -> EnrichedCluster {
    let mut tcp = Vec::new();
    let grpc;
    let ui = format!("https://{}.mesdb.eventstore.cloud:2113", cluster.id.0);

    if let Topology::ThreeNodeMultiZone = cluster.topology {
        for idx in 0..3 {
            tcp.push(format!(
                "{}-{}.mesdb.eventstore.cloud:1113",
                cluster.id.0, idx
            ));
        }

        grpc = format!(
            "esdb+discover://{}.mesdb.eventstore.cloud:2113",
            cluster.id.0
        );
    } else {
        tcp.push(format!("{}.mesdb.eventstore.cloud:1113", cluster.id.0));
        grpc = format!("esdb://{}.mesdb.eventstore.cloud:2113", cluster.id.0);
    }

    EnrichedCluster {
        cluster,
        addresses: ClusterAddresses { tcp, grpc, ui },
    }
}
