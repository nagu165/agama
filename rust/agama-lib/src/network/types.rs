use serde::{Deserialize, Serialize};
use std::{fmt, str};
use thiserror::Error;
use zbus;

/// Network device
#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub type_: DeviceType,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SSID(pub Vec<u8>);

impl SSID {
    pub fn to_vec(&self) -> &Vec<u8> {
        &self.0
    }
}

impl fmt::Display for SSID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

impl From<SSID> for Vec<u8> {
    fn from(value: SSID) -> Self {
        value.0
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Loopback = 0,
    Ethernet = 1,
    Wireless = 2,
    Dummy = 3,
    Bond = 4,
}

/// Bond mode
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BondMode {
    #[serde(rename = "balance-rr")]
    RoundRobin = 0,
    #[serde(rename = "active-backup")]
    ActiveBackup = 1,
    #[serde(rename = "balance-xor")]
    BalanceXOR = 2,
    #[serde(rename = "broadcast")]
    Broadcast = 3,
    #[serde(rename = "802.3ad")]
    LACP = 4,
    #[serde(rename = "balance-tlb")]
    BalanceTLB = 5,
    #[serde(rename = "balance-alb")]
    BalanceALB = 6,
}
impl Default for BondMode {
    fn default() -> Self {
        Self::RoundRobin
    }
}

impl std::fmt::Display for BondMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BondMode::RoundRobin => "balance-rr",
                BondMode::ActiveBackup => "active-backup",
                BondMode::BalanceXOR => "balance-xor",
                BondMode::Broadcast => "broadcast",
                BondMode::LACP => "802.3ad",
                BondMode::BalanceTLB => "balance-tlb",
                BondMode::BalanceALB => "balance-alb",
            }
        )
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("Invalid bond mode: {0}")]
pub struct InvalidBondMode(String);

impl TryFrom<&str> for BondMode {
    type Error = InvalidBondMode;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "balance-rr" => Ok(BondMode::RoundRobin),
            "active-backup" => Ok(BondMode::ActiveBackup),
            "balance-xor" => Ok(BondMode::BalanceXOR),
            "broadcast" => Ok(BondMode::Broadcast),
            "802.3ad" => Ok(BondMode::LACP),
            "balance-tlb" => Ok(BondMode::BalanceTLB),
            "balance-alb" => Ok(BondMode::BalanceALB),
            _ => Err(InvalidBondMode(value.to_string())),
        }
    }
}
impl TryFrom<u8> for BondMode {
    type Error = InvalidBondMode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BondMode::RoundRobin),
            1 => Ok(BondMode::ActiveBackup),
            2 => Ok(BondMode::BalanceXOR),
            3 => Ok(BondMode::Broadcast),
            4 => Ok(BondMode::LACP),
            5 => Ok(BondMode::BalanceTLB),
            6 => Ok(BondMode::BalanceALB),
            _ => Err(InvalidBondMode(value.to_string())),
        }
    }
}

impl From<InvalidBondMode> for zbus::fdo::Error {
    fn from(value: InvalidBondMode) -> zbus::fdo::Error {
        zbus::fdo::Error::Failed(format!("Network error: {value}"))
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("Invalid device type: {0}")]
pub struct InvalidDeviceType(u8);

impl TryFrom<u8> for DeviceType {
    type Error = InvalidDeviceType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceType::Loopback),
            1 => Ok(DeviceType::Ethernet),
            2 => Ok(DeviceType::Wireless),
            3 => Ok(DeviceType::Dummy),
            4 => Ok(DeviceType::Bond),
            _ => Err(InvalidDeviceType(value)),
        }
    }
}

impl From<InvalidDeviceType> for zbus::fdo::Error {
    fn from(value: InvalidDeviceType) -> zbus::fdo::Error {
        zbus::fdo::Error::Failed(format!("Network error: {value}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_ssid() {
        let ssid = SSID(vec![97, 103, 97, 109, 97]);
        assert_eq!(format!("{}", ssid), "agama");
    }

    #[test]
    fn test_ssid_to_vec() {
        let vec = vec![97, 103, 97, 109, 97];
        let ssid = SSID(vec.clone());
        assert_eq!(ssid.to_vec(), &vec);
    }

    #[test]
    fn test_device_type_from_u8() {
        let dtype = DeviceType::try_from(0);
        assert_eq!(dtype, Ok(DeviceType::Loopback));

        let dtype = DeviceType::try_from(128);
        assert_eq!(dtype, Err(InvalidDeviceType(128)));
    }

    #[test]
    fn test_display_bond_mode() {
        let mode = BondMode::try_from(1).unwrap();
        assert_eq!(format!("{}", mode), "active-backup");
    }
}
