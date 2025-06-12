use serde::{Deserialize, Serialize};
use zbus::zvariant;
use zvariant::{OwnedObjectPath, Value};
mod properties;

//
// https://upower.freedesktop.org/docs/
// https://upower.freedesktop.org/docs/ref-dbus.html
// https://upower.freedesktop.org/docs/Device.html
// https://gitlab.freedesktop.org/upower
#[derive(Serialize, Deserialize)]
pub struct UPowerDevice {
    #[zvariant(rename = "NativePath")]
    native_path: Option<String>,
    #[zvariant(rename = "Vendor")]
    vendor: Option<String>,
    #[zvariant(rename = "Model")]
    model: Option<String>,
    #[zvariant(rename = "Serial")]
    serial: Option<String>,
    #[zvariant(rename = "UpdateTime")]
    update_time: Option<u64>,
    #[zvariant(rename = "Type")]
    type_: Option<properties::Type>,
    #[zvariant(rename = "PowerSupply")]
    power_suply: Option<bool>,
    #[zvariant(rename = "HasHistory")]
    has_history: Option<bool>,
    #[zvariant(rename = "HasStatistics")]
    has_statistics: Option<bool>,
    #[zvariant(rename = "OnLine")]
    on_line: Option<bool>,
    #[zvariant(rename = "Energy")]
    energy: Option<f64>,
    #[zvariant(rename = "EnergyEmpty")]
    energy_empty: Option<f64>,
    #[zvariant(rename = "EnergyFull")]
    energy_full: Option<f64>,
    #[zvariant(rename = "EnergyFullDesign")]
    energy_full_design: Option<f64>,
    #[zvariant(rename = "EnergyRate")]
    energy_rate: Option<f64>,
    #[zvariant(rename = "ChargeCycles")]
    charge_cycles: Option<i32>,
    #[zvariant(rename = "Luminosity")]
    luminosity: Option<f64>,
    #[zvariant(rename = "Voltage")]
    voltage: Option<f64>,
    #[zvariant(rename = "TimeToEmpty")]
    time_to_empty: Option<i64>,
    #[zvariant(rename = "TimeToFull")]
    time_to_full: Option<i64>,
    #[zvariant(rename = "Percentage")]
    percentage: Option<f64>,
    #[zvariant(rename = "Temperature")]
    temperature: Option<f64>,
    #[zvariant(rename = "IsPresent")]
    is_present: Option<bool>,
    #[zvariant(rename = "State")]
    state: Option<properties::State>,
    #[zvariant(rename = "IsRechargeable")]
    is_rechargeable: Option<bool>,
    #[zvariant(rename = "Capacity")]
    capacity: Option<f64>,
    #[zvariant(rename = "Technology")]
    technology: Option<properties::Technology>,
    #[zvariant(rename = "WarningLevel")]
    warning_level: Option<properties::WarningLevel>,
    #[zvariant(rename = "BatteryLevel")]
    battery_level: Option<properties::BatteryLevel>,
    #[zvariant(rename = "IconName")]
    icon_name: Option<String>,
    #[zvariant(rename = "ChargeStartThreshold")]
    charge_start_threshold: Option<u32>, // betweeen 1 and 100
    #[zvariant(rename = "ChargeEndThreshold")]
    charge_end_threshold: Option<u32>, // betweeen 1 and 100
    #[zvariant(rename = "ChargeThresholdEnabled")]
    charge_threshold_enabled: Option<bool>,
    #[zvariant(rename = "ChargeThresholdSupported")]
    charge_threshold_supported: Option<bool>,
    #[zvariant(rename = "VoltageMinDesign")]
    voltage_min_design: Option<f64>,
    #[zvariant(rename = "VoltageMaxDesign")]
    voltage_max_design: Option<f64>,
    #[zvariant(rename = "CapacityLevel")]
    capacity_level: Option<properties::CapacityLevel>,
}

#[derive(Debug, PartialEq, Eq, Type, Serialize)]
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
            Err(zvariant::Error::Message(format!(
                "Value {val} is out of bounds (0-100)."
            )))
        }
    }
}

pub struct UPowerInfo {
    devices: Vec<UPowerDevice>,
}
