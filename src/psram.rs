#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - psram_configure"]
    pub phy_cfg_00: PHY_CFG_00,
    _reserved1: [u8; 0x0100],
    #[doc = "0x104 - phy_cfg_04"]
    pub phy_cfg_04: PHY_CFG_04,
    #[doc = "0x108 - phy_cfg_08"]
    pub phy_cfg_08: PHY_CFG_08,
    #[doc = "0x10c - phy_cfg_0C"]
    pub phy_cfg_10: PHY_CFG_10,
    _reserved4: [u8; 0x04],
    #[doc = "0x114 - phy_cfg_14"]
    pub phy_cfg_14: PHY_CFG_14,
    #[doc = "0x118 - phy_cfg_18"]
    pub phy_cfg_18: PHY_CFG_18,
    #[doc = "0x11c - phy_cfg_1C"]
    pub phy_cfg_20: PHY_CFG_20,
    _reserved7: [u8; 0x04],
    #[doc = "0x124 - phy_cfg_24"]
    pub phy_cfg_24: PHY_CFG_24,
    #[doc = "0x128 - phy_cfg_28"]
    pub phy_cfg_28: PHY_CFG_28,
    #[doc = "0x12c - phy_cfg_2C"]
    pub phy_cfg_30: PHY_CFG_30,
    _reserved10: [u8; 0x04],
    #[doc = "0x134 - phy_cfg_34"]
    pub phy_cfg_34: PHY_CFG_34,
    #[doc = "0x138 - phy_cfg_38"]
    pub phy_cfg_38: PHY_CFG_38,
    #[doc = "0x13c - phy_cfg_3C"]
    pub phy_cfg_40: PHY_CFG_40,
    _reserved13: [u8; 0x04],
    #[doc = "0x144 - phy_cfg_44"]
    pub phy_cfg_44: PHY_CFG_44,
    #[doc = "0x148 - phy_cfg_48"]
    pub phy_cfg_48: PHY_CFG_48,
}
#[doc = "phy_cfg_00 (rw) register accessor: an alias for `Reg<PHY_CFG_00_SPEC>`"]
pub type PHY_CFG_00 = crate::Reg<phy_cfg_00::PHY_CFG_00_SPEC>;
#[doc = "psram_configure"]
pub mod phy_cfg_00;
#[doc = "phy_cfg_04 (rw) register accessor: an alias for `Reg<PHY_CFG_04_SPEC>`"]
pub type PHY_CFG_04 = crate::Reg<phy_cfg_04::PHY_CFG_04_SPEC>;
#[doc = "phy_cfg_04"]
pub mod phy_cfg_04;
#[doc = "phy_cfg_08 (rw) register accessor: an alias for `Reg<PHY_CFG_08_SPEC>`"]
pub type PHY_CFG_08 = crate::Reg<phy_cfg_08::PHY_CFG_08_SPEC>;
#[doc = "phy_cfg_08"]
pub mod phy_cfg_08;
#[doc = "phy_cfg_10 (rw) register accessor: an alias for `Reg<PHY_CFG_10_SPEC>`"]
pub type PHY_CFG_10 = crate::Reg<phy_cfg_10::PHY_CFG_10_SPEC>;
#[doc = "phy_cfg_0C"]
pub mod phy_cfg_10;
#[doc = "phy_cfg_14 (rw) register accessor: an alias for `Reg<PHY_CFG_14_SPEC>`"]
pub type PHY_CFG_14 = crate::Reg<phy_cfg_14::PHY_CFG_14_SPEC>;
#[doc = "phy_cfg_14"]
pub mod phy_cfg_14;
#[doc = "phy_cfg_18 (rw) register accessor: an alias for `Reg<PHY_CFG_18_SPEC>`"]
pub type PHY_CFG_18 = crate::Reg<phy_cfg_18::PHY_CFG_18_SPEC>;
#[doc = "phy_cfg_18"]
pub mod phy_cfg_18;
#[doc = "phy_cfg_20 (rw) register accessor: an alias for `Reg<PHY_CFG_20_SPEC>`"]
pub type PHY_CFG_20 = crate::Reg<phy_cfg_20::PHY_CFG_20_SPEC>;
#[doc = "phy_cfg_1C"]
pub mod phy_cfg_20;
#[doc = "phy_cfg_24 (rw) register accessor: an alias for `Reg<PHY_CFG_24_SPEC>`"]
pub type PHY_CFG_24 = crate::Reg<phy_cfg_24::PHY_CFG_24_SPEC>;
#[doc = "phy_cfg_24"]
pub mod phy_cfg_24;
#[doc = "phy_cfg_28 (rw) register accessor: an alias for `Reg<PHY_CFG_28_SPEC>`"]
pub type PHY_CFG_28 = crate::Reg<phy_cfg_28::PHY_CFG_28_SPEC>;
#[doc = "phy_cfg_28"]
pub mod phy_cfg_28;
#[doc = "phy_cfg_30 (rw) register accessor: an alias for `Reg<PHY_CFG_30_SPEC>`"]
pub type PHY_CFG_30 = crate::Reg<phy_cfg_30::PHY_CFG_30_SPEC>;
#[doc = "phy_cfg_2C"]
pub mod phy_cfg_30;
#[doc = "phy_cfg_34 (rw) register accessor: an alias for `Reg<PHY_CFG_34_SPEC>`"]
pub type PHY_CFG_34 = crate::Reg<phy_cfg_34::PHY_CFG_34_SPEC>;
#[doc = "phy_cfg_34"]
pub mod phy_cfg_34;
#[doc = "phy_cfg_38 (rw) register accessor: an alias for `Reg<PHY_CFG_38_SPEC>`"]
pub type PHY_CFG_38 = crate::Reg<phy_cfg_38::PHY_CFG_38_SPEC>;
#[doc = "phy_cfg_38"]
pub mod phy_cfg_38;
#[doc = "phy_cfg_40 (rw) register accessor: an alias for `Reg<PHY_CFG_40_SPEC>`"]
pub type PHY_CFG_40 = crate::Reg<phy_cfg_40::PHY_CFG_40_SPEC>;
#[doc = "phy_cfg_3C"]
pub mod phy_cfg_40;
#[doc = "phy_cfg_44 (rw) register accessor: an alias for `Reg<PHY_CFG_44_SPEC>`"]
pub type PHY_CFG_44 = crate::Reg<phy_cfg_44::PHY_CFG_44_SPEC>;
#[doc = "phy_cfg_44"]
pub mod phy_cfg_44;
#[doc = "phy_cfg_48 (rw) register accessor: an alias for `Reg<PHY_CFG_48_SPEC>`"]
pub type PHY_CFG_48 = crate::Reg<phy_cfg_48::PHY_CFG_48_SPEC>;
#[doc = "phy_cfg_48"]
pub mod phy_cfg_48;
