use warp;
use cluster::api::routes::routes;

pub async fn serve(bindaddr: &str) {
    let bind_parts: Vec<&str> = bindaddr.split(':').collect();
    if bind_parts.len() != 2 {
        eprintln!("Invalid bind address format. Expected format: IP:PORT");
        std::process::exit(1);
    }

    let bind_ip: std::net::IpAddr = match bind_parts[0].parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address format");
            std::process::exit(1);
        }
    };

    let bind_port = match bind_parts[1].parse() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port format");
            std::process::exit(1);
        }
    };

    warp::serve(routes()).run((bind_ip, bind_port)).await;
}