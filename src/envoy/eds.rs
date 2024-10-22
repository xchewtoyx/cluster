use serde::Serialize;
use std::collections::HashMap;
use crate::consul::api::Service;

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

pub async fn transform_consul_to_eds(consul_services: Vec<Service>) -> Result<String, serde_json::Error> {
    // Group services by name
    let mut service_groups: HashMap<String, Vec<&Service>> = HashMap::new();
    for service in &consul_services {
        service_groups
            .entry(service.name.clone())
            .or_default()
            .push(service);
    }

    // Create EDS response for the first service group
    // (assuming we're dealing with one service type at a time)
    let service_name = consul_services[0].name.clone();
    
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

    let endpoint = EnvoyEndpoint {
        cluster_name: service_name,
        endpoints: vec![cluster_load_assignment],
    };
    serde_json::to_string(&endpoint)
}
