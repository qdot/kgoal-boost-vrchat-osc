
extern crate rosc;

use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::str::FromStr;
use tokio::net::{UdpSocket};
use rosc::{OscMessage, OscPacket, OscType, encoder};
use futures_util::StreamExt;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;

async fn find_boost(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Boost"))
        {
            return Some(p);
        }
    }
    None
}

#[tokio::main]
async fn main() {
    let SERVICE_UUID: Uuid = Uuid::from_str("8e7c6065-7656-17ad-1b41-b53d1a548e0d").unwrap();
    let CHAR_UUID: Uuid = Uuid::from_str("10c2be2d-d2d5-b7a8-5f42-e2468c9ebbf5").unwrap();
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let central = manager
        .adapters()
        .await
        .expect("Unable to fetch adapter list.")
        .into_iter()
        .nth(0)
        .expect("Unable to find adapters.");

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await.unwrap();
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // find the device we're interested in
    let boost = find_boost(&central).await.expect("No boost found");
    println!("Found device");
    // connect to the device
    boost.connect().await.unwrap();
    println!("Connected");
    // discover services and characteristics
    boost.discover_services().await.unwrap();
    println!("Got Services");
    // find the characteristic we want
    let chars = boost.characteristics();
    let cmd_char = chars
        .iter()
        .find(|c| c.uuid == CHAR_UUID)
        .expect("Unable to find characterics");
    println!("Got Chracteristic");

    let host_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25172);
    let socket = UdpSocket::bind(&host_addr).await.unwrap();
    let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/Squeeze".to_string(),
        args: vec![OscType::Float(0.99f32)],
    }))
    .unwrap();
    println!("Generated message.");

    boost.subscribe(cmd_char).await.unwrap();
    let mut stream = boost.notifications().await.unwrap();
    while let Some(event) = stream.next().await {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/avatar/parameters/Squeeze".to_string(),
            args: vec![OscType::Float((event.value[4] as f32 / 100f32).clamp(0.0, 0.99f32))],
        }))
        .unwrap();
        socket.send_to(&msg_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000)).await.unwrap();
        println!("{:?}", event);
    }


    
    println!("Message sent.");
}
