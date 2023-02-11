#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - mcu_bus_cfg0"]
    pub mcu_bus_cfg0: MCU_BUS_CFG0,
    #[doc = "0x04 - mcu_bus_cfg1"]
    pub mcu_bus_cfg1: MCU_BUS_CFG1,
    _reserved2: [u8; 0x0c],
    #[doc = "0x14 - mcu_e907_rtc"]
    pub mcu_e907_rtc: MCU_E907_RTC,
    _reserved3: [u8; 0xe8],
    #[doc = "0x100 - mcu_cfg1"]
    pub mcu_cfg1: MCU_CFG1,
    _reserved4: [u8; 0x0c],
    #[doc = "0x110 - mcu1_log1"]
    pub mcu1_log1: MCU1_LOG1,
    #[doc = "0x114 - mcu1_log2"]
    pub mcu1_log2: MCU1_LOG2,
    #[doc = "0x118 - mcu1_log3"]
    pub mcu1_log3: MCU1_LOG3,
    #[doc = "0x11c - mcu1_log4"]
    pub mcu1_log4: MCU1_LOG4,
    #[doc = "0x120 - mcu1_log5"]
    pub mcu1_log5: MCU1_LOG5,
    _reserved9: [u8; 0xe4],
    #[doc = "0x208 - irom1_misr_dataout_0"]
    pub irom1_misr_dataout_0: IROM1_MISR_DATAOUT_0,
    #[doc = "0x20c - irom1_misr_dataout_1"]
    pub irom1_misr_dataout_1: IROM1_MISR_DATAOUT_1,
}
#[doc = "mcu_bus_cfg0 (rw) register accessor: an alias for `Reg<MCU_BUS_CFG0_SPEC>`"]
pub type MCU_BUS_CFG0 = crate::Reg<mcu_bus_cfg0::MCU_BUS_CFG0_SPEC>;
#[doc = "mcu_bus_cfg0"]
pub mod mcu_bus_cfg0;
#[doc = "mcu_bus_cfg1 (rw) register accessor: an alias for `Reg<MCU_BUS_CFG1_SPEC>`"]
pub type MCU_BUS_CFG1 = crate::Reg<mcu_bus_cfg1::MCU_BUS_CFG1_SPEC>;
#[doc = "mcu_bus_cfg1"]
pub mod mcu_bus_cfg1;
#[doc = "mcu_e907_rtc (rw) register accessor: an alias for `Reg<MCU_E907_RTC_SPEC>`"]
pub type MCU_E907_RTC = crate::Reg<mcu_e907_rtc::MCU_E907_RTC_SPEC>;
#[doc = "mcu_e907_rtc"]
pub mod mcu_e907_rtc;
#[doc = "mcu_cfg1 (rw) register accessor: an alias for `Reg<MCU_CFG1_SPEC>`"]
pub type MCU_CFG1 = crate::Reg<mcu_cfg1::MCU_CFG1_SPEC>;
#[doc = "mcu_cfg1"]
pub mod mcu_cfg1;
#[doc = "mcu1_log1 (rw) register accessor: an alias for `Reg<MCU1_LOG1_SPEC>`"]
pub type MCU1_LOG1 = crate::Reg<mcu1_log1::MCU1_LOG1_SPEC>;
#[doc = "mcu1_log1"]
pub mod mcu1_log1;
#[doc = "mcu1_log2 (rw) register accessor: an alias for `Reg<MCU1_LOG2_SPEC>`"]
pub type MCU1_LOG2 = crate::Reg<mcu1_log2::MCU1_LOG2_SPEC>;
#[doc = "mcu1_log2"]
pub mod mcu1_log2;
#[doc = "mcu1_log3 (rw) register accessor: an alias for `Reg<MCU1_LOG3_SPEC>`"]
pub type MCU1_LOG3 = crate::Reg<mcu1_log3::MCU1_LOG3_SPEC>;
#[doc = "mcu1_log3"]
pub mod mcu1_log3;
#[doc = "mcu1_log4 (rw) register accessor: an alias for `Reg<MCU1_LOG4_SPEC>`"]
pub type MCU1_LOG4 = crate::Reg<mcu1_log4::MCU1_LOG4_SPEC>;
#[doc = "mcu1_log4"]
pub mod mcu1_log4;
#[doc = "mcu1_log5 (rw) register accessor: an alias for `Reg<MCU1_LOG5_SPEC>`"]
pub type MCU1_LOG5 = crate::Reg<mcu1_log5::MCU1_LOG5_SPEC>;
#[doc = "mcu1_log5"]
pub mod mcu1_log5;
#[doc = "irom1_misr_dataout_0 (rw) register accessor: an alias for `Reg<IROM1_MISR_DATAOUT_0_SPEC>`"]
pub type IROM1_MISR_DATAOUT_0 = crate::Reg<irom1_misr_dataout_0::IROM1_MISR_DATAOUT_0_SPEC>;
#[doc = "irom1_misr_dataout_0"]
pub mod irom1_misr_dataout_0;
#[doc = "irom1_misr_dataout_1 (rw) register accessor: an alias for `Reg<IROM1_MISR_DATAOUT_1_SPEC>`"]
pub type IROM1_MISR_DATAOUT_1 = crate::Reg<irom1_misr_dataout_1::IROM1_MISR_DATAOUT_1_SPEC>;
#[doc = "irom1_misr_dataout_1"]
pub mod irom1_misr_dataout_1;
