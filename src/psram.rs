#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - psram_configure"]
    pub psram_configure: PSRAM_CONFIGURE,
    #[doc = "0x04 - psram_manual_control"]
    pub psram_manual_control: PSRAM_MANUAL_CONTROL,
    #[doc = "0x08 - fifo_thres_control"]
    pub fifo_thres_control: FIFO_THRES_CONTROL,
    #[doc = "0x0c - psram_manual_control2"]
    pub psram_manual_control2: PSRAM_MANUAL_CONTROL2,
    #[doc = "0x10 - winbond_psram_configure"]
    pub winbond_psram_configure: WINBOND_PSRAM_CONFIGURE,
    #[doc = "0x14 - winbond_psram_status"]
    pub winbond_psram_status: WINBOND_PSRAM_STATUS,
    #[doc = "0x18 - winbond_psram_configure2"]
    pub winbond_psram_configure2: WINBOND_PSRAM_CONFIGURE2,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - apmemory_psram_configure"]
    pub apmemory_psram_configure: APMEMORY_PSRAM_CONFIGURE,
    #[doc = "0x24 - apmemory_psram_status"]
    pub apmemory_psram_status: APMEMORY_PSRAM_STATUS,
    _reserved9: [u8; 0x08],
    #[doc = "0x30 - psram_manual_control3"]
    pub psram_manual_control3: PSRAM_MANUAL_CONTROL3,
    _reserved10: [u8; 0x4c],
    #[doc = "0x80 - psram_intf_delay_ctrl0"]
    pub psram_intf_delay_ctrl0: PSRAM_INTF_DELAY_CTRL0,
    #[doc = "0x84 - psram_intf_delay_ctrl1"]
    pub psram_intf_delay_ctrl1: PSRAM_INTF_DELAY_CTRL1,
    #[doc = "0x88 - psram_intf_delay_ctrl2"]
    pub psram_intf_delay_ctrl2: PSRAM_INTF_DELAY_CTRL2,
    #[doc = "0x8c - psram_intf_delay_ctrl3"]
    pub psram_intf_delay_ctrl3: PSRAM_INTF_DELAY_CTRL3,
    #[doc = "0x90 - psram_intf_delay_ctrl4"]
    pub psram_intf_delay_ctrl4: PSRAM_INTF_DELAY_CTRL4,
    #[doc = "0x94 - psram_intf_delay_ctrl5"]
    pub psram_intf_delay_ctrl5: PSRAM_INTF_DELAY_CTRL5,
    #[doc = "0x98 - psram_intf_delay_ctrl6"]
    pub psram_intf_delay_ctrl6: PSRAM_INTF_DELAY_CTRL6,
    #[doc = "0x9c - psram_intf_delay_ctrl7"]
    pub psram_intf_delay_ctrl7: PSRAM_INTF_DELAY_CTRL7,
    #[doc = "0xa0 - psram_intf_delay_ctrl8"]
    pub psram_intf_delay_ctrl8: PSRAM_INTF_DELAY_CTRL8,
    #[doc = "0xa4 - psram_intf_delay_ctrl9"]
    pub psram_intf_delay_ctrl9: PSRAM_INTF_DELAY_CTRL9,
    #[doc = "0xa8 - psram_intf_delay_ctrlA"]
    pub psram_dbg_sel: PSRAM_DBG_SEL,
    _reserved21: [u8; 0x44],
    #[doc = "0xf0 - psram_dummy_reg"]
    pub psram_dummy_reg: PSRAM_DUMMY_REG,
    #[doc = "0xf4 - psram_timeout_reg"]
    pub psram_timeout_reg: PSRAM_TIMEOUT_REG,
    _reserved23: [u8; 0x08],
    #[doc = "0x100 - psram_rough_delay_ctrl0"]
    pub psram_rough_delay_ctrl0: PSRAM_ROUGH_DELAY_CTRL0,
    _reserved_24_phy_cfg_04: [u8; 0x04],
    _reserved_25_phy_cfg_08: [u8; 0x04],
    _reserved_26_phy_cfg_10: [u8; 0x04],
    #[doc = "0x110 - psram_rough_delay_ctrl4"]
    pub psram_rough_delay_ctrl4: PSRAM_ROUGH_DELAY_CTRL4,
    _reserved_28_phy_cfg_14: [u8; 0x04],
    _reserved_29_phy_cfg_18: [u8; 0x04],
    _reserved_30_phy_cfg_20: [u8; 0x04],
    #[doc = "0x120 - psram_rough_delay_ctrl8"]
    pub psram_rough_delay_ctrl8: PSRAM_ROUGH_DELAY_CTRL8,
    _reserved_32_phy_cfg_24: [u8; 0x04],
    _reserved_33_phy_cfg_: [u8; 0x04],
    #[doc = "0x12c - phy_cfg_2C"]
    pub phy_cfg_30: PHY_CFG_30,
    _reserved35: [u8; 0x04],
    #[doc = "0x134 - phy_cfg_34"]
    pub phy_cfg_34: PHY_CFG_34,
    #[doc = "0x138 - phy_cfg_38"]
    pub phy_cfg_38: PHY_CFG_38,
    #[doc = "0x13c - phy_cfg_3C"]
    pub phy_cfg_40: PHY_CFG_40,
    _reserved38: [u8; 0x04],
    #[doc = "0x144 - phy_cfg_44"]
    pub phy_cfg_44: PHY_CFG_44,
    #[doc = "0x148 - phy_cfg_48"]
    pub phy_cfg_48: PHY_CFG_48,
}
impl RegisterBlock {
    #[doc = "0x104 - phy_cfg_04"]
    #[inline(always)]
    pub const fn phy_cfg_04(&self) -> &PHY_CFG_04 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260usize).cast() }
    }
    #[doc = "0x104 - psram_rough_delay_ctrl1"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl1(&self) -> &PSRAM_ROUGH_DELAY_CTRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260usize).cast() }
    }
    #[doc = "0x108 - phy_cfg_08"]
    #[inline(always)]
    pub const fn phy_cfg_08(&self) -> &PHY_CFG_08 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    #[doc = "0x108 - psram_rough_delay_ctrl2"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl2(&self) -> &PSRAM_ROUGH_DELAY_CTRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    #[doc = "0x10c - phy_cfg_0C"]
    #[inline(always)]
    pub const fn phy_cfg_10(&self) -> &PHY_CFG_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268usize).cast() }
    }
    #[doc = "0x10c - psram_rough_delay_ctrl3"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl3(&self) -> &PSRAM_ROUGH_DELAY_CTRL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268usize).cast() }
    }
    #[doc = "0x114 - phy_cfg_14"]
    #[inline(always)]
    pub const fn phy_cfg_14(&self) -> &PHY_CFG_14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276usize).cast() }
    }
    #[doc = "0x114 - psram_rough_delay_ctrl5"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl5(&self) -> &PSRAM_ROUGH_DELAY_CTRL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276usize).cast() }
    }
    #[doc = "0x118 - phy_cfg_18"]
    #[inline(always)]
    pub const fn phy_cfg_18(&self) -> &PHY_CFG_18 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280usize).cast() }
    }
    #[doc = "0x118 - psram_rough_delay_ctrl6"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl6(&self) -> &PSRAM_ROUGH_DELAY_CTRL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280usize).cast() }
    }
    #[doc = "0x11c - phy_cfg_1C"]
    #[inline(always)]
    pub const fn phy_cfg_20(&self) -> &PHY_CFG_20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284usize).cast() }
    }
    #[doc = "0x11c - psram_rough_delay_ctrl7"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl7(&self) -> &PSRAM_ROUGH_DELAY_CTRL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284usize).cast() }
    }
    #[doc = "0x124 - phy_cfg_24"]
    #[inline(always)]
    pub const fn phy_cfg_24(&self) -> &PHY_CFG_24 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292usize).cast() }
    }
    #[doc = "0x124 - psram_rough_delay_ctrl9"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl9(&self) -> &PSRAM_ROUGH_DELAY_CTRL9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292usize).cast() }
    }
    #[doc = "0x128 - phy_cfg_28"]
    #[inline(always)]
    pub const fn phy_cfg_28(&self) -> &PHY_CFG_28 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
    #[doc = "0x128 - psram_rough_delay_ctrlA"]
    #[inline(always)]
    pub const fn phy_cfg_00(&self) -> &PHY_CFG_00 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
}
#[doc = "psram_configure (rw) register accessor: an alias for `Reg<PSRAM_CONFIGURE_SPEC>`"]
pub type PSRAM_CONFIGURE = crate::Reg<psram_configure::PSRAM_CONFIGURE_SPEC>;
#[doc = "psram_configure"]
pub mod psram_configure;
#[doc = "psram_manual_control (rw) register accessor: an alias for `Reg<PSRAM_MANUAL_CONTROL_SPEC>`"]
pub type PSRAM_MANUAL_CONTROL = crate::Reg<psram_manual_control::PSRAM_MANUAL_CONTROL_SPEC>;
#[doc = "psram_manual_control"]
pub mod psram_manual_control;
#[doc = "fifo_thres_control (rw) register accessor: an alias for `Reg<FIFO_THRES_CONTROL_SPEC>`"]
pub type FIFO_THRES_CONTROL = crate::Reg<fifo_thres_control::FIFO_THRES_CONTROL_SPEC>;
#[doc = "fifo_thres_control"]
pub mod fifo_thres_control;
#[doc = "psram_manual_control2 (rw) register accessor: an alias for `Reg<PSRAM_MANUAL_CONTROL2_SPEC>`"]
pub type PSRAM_MANUAL_CONTROL2 = crate::Reg<psram_manual_control2::PSRAM_MANUAL_CONTROL2_SPEC>;
#[doc = "psram_manual_control2"]
pub mod psram_manual_control2;
#[doc = "winbond_psram_configure (rw) register accessor: an alias for `Reg<WINBOND_PSRAM_CONFIGURE_SPEC>`"]
pub type WINBOND_PSRAM_CONFIGURE =
    crate::Reg<winbond_psram_configure::WINBOND_PSRAM_CONFIGURE_SPEC>;
#[doc = "winbond_psram_configure"]
pub mod winbond_psram_configure;
#[doc = "winbond_psram_status (rw) register accessor: an alias for `Reg<WINBOND_PSRAM_STATUS_SPEC>`"]
pub type WINBOND_PSRAM_STATUS = crate::Reg<winbond_psram_status::WINBOND_PSRAM_STATUS_SPEC>;
#[doc = "winbond_psram_status"]
pub mod winbond_psram_status;
#[doc = "winbond_psram_configure2 (rw) register accessor: an alias for `Reg<WINBOND_PSRAM_CONFIGURE2_SPEC>`"]
pub type WINBOND_PSRAM_CONFIGURE2 =
    crate::Reg<winbond_psram_configure2::WINBOND_PSRAM_CONFIGURE2_SPEC>;
#[doc = "winbond_psram_configure2"]
pub mod winbond_psram_configure2;
#[doc = "apmemory_psram_configure (rw) register accessor: an alias for `Reg<APMEMORY_PSRAM_CONFIGURE_SPEC>`"]
pub type APMEMORY_PSRAM_CONFIGURE =
    crate::Reg<apmemory_psram_configure::APMEMORY_PSRAM_CONFIGURE_SPEC>;
#[doc = "apmemory_psram_configure"]
pub mod apmemory_psram_configure;
#[doc = "apmemory_psram_status (rw) register accessor: an alias for `Reg<APMEMORY_PSRAM_STATUS_SPEC>`"]
pub type APMEMORY_PSRAM_STATUS = crate::Reg<apmemory_psram_status::APMEMORY_PSRAM_STATUS_SPEC>;
#[doc = "apmemory_psram_status"]
pub mod apmemory_psram_status;
#[doc = "psram_manual_control3 (rw) register accessor: an alias for `Reg<PSRAM_MANUAL_CONTROL3_SPEC>`"]
pub type PSRAM_MANUAL_CONTROL3 = crate::Reg<psram_manual_control3::PSRAM_MANUAL_CONTROL3_SPEC>;
#[doc = "psram_manual_control3"]
pub mod psram_manual_control3;
#[doc = "psram_intf_delay_ctrl0 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL0_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL0 = crate::Reg<psram_intf_delay_ctrl0::PSRAM_INTF_DELAY_CTRL0_SPEC>;
#[doc = "psram_intf_delay_ctrl0"]
pub mod psram_intf_delay_ctrl0;
#[doc = "psram_intf_delay_ctrl1 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL1_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL1 = crate::Reg<psram_intf_delay_ctrl1::PSRAM_INTF_DELAY_CTRL1_SPEC>;
#[doc = "psram_intf_delay_ctrl1"]
pub mod psram_intf_delay_ctrl1;
#[doc = "psram_intf_delay_ctrl2 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL2_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL2 = crate::Reg<psram_intf_delay_ctrl2::PSRAM_INTF_DELAY_CTRL2_SPEC>;
#[doc = "psram_intf_delay_ctrl2"]
pub mod psram_intf_delay_ctrl2;
#[doc = "psram_intf_delay_ctrl3 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL3_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL3 = crate::Reg<psram_intf_delay_ctrl3::PSRAM_INTF_DELAY_CTRL3_SPEC>;
#[doc = "psram_intf_delay_ctrl3"]
pub mod psram_intf_delay_ctrl3;
#[doc = "psram_intf_delay_ctrl4 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL4_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL4 = crate::Reg<psram_intf_delay_ctrl4::PSRAM_INTF_DELAY_CTRL4_SPEC>;
#[doc = "psram_intf_delay_ctrl4"]
pub mod psram_intf_delay_ctrl4;
#[doc = "psram_intf_delay_ctrl5 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL5_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL5 = crate::Reg<psram_intf_delay_ctrl5::PSRAM_INTF_DELAY_CTRL5_SPEC>;
#[doc = "psram_intf_delay_ctrl5"]
pub mod psram_intf_delay_ctrl5;
#[doc = "psram_intf_delay_ctrl6 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL6_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL6 = crate::Reg<psram_intf_delay_ctrl6::PSRAM_INTF_DELAY_CTRL6_SPEC>;
#[doc = "psram_intf_delay_ctrl6"]
pub mod psram_intf_delay_ctrl6;
#[doc = "psram_intf_delay_ctrl7 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL7_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL7 = crate::Reg<psram_intf_delay_ctrl7::PSRAM_INTF_DELAY_CTRL7_SPEC>;
#[doc = "psram_intf_delay_ctrl7"]
pub mod psram_intf_delay_ctrl7;
#[doc = "psram_intf_delay_ctrl8 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL8_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL8 = crate::Reg<psram_intf_delay_ctrl8::PSRAM_INTF_DELAY_CTRL8_SPEC>;
#[doc = "psram_intf_delay_ctrl8"]
pub mod psram_intf_delay_ctrl8;
#[doc = "psram_intf_delay_ctrl9 (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRL9_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRL9 = crate::Reg<psram_intf_delay_ctrl9::PSRAM_INTF_DELAY_CTRL9_SPEC>;
#[doc = "psram_intf_delay_ctrl9"]
pub mod psram_intf_delay_ctrl9;
#[doc = "psram_dbg_sel (rw) register accessor: an alias for `Reg<PSRAM_DBG_SEL_SPEC>`"]
pub type PSRAM_DBG_SEL = crate::Reg<psram_dbg_sel::PSRAM_DBG_SEL_SPEC>;
#[doc = "psram_intf_delay_ctrlA"]
pub mod psram_dbg_sel;
#[doc = "psram_dummy_reg (rw) register accessor: an alias for `Reg<PSRAM_DUMMY_REG_SPEC>`"]
pub type PSRAM_DUMMY_REG = crate::Reg<psram_dummy_reg::PSRAM_DUMMY_REG_SPEC>;
#[doc = "psram_dummy_reg"]
pub mod psram_dummy_reg;
#[doc = "psram_timeout_reg (rw) register accessor: an alias for `Reg<PSRAM_TIMEOUT_REG_SPEC>`"]
pub type PSRAM_TIMEOUT_REG = crate::Reg<psram_timeout_reg::PSRAM_TIMEOUT_REG_SPEC>;
#[doc = "psram_timeout_reg"]
pub mod psram_timeout_reg;
#[doc = "psram_rough_delay_ctrl0 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL0_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL0 =
    crate::Reg<psram_rough_delay_ctrl0::PSRAM_ROUGH_DELAY_CTRL0_SPEC>;
#[doc = "psram_rough_delay_ctrl0"]
pub mod psram_rough_delay_ctrl0;
#[doc = "psram_rough_delay_ctrl1 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL1_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL1 =
    crate::Reg<psram_rough_delay_ctrl1::PSRAM_ROUGH_DELAY_CTRL1_SPEC>;
#[doc = "psram_rough_delay_ctrl1"]
pub mod psram_rough_delay_ctrl1;
#[doc = "psram_rough_delay_ctrl2 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL2_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL2 =
    crate::Reg<psram_rough_delay_ctrl2::PSRAM_ROUGH_DELAY_CTRL2_SPEC>;
#[doc = "psram_rough_delay_ctrl2"]
pub mod psram_rough_delay_ctrl2;
#[doc = "psram_rough_delay_ctrl3 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL3_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL3 =
    crate::Reg<psram_rough_delay_ctrl3::PSRAM_ROUGH_DELAY_CTRL3_SPEC>;
#[doc = "psram_rough_delay_ctrl3"]
pub mod psram_rough_delay_ctrl3;
#[doc = "psram_rough_delay_ctrl4 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL4_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL4 =
    crate::Reg<psram_rough_delay_ctrl4::PSRAM_ROUGH_DELAY_CTRL4_SPEC>;
#[doc = "psram_rough_delay_ctrl4"]
pub mod psram_rough_delay_ctrl4;
#[doc = "psram_rough_delay_ctrl5 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL5_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL5 =
    crate::Reg<psram_rough_delay_ctrl5::PSRAM_ROUGH_DELAY_CTRL5_SPEC>;
#[doc = "psram_rough_delay_ctrl5"]
pub mod psram_rough_delay_ctrl5;
#[doc = "psram_rough_delay_ctrl6 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL6_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL6 =
    crate::Reg<psram_rough_delay_ctrl6::PSRAM_ROUGH_DELAY_CTRL6_SPEC>;
#[doc = "psram_rough_delay_ctrl6"]
pub mod psram_rough_delay_ctrl6;
#[doc = "psram_rough_delay_ctrl7 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL7_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL7 =
    crate::Reg<psram_rough_delay_ctrl7::PSRAM_ROUGH_DELAY_CTRL7_SPEC>;
#[doc = "psram_rough_delay_ctrl7"]
pub mod psram_rough_delay_ctrl7;
#[doc = "psram_rough_delay_ctrl8 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL8_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL8 =
    crate::Reg<psram_rough_delay_ctrl8::PSRAM_ROUGH_DELAY_CTRL8_SPEC>;
#[doc = "psram_rough_delay_ctrl8"]
pub mod psram_rough_delay_ctrl8;
#[doc = "psram_rough_delay_ctrl9 (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRL9_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRL9 =
    crate::Reg<psram_rough_delay_ctrl9::PSRAM_ROUGH_DELAY_CTRL9_SPEC>;
#[doc = "psram_rough_delay_ctrl9"]
pub mod psram_rough_delay_ctrl9;
#[doc = "phy_cfg_00 (rw) register accessor: an alias for `Reg<PHY_CFG_00_SPEC>`"]
pub type PHY_CFG_00 = crate::Reg<phy_cfg_00::PHY_CFG_00_SPEC>;
#[doc = "psram_rough_delay_ctrlA"]
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
