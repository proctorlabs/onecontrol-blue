use bluer::AdapterEvent;
use futures::{pin_mut, StreamExt};

pub async fn scan(device_name: &str) -> bluer::Result<()> {
    let target_device = Some(device_name.to_string());
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    info!(
        "Discovering devices using Bluetooth adapter {}",
        adapter.name()
    );
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices().await?;
    pin_mut!(device_events);

    loop {
        if let Some(device_event) = device_events.next().await {
            match device_event {
                AdapterEvent::DeviceAdded(addr) => {
                    let device = adapter.device(addr)?;
                    if target_device == device.name().await? {
                        info!("Found device at {}", addr);
                        return Ok(());
                    }
                }
                _ => (),
            }
        }
    }
}
