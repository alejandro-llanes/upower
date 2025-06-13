use properties::{BatteryLevel, CapacityLevel, DeviceType, State, Technology, WarningLevel};
use serde::{Deserialize, Serialize};
use zbus::{
    Connection, proxy,
    zvariant::{OwnedObjectPath, Type},
};
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

    #[zbus(property)]
    fn type_(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn power_supply(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn has_history(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn has_statistics(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    #[zbus(name = "Online")]
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
    fn tecnology(&self) -> zbus::Result<u32>;

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

//
// https://upower.freedesktop.org/docs/
// https://upower.freedesktop.org/docs/ref-dbus.html
// https://upower.freedesktop.org/docs/Device.html
// https://gitlab.freedesktop.org/upower
#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
//#[serde(rename_all = "camelCase")]
pub struct UPowerDevice {
    #[serde(rename = "NativePath")]
    native_path: String,
    #[zvariant(rename = "Vendor")]
    vendor: String,
    #[zvariant(rename = "Model")]
    model: String,
    #[zvariant(rename = "Serial")]
    serial: String,
    #[zvariant(rename = "UpdateTime")]
    update_time: u64,
    #[zvariant(rename = "Type")]
    type_: properties::DeviceType,
    #[zvariant(rename = "PowerSupply")]
    power_supply: bool,
    #[zvariant(rename = "HasHistory")]
    has_history: bool,
    #[zvariant(rename = "HasStatistics")]
    has_statistics: bool,
    #[zvariant(rename = "Online")]
    on_line: bool,
    #[zvariant(rename = "Energy")]
    energy: f64,
    #[zvariant(rename = "EnergyEmpty")]
    energy_empty: f64,
    #[zvariant(rename = "EnergyFull")]
    energy_full: f64,
    #[zvariant(rename = "EnergyFullDesign")]
    energy_full_design: f64,
    #[zvariant(rename = "EnergyRate")]
    energy_rate: f64,
    #[zvariant(rename = "ChargeCycles")]
    charge_cycles: i32,
    #[zvariant(rename = "Luminosity")]
    luminosity: f64,
    #[zvariant(rename = "Voltage")]
    voltage: f64,
    #[zvariant(rename = "TimeToEmpty")]
    time_to_empty: i64,
    #[zvariant(rename = "TimeToFull")]
    time_to_full: i64,
    #[zvariant(rename = "Percentage")]
    percentage: f64,
    #[zvariant(rename = "Temperature")]
    temperature: f64,
    #[zvariant(rename = "IsPresent")]
    is_present: bool,
    #[zvariant(rename = "State")]
    state: properties::State,
    #[zvariant(rename = "IsRechargeable")]
    is_rechargeable: bool,
    #[zvariant(rename = "Capacity")]
    capacity: f64,
    #[zvariant(rename = "Technology")]
    technology: properties::Technology,
    #[zvariant(rename = "WarningLevel")]
    warning_level: properties::WarningLevel,
    #[zvariant(rename = "BatteryLevel")]
    battery_level: properties::BatteryLevel,
    #[zvariant(rename = "IconName")]
    icon_name: String,
    #[zvariant(rename = "ChargeStartThreshold")]
    charge_start_threshold: u32, // betweeen 1 and 100
    #[zvariant(rename = "ChargeEndThreshold")]
    charge_end_threshold: u32, // betweeen 1 and 100
    #[zvariant(rename = "ChargeThresholdEnabled")]
    charge_threshold_enabled: bool,
    #[zvariant(rename = "ChargeThresholdSupported")]
    charge_threshold_supported: bool,
    #[zvariant(rename = "VoltageMinDesign")]
    voltage_min_design: f64,
    #[zvariant(rename = "VoltageMaxDesign")]
    voltage_max_design: f64,
    #[zvariant(rename = "CapacityLevel")]
    capacity_level: properties::CapacityLevel,
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
        let native_path = value.native_path().await?;
        let vendor = value.vendor().await?;
        let model = value.model().await?;
        let serial = value.serial().await?;
        let update_time = value.update_time().await?;
        let type_ = DeviceType::from(value.type_().await?);
        let power_supply = value.power_supply().await?;
        let has_history = value.has_history().await?;
        let has_statistics = value.has_statistics().await?;
        let on_line = value.on_line().await?;
        let energy = value.energy().await?;
        let energy_empty = value.energy_empty().await?;
        let energy_full = value.energy_full().await?;
        let energy_full_design = value.energy_full_design().await?;
        let energy_rate = value.energy_rate().await?;
        let charge_cycles = value.charge_cycles().await?;
        let luminosity = value.luminosity().await?;
        let voltage = value.voltage().await?;
        let time_to_empty = value.time_to_empty().await?;
        let time_to_full = value.time_to_full().await?;
        let percentage = value.percentage().await?;
        let temperature = value.temperature().await?;
        let is_present = value.is_present().await?;
        let state = State::from(value.state().await?);
        let is_rechargeable = value.is_rechargeable().await?;
        let capacity = value.capacity().await?;
        let technology = Technology::from(value.tecnology().await?);
        let warning_level = WarningLevel::from(value.warning_level().await?);
        let battery_level = BatteryLevel::from(value.battery_level().await?);
        let icon_name = value.icon_name().await?;
        let charge_start_threshold = value.charge_start_threshold().await?;
        let charge_end_threshold = value.charge_end_threshold().await?;
        let charge_threshold_enabled = value.charge_threshold_enabled().await?;
        let charge_threshold_supported = value.charge_threshold_supported().await?;
        let voltage_min_design = value.voltage_min_design().await?;
        let voltage_max_design = value.voltage_max_design().await?;
        let capacity_level = CapacityLevel::from(value.capacity_level().await?);
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

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct BoundedU32(u32);

impl<'de> Deserialize<'de> for BoundedU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = u32::deserialize(deserializer)?;
        if (0..=100).contains(&val) {
            Ok(BoundedU32(val))
        } else {
            Err(serde::de::Error::custom(format!(
                "Value {val} is out of bounds (0-100)."
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
        //assert_eq!(UPower::new().await, Result::Ok)
        let upower = UPower::new().await;
        println!("{upower:?}");
        assert!(upower.is_ok())
    }
}
