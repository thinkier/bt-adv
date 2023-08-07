use bluez::management::*;
use enumflags2::make_bitflags;

const BEACON_NAME: &str = "thinkier BlueTooth Beacon";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mgmt = ManagementStream::open()?;
    let ctr = Controller::none();
    let mut adv_data = vec![BEACON_NAME.len() as u8 + 1, 0x09];
    adv_data.extend(BEACON_NAME.bytes());
    let info = AdvertisingParams {
        instance: 1,
        flags: make_bitflags!(AdvertisingFlags::{AdvertiseLimitedDiscoverable}),
        duration: 1,
        timeout: 30,
        adv_data,
        scan_rsp: vec![],
    };

    let byte = add_advertising(&mut mgmt, ctr, info, None).await?;

    Ok(())
}
