use hap::{
    accessory::{outlet::OutletAccessory, AccessoryCategory, AccessoryInformation},
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    tokio,
    Config,
    MacAddress,
    Pin,
};

#[tokio::main]
async fn main() {
    let outlet = OutletAccessory::new(1, AccessoryInformation {
        name: "Acme Outlet".into(),
        ..Default::default()
    })
    .unwrap();

    let mut storage = FileStorage::current_dir().await.unwrap();

    let config = match storage.load_config().await {
        Ok(config) => config,
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
                name: "Acme Outlet".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Outlet,
                ..Default::default()
            };
            storage.save_config(&config).await.unwrap();
            config
        },
    };

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(outlet).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=info");
    env_logger::init();

    handle.await;
}
