use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(from = "u32")]
pub enum Type {
    Unknown,
    LinePower,
    Battery,
    Ups,
    Monitor,
    Mouse,
    Keyboard,
    Pda,
    Phone,
    MediaPlayer,
    Tablet,
    Computer,
    GamingInput,
    Pen,
    Touchpad,
    Modem,
    Network,
    Headset,
    Speakers,
    Headphones,
    Video,
    OtherAudio,
    RemoteControl,
    Printer,
    Scanner,
    Camera,
    Wearable,
    Toy,
    BluetoothGeneric,
}

impl From<u32> for Type {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::LinePower,
            2 => Self::Battery,
            3 => Self::Ups,
            4 => Self::Monitor,
            5 => Self::Mouse,
            6 => Self::Keyboard,
            7 => Self::Pda,
            8 => Self::Phone,
            9 => Self::MediaPlayer,
            10 => Self::Tablet,
            11 => Self::Computer,
            12 => Self::GamingInput,
            13 => Self::Pen,
            14 => Self::Touchpad,
            15 => Self::Modem,
            16 => Self::Network,
            17 => Self::Headset,
            18 => Self::Speakers,
            19 => Self::Headphones,
            20 => Self::Video,
            21 => Self::OtherAudio,
            22 => Self::RemoteControl,
            23 => Self::Printer,
            24 => Self::Scanner,
            25 => Self::Camera,
            26 => Self::Wearable,
            27 => Self::Toy,
            27 => Self::BluetoothGeneric,
            _ => Self::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(from = "u32")]
pub enum State {
    Unknown,
    Charging,
    Discharging,
    Empty,
    FullyCharged,
    PendingCharge,
    PendingDischarge,
}

impl From<u32> for State {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Charging,
            2 => Self::DisCharging,
            3 => Self::Empty,
            4 => Self::FullyCharged,
            5 => Self::PendingCharge,
            6 => Self::PendingDischarge,
            _ => Self::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(from = "u32")]
pub enum Technology {
    Unknown,
    LithiumIon,
    LithiumPolymer,
    LithiumIronPhosphate,
    LeadAcid,
    NickelCadmium,
    NickelMetalHydride,
}

impl From<u32> for Technology {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::LithiumIon,
            2 => Self::LithiumPolymer,
            3 => Self::LithiumIronPhosphate,
            4 => Self::LeadAcid,
            5 => Self::NickelCadmium,
            6 => Self::NickelMetalHydride,
            _ => Self::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(from = "u32")]
pub enum WarningLevel {
    Unknown,
    None,
    Discharging,
    Low,
    Critical,
    Action,
}

impl From<u32> for WarningLevel {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::None,
            // only valid for UPSes
            2 => Self::Discharging,
            3 => Self::Low,
            4 => Self::Critical,
            5 => Self::Action,
            _ => Self::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(from = "u32")]
pub enum BatteryLevel {
    Unknown,
    None,
    Low,
    Critical,
    Normal,
    High,
    Full,
}

impl From<u32> for BatteryLevel {
    fn from(value: u32) -> Self {
        match value {
            // the battery does not use a coarse level of battery reporting
            1 => Self::None,
            3 => Self::Low,
            4 => Self::Critical,
            6 => Self::Normal,
            7 => Self::High,
            8 => Self::Full,
            _ => Self::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(from = "String")]
pub enum CapacityLevel {
    Unknown,
    Critical,
    Low,
    Normal,
    High,
    Full,
}

impl From<String> for CapacityLevel {
    fn from(value: String) -> Self {
        match *value.to_lowercase() {
            "critical" => Self::Critical,
            "low" => Self::Low,
            "normal" => Self::Normal,
            "high" => Self::High,
            "full" => Self::Full,
            _ => Self::Unknown,
        }
    }
}
