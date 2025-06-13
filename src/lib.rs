use properties::{BatteryLevel, CapacityLevel, DeviceType, State, Technology, WarningLevel};
use serde::{Deserialize, Serialize};
use zbus::{Connection, proxy, zvariant::OwnedObjectPath};

// https://upower.freedesktop.org/docs/
// https://upower.freedesktop.org/docs/ref-dbus.html
// https://upower.freedesktop.org/docs/Device.html
// https://gitlab.freedesktop.org/upower

mod properties;

const UPOWER_DESTINATION: &str = "org.freedesktop.UPower";
const UPOWER_PATH: &str = "/org/freedesktop/UPower";
const UPOWER_INTERFACE: &str = "org.freedesktop.UPower";
const UPOWER_DEVICE_INTERFACE: &str = "org.freedesktop.UPower.Device";

#[proxy(gen_async = true)]
trait UPower {
    fn enumerate_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[proxy(gen_async = true)]
trait UPowerDevice {
    #[zbus(property)]
    fn native_path(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn vendor(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn serial(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn update_time(&self) -> zbus::Result<u64>;

    #[zbus(name = "Type", property)]
    fn type_(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn power_supply(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn has_history(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn has_statistics(&self) -> zbus::Result<bool>;

    #[zbus(name = "Online", property)]
    fn on_line(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn energy(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_empty(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_full(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_full_design(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_rate(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn charge_cycles(&self) -> zbus::Result<i32>;

    #[zbus(property)]
    fn luminosity(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn voltage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn time_to_empty(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn time_to_full(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn temperature(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn is_present(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn is_rechargeable(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn capacity(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn technology(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn warning_level(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn battery_level(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn charge_start_threshold(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn charge_end_threshold(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn charge_threshold_enabled(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn charge_threshold_supported(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn voltage_min_design(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn voltage_max_design(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn capacity_level(&self) -> zbus::Result<String>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UPowerDevice {
    native_path: Option<String>,
    vendor: Option<String>,
    model: Option<String>,
    serial: Option<String>,
    update_time: Option<u64>,
    type_: Option<properties::DeviceType>,
    power_supply: Option<bool>,
    has_history: Option<bool>,
    has_statistics: Option<bool>,
    on_line: Option<bool>,
    energy: Option<f64>,
    energy_empty: Option<f64>,
    energy_full: Option<f64>,
    energy_full_design: Option<f64>,
    energy_rate: Option<f64>,
    charge_cycles: Option<i32>,
    luminosity: Option<f64>,
    voltage: Option<f64>,
    time_to_empty: Option<i64>,
    time_to_full: Option<i64>,
    percentage: Option<f64>,
    temperature: Option<f64>,
    is_present: Option<bool>,
    state: Option<properties::State>,
    is_rechargeable: Option<bool>,
    capacity: Option<f64>,
    technology: Option<properties::Technology>,
    warning_level: Option<properties::WarningLevel>,
    battery_level: Option<properties::BatteryLevel>,
    icon_name: Option<String>,
    charge_start_threshold: Option<BoundedU32>, // betweeen 1 and 100
    charge_end_threshold: Option<BoundedU32>,   // betweeen 1 and 100
    charge_threshold_enabled: Option<bool>,
    charge_threshold_supported: Option<bool>,
    voltage_min_design: Option<f64>,
    voltage_max_design: Option<f64>,
    capacity_level: Option<properties::CapacityLevel>,
}

pub trait AsyncTryFrom<T>
where
    Self: Sized + Sync + Send,
{
    type Error;
    fn as_try_from(value: T) -> impl std::future::Future<Output = Result<Self, Self::Error>>;
}

impl AsyncTryFrom<UPowerDeviceProxy<'_>> for UPowerDevice {
    type Error = zbus::Error;
    async fn as_try_from(value: UPowerDeviceProxy<'_>) -> Result<Self, Self::Error> {
        let native_path = value.native_path().await.ok();
        let vendor = value.vendor().await.ok();
        let model = value.model().await.ok();
        let serial = value.serial().await.ok();
        let update_time = value.update_time().await.ok();
        let type_ = value
            .type_()
            .await
            .map_or(None, |val| Some(DeviceType::from(val)));
        let power_supply = value.power_supply().await.ok();
        let has_history = value.has_history().await.ok();
        let has_statistics = value.has_statistics().await.ok();
        let on_line = value.on_line().await.ok();
        let energy = value.energy().await.ok();
        let energy_empty = value.energy_empty().await.ok();
        let energy_full = value.energy_full().await.ok();
        let energy_full_design = value.energy_full_design().await.ok();
        let energy_rate = value.energy_rate().await.ok();
        let charge_cycles = value.charge_cycles().await.ok();
        let luminosity = value.luminosity().await.ok();
        let voltage = value.voltage().await.ok();
        let time_to_empty = value.time_to_empty().await.ok();
        let time_to_full = value.time_to_full().await.ok();
        let percentage = value.percentage().await.ok();
        let temperature = value.temperature().await.ok();
        let is_present = value.is_present().await.ok();
        let state = value
            .state()
            .await
            .map_or(None, |val| Some(State::from(val)));
        let is_rechargeable = value.is_rechargeable().await.ok();
        let capacity = value.capacity().await.ok();
        let technology = value
            .technology()
            .await
            .map_or(None, |val| Some(Technology::from(val)));
        let warning_level = value
            .warning_level()
            .await
            .map_or(None, |val| Some(WarningLevel::from(val)));
        let battery_level = value
            .battery_level()
            .await
            .map_or(None, |val| Some(BatteryLevel::from(val)));
        let icon_name = value.icon_name().await.ok();
        let charge_start_threshold = value
            .charge_start_threshold()
            .await
            .map_or(None, |val| BoundedU32::try_from(val).ok());
        let charge_end_threshold = value
            .charge_end_threshold()
            .await
            .map_or(None, |val| BoundedU32::try_from(val).ok());
        let charge_threshold_enabled = value.charge_threshold_enabled().await.ok();
        let charge_threshold_supported = value.charge_threshold_supported().await.ok();
        let voltage_min_design = value.voltage_min_design().await.ok();
        let voltage_max_design = value.voltage_max_design().await.ok();
        let capacity_level = value
            .capacity_level()
            .await
            .map_or(None, |val| Some(CapacityLevel::from(val)));

        Ok(Self {
            native_path,
            vendor,
            model,
            serial,
            update_time,
            type_,
            power_supply,
            has_history,
            has_statistics,
            on_line,
            energy,
            energy_empty,
            energy_full,
            energy_full_design,
            energy_rate,
            charge_cycles,
            luminosity,
            voltage,
            time_to_empty,
            time_to_full,
            percentage,
            temperature,
            is_present,
            state,
            is_rechargeable,
            capacity,
            technology,
            warning_level,
            battery_level,
            icon_name,
            charge_start_threshold,
            charge_end_threshold,
            charge_threshold_enabled,
            charge_threshold_supported,
            voltage_min_design,
            voltage_max_design,
            capacity_level,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundedU32(u32);

impl TryFrom<u32> for BoundedU32 {
    type Error = zbus::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&value) {
            Ok(BoundedU32(value))
        } else {
            Err(zbus::Error::Failure(format!(
                "Value {value} is out of bounds (0-100)."
            )))
        }
    }
}

impl std::fmt::Display for BoundedU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct UPower {
    devices: Vec<UPowerDevice>,
}

impl UPower {
    pub async fn new() -> zbus::Result<Self> {
        let conn = Connection::system().await?;

        let mut devices: Vec<UPowerDevice> = Vec::default();

        let upower_proxy = UPowerProxy::builder(&conn)
            .destination(UPOWER_DESTINATION)?
            .interface(UPOWER_INTERFACE)?
            .path(UPOWER_PATH)?
            .build()
            .await?;

        let devices_list = upower_proxy.enumerate_devices().await?;

        for device_path in devices_list {
            let path_str = device_path.as_str();

            let dev_proxy = UPowerDeviceProxy::builder(&conn)
                .destination(UPOWER_DESTINATION)?
                .interface(UPOWER_DEVICE_INTERFACE)?
                .path(path_str)?
                .build()
                .await?;

            let device = UPowerDevice::as_try_from(dev_proxy).await?;
            devices.push(device);
        }
        Ok(Self { devices })
    }

    pub fn devices(&self) -> &Vec<UPowerDevice> {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn it_works() {
        let upower = UPower::new().await;
        //println!("{upower:?}");
        assert!(upower.is_ok())
    }
}
