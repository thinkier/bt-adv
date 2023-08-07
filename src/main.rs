use bluez::management::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut mgmt = ManagementStream::open()?;
	let ctr = Controller::none();
	let info = AdvertisingParams {
		instance: 1,
		flags: AdvertisingFlags::AdvertiseLimitedDiscoverable,
		duration: 1,
		timeout: 30,
		adv_data: vec![],
		scan_rsp: vec![],
	};

	let byte = add_advertising(&mut mgmt, ctr, info, None).await?;

	Ok(())
}
