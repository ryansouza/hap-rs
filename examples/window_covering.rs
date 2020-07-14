use std::net::{IpAddr, SocketAddr};

use hap::{
    accessory::{window_covering::WindowCoveringAccessory, AccessoryCategory, AccessoryInformation},
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    tokio,
    Config,
    MacAddress,
    Pin,
};

#[tokio::main]
async fn main() {
    let current_ipv4 = || -> Option<IpAddr> {
        for iface in pnet::datalink::interfaces() {
            for ip_network in iface.ips {
                if ip_network.is_ipv4() {
                    let ip = ip_network.ip();
                    if !ip.is_loopback() {
                        return Some(ip);
                    }
                }
            }
        }
        None
    };

    let window_covering = WindowCoveringAccessory::new(1, AccessoryInformation {
        name: "Acme Window Covering".into(),
        ..Default::default()
    })
    .unwrap();

    let mut storage = FileStorage::current_dir().await.unwrap();

    let config = match storage.load_config().await {
        Ok(config) => config,
        Err(_) => {
            let config = Config {
                socket_addr: SocketAddr::new(current_ipv4().unwrap(), 32000),
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
                name: "Acme Window Covering".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::WindowCovering,
                ..Default::default()
            };
            storage.save_config(&config).await.unwrap();
            config
        },
    };

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(window_covering).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=info");
    env_logger::init();

    handle.await;
}
