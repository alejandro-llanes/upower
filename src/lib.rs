use serde::{Deserialize, Serialize};
use zbus::{
    Connection, Proxy, ProxyBuilder, fdo::PropertiesProxy, names::InterfaceName,
    zvariant::OwnedValue,
};
use zvariant::{ObjectPath, OwnedObjectPath, Type, Value, serialized::Context, to_bytes};
mod properties;

const UPOWER_DEST: &str = "org.freedesktop.UPower";
const UPOWER_PATH: &str = "/org/freedesktop/UPower";
const UPOWER_INTERFACE: &str = "org.freedesktop.UPower";
const UPOWER_DEVICES_DEST: &str = "org.freedesktop.UPower.Device";

//
// https://upower.freedesktop.org/docs/
// https://upower.freedesktop.org/docs/ref-dbus.html
// https://upower.freedesktop.org/docs/Device.html
// https://gitlab.freedesktop.org/upower
#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct UPowerDevice {
    #[zvariant(rename = "NativePath")]
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
    power_suply: bool,
    #[zvariant(rename = "HasHistory")]
    has_history: bool,
    #[zvariant(rename = "HasStatistics")]
    has_statistics: bool,
    #[zvariant(rename = "OnLine")]
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
        println!("Line 127");
        let mut devices: Vec<UPowerDevice> = Vec::default();
        println!("Line 129");
        let upower_proxy: Proxy = ProxyBuilder::new_bare(&conn)
            .destination(UPOWER_DEST)?
            .interface(UPOWER_INTERFACE)?
            .path(UPOWER_PATH)?
            .build()
            .await?;
        println!("Line 141");
        let devices_list: Vec<ObjectPath<'_>> = upower_proxy.call("EnumerateDevices", &()).await?;
        println!("Line 143");
        for device_path in devices_list {
            let path_str = device_path.as_str();
            println!("Line 146");
            let prop_proxy = PropertiesProxy::builder(&conn)
                .destination(UPOWER_DEST)?
                .path(path_str)?
                .build()
                .await?;
            println!("Line 152");
            let props: std::collections::HashMap<String, OwnedValue> = prop_proxy
                .get_all(InterfaceName::from_static_str(UPOWER_DEVICES_DEST)?)
                .await?;
            println!("Line 156");
            let json_values: serde_json::Value = serde_json::to_value(&props).unwrap();
            println!("Line 158");
            let device: UPowerDevice = serde_json::from_value(json_values).unwrap();
            println!("Line 160");
            devices.push(device);
        }
        println!("Line 163");
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
