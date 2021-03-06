use hap::{
    accessory::{lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    characteristic::CharacteristicCallbacks,
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    tokio,
    Config,
    MacAddress,
    Pin,
};

#[tokio::main]
async fn main() {
    let mut lightbulb = LightbulbAccessory::new(1, AccessoryInformation {
        name: "Acme Lightbulb".into(),
        ..Default::default()
    })
    .unwrap();

    lightbulb.lightbulb.on.on_read(Some(|| {
        println!("on characteristic read");
        None
    }));
    lightbulb
        .lightbulb
        .on
        .on_update(Some(|current_val: &bool, new_val: &bool| {
            println!("on characteristic updated from {} to {}", current_val, new_val);
        }));

    let mut storage = FileStorage::current_dir().await.unwrap();

    let config = match storage.load_config().await {
        Ok(config) => config,
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
                name: "Acme Lightbulb".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Lightbulb,
                ..Default::default()
            };
            storage.save_config(&config).await.unwrap();
            config
        },
    };

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(lightbulb).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=info");
    env_logger::init();

    handle.await;
}
