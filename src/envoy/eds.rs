use serde::Serialize;
use std::collections::HashMap;
use crate::consul::services::Service;

#[derive(Debug, Serialize)]
pub struct EnvoyEndpoint {
    pub cluster_name: String,
    pub endpoints: Vec<ClusterLoadAssignment>,
}

#[derive(Debug, Serialize)]
pub struct ClusterLoadAssignment {
    pub cluster_name: String,
    pub endpoints: Vec<LocalityLbEndpoints>,
}

#[derive(Debug, Serialize)]
pub struct LocalityLbEndpoints {
    pub lb_endpoints: Vec<LbEndpoint>,
}

#[derive(Debug, Serialize)]
pub struct LbEndpoint {
    pub endpoint: Endpoint,
}

#[derive(Debug, Serialize)]
pub struct Endpoint {
    pub address: Address,
}

#[derive(Debug, Serialize)]
pub struct Address {
    pub socket_address: SocketAddress,
}

#[derive(Debug, Serialize)]
pub struct SocketAddress {
    pub address: String,
    pub port_value: u16,
}

pub async fn transform_consul_to_eds(consul_services: Vec<Service>) -> Result<EnvoyEndpoint, serde_json::Error> {
    // Group services by name
    let mut service_groups: HashMap<String, Vec<&Service>> = HashMap::new();
    for service in &consul_services {
        service_groups
            .entry(format!("cluster_{}", service.name))
            .or_default()
            .push(service);
    }

    // Create EDS response for the first service group
    // (assuming we're dealing with one service type at a time)
    let service_name = format!("cluster_{}", consul_services[0].name);

    let lb_endpoints: Vec<LbEndpoint> = consul_services
        .iter()
        .map(|service| LbEndpoint {
            endpoint: Endpoint {
                address: Address {
                    socket_address: SocketAddress {
                        address: service.address.clone(),
                        port_value: service.port,
                    },
                },
            },
        })
        .collect();

    let locality_lb_endpoints = LocalityLbEndpoints { lb_endpoints };

    let cluster_load_assignment = ClusterLoadAssignment {
        cluster_name: service_name.clone(),
        endpoints: vec![locality_lb_endpoints],
    };

    Ok(EnvoyEndpoint {
        cluster_name: service_name,
        endpoints: vec![cluster_load_assignment],
    })
}
