#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uhs_basic: [u8; 0x04],
    _reserved_1_uhs_cmd: [u8; 0x04],
    _reserved_2_uhs_fifo_thre: [u8; 0x04],
    _reserved_3_uhs_manual: [u8; 0x04],
    _reserved_4_uhs_auto_fresh_1: [u8; 0x04],
    _reserved_5_uhs_auto_fresh_2: [u8; 0x04],
    _reserved_6_uhs_auto_fresh_3: [u8; 0x04],
    #[doc = "0x1c - UHS_auto_fresh_4"]
    pub uhs_auto_fresh_4: UHS_AUTO_FRESH_4,
    _reserved_8_uhs_psram_configure: [u8; 0x04],
    _reserved_9_uhs_psram_status: [u8; 0x04],
    _reserved10: [u8; 0x08],
    _reserved_10_uhs_timing_ctrl: [u8; 0x04],
    #[doc = "0x34 - UHS_rsvd_reg"]
    pub uhs_rsvd_reg: UHS_RSVD_REG,
    _reserved12: [u8; 0x48],
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
    pub psram_intf_delay_ctrla: PSRAM_INTF_DELAY_CTRLA,
    #[doc = "0xac - psram_intf_delay_ctrlB"]
    pub psram_intf_delay_ctrlb: PSRAM_INTF_DELAY_CTRLB,
    _reserved24: [u8; 0x10],
    _reserved_24_uhs_dbg_sel: [u8; 0x04],
    _reserved25: [u8; 0x2c],
    _reserved_25_uhs_dummy_reg: [u8; 0x04],
    #[doc = "0xf4 - psram_timeout_reg"]
    pub psram_timeout_reg: PSRAM_TIMEOUT_REG,
    _reserved27: [u8; 0x08],
    _reserved_27_phy_cfg_00: [u8; 0x04],
    _reserved_28_phy_cfg_04: [u8; 0x04],
    _reserved_29_phy_cfg_08: [u8; 0x04],
    _reserved_30_phy_cfg_0c: [u8; 0x04],
    _reserved_31_phy_cfg_10: [u8; 0x04],
    _reserved_32_phy_cfg_14: [u8; 0x04],
    _reserved_33_phy_cfg_18: [u8; 0x04],
    _reserved_34_phy_cfg_1c: [u8; 0x04],
    _reserved_35_phy_cfg_20: [u8; 0x04],
    _reserved_36_phy_cfg_24: [u8; 0x04],
    _reserved_37_phy_cfg_28: [u8; 0x04],
    _reserved_38_phy_cfg_2c: [u8; 0x04],
    #[doc = "0x130 - phy_cfg_30"]
    pub phy_cfg_30: PHY_CFG_30,
    #[doc = "0x134 - phy_cfg_34"]
    pub phy_cfg_34: PHY_CFG_34,
    #[doc = "0x138 - phy_cfg_38"]
    pub phy_cfg_38: PHY_CFG_38,
    #[doc = "0x13c - phy_cfg_3C"]
    pub phy_cfg_3c: PHY_CFG_3C,
    #[doc = "0x140 - phy_cfg_40"]
    pub phy_cfg_40: PHY_CFG_40,
    #[doc = "0x144 - phy_cfg_44"]
    pub phy_cfg_44: PHY_CFG_44,
    #[doc = "0x148 - phy_cfg_48"]
    pub phy_cfg_48: PHY_CFG_48,
    #[doc = "0x14c - phy_cfg_4C"]
    pub phy_cfg_4c: PHY_CFG_4C,
    #[doc = "0x150 - phy_cfg_50"]
    pub phy_cfg_50: PHY_CFG_50,
}
impl RegisterBlock {
    #[doc = "0x00 - UHS_basic"]
    #[inline(always)]
    pub const fn uhs_basic(&self) -> &UHS_BASIC {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - psram_configure"]
    #[inline(always)]
    pub const fn psram_configure(&self) -> &PSRAM_CONFIGURE {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - UHS_cmd"]
    #[inline(always)]
    pub const fn uhs_cmd(&self) -> &UHS_CMD {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - psram_manual_control"]
    #[inline(always)]
    pub const fn psram_manual_control(&self) -> &PSRAM_MANUAL_CONTROL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - UHS_fifo_thre"]
    #[inline(always)]
    pub const fn uhs_fifo_thre(&self) -> &UHS_FIFO_THRE {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - fifo_thres_control"]
    #[inline(always)]
    pub const fn fifo_thres_control(&self) -> &FIFO_THRES_CONTROL {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0c - UHS_manual"]
    #[inline(always)]
    pub const fn uhs_manual(&self) -> &UHS_MANUAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - psram_manual_control2"]
    #[inline(always)]
    pub const fn psram_manual_control2(&self) -> &PSRAM_MANUAL_CONTROL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x10 - UHS_auto_fresh_1"]
    #[inline(always)]
    pub const fn uhs_auto_fresh_1(&self) -> &UHS_AUTO_FRESH_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - winbond_psram_configure"]
    #[inline(always)]
    pub const fn winbond_psram_configure(&self) -> &WINBOND_PSRAM_CONFIGURE {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x14 - UHS_auto_fresh_2"]
    #[inline(always)]
    pub const fn uhs_auto_fresh_2(&self) -> &UHS_AUTO_FRESH_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - winbond_psram_status"]
    #[inline(always)]
    pub const fn winbond_psram_status(&self) -> &WINBOND_PSRAM_STATUS {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x18 - UHS_auto_fresh_3"]
    #[inline(always)]
    pub const fn uhs_auto_fresh_3(&self) -> &UHS_AUTO_FRESH_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - winbond_psram_configure2"]
    #[inline(always)]
    pub const fn winbond_psram_configure2(&self) -> &WINBOND_PSRAM_CONFIGURE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x20 - UHS_psram_configure"]
    #[inline(always)]
    pub const fn uhs_psram_configure(&self) -> &UHS_PSRAM_CONFIGURE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - apmemory_psram_configure"]
    #[inline(always)]
    pub const fn apmemory_psram_configure(&self) -> &APMEMORY_PSRAM_CONFIGURE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x24 - UHS_psram_status"]
    #[inline(always)]
    pub const fn uhs_psram_status(&self) -> &UHS_PSRAM_STATUS {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x24 - apmemory_psram_status"]
    #[inline(always)]
    pub const fn apmemory_psram_status(&self) -> &APMEMORY_PSRAM_STATUS {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x30 - UHS_timing_ctrl"]
    #[inline(always)]
    pub const fn uhs_timing_ctrl(&self) -> &UHS_TIMING_CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - psram_manual_control3"]
    #[inline(always)]
    pub const fn psram_manual_control3(&self) -> &PSRAM_MANUAL_CONTROL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0xc0 - UHS_dbg_sel"]
    #[inline(always)]
    pub const fn uhs_dbg_sel(&self) -> &UHS_DBG_SEL {
        unsafe { &*(self as *const Self).cast::<u8>().add(192usize).cast() }
    }
    #[doc = "0xc0 - psram_dbg_sel"]
    #[inline(always)]
    pub const fn psram_dbg_sel(&self) -> &PSRAM_DBG_SEL {
        unsafe { &*(self as *const Self).cast::<u8>().add(192usize).cast() }
    }
    #[doc = "0xf0 - UHS_dummy_reg"]
    #[inline(always)]
    pub const fn uhs_dummy_reg(&self) -> &UHS_DUMMY_REG {
        unsafe { &*(self as *const Self).cast::<u8>().add(240usize).cast() }
    }
    #[doc = "0xf0 - psram_dummy_reg"]
    #[inline(always)]
    pub const fn psram_dummy_reg(&self) -> &PSRAM_DUMMY_REG {
        unsafe { &*(self as *const Self).cast::<u8>().add(240usize).cast() }
    }
    #[doc = "0x100 - phy_cfg_00"]
    #[inline(always)]
    pub const fn phy_cfg_00(&self) -> &PHY_CFG_00 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
    #[doc = "0x100 - psram_rough_delay_ctrl0"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl0(&self) -> &PSRAM_ROUGH_DELAY_CTRL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
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
    pub const fn phy_cfg_0c(&self) -> &PHY_CFG_0C {
        unsafe { &*(self as *const Self).cast::<u8>().add(268usize).cast() }
    }
    #[doc = "0x10c - psram_rough_delay_ctrl3"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl3(&self) -> &PSRAM_ROUGH_DELAY_CTRL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268usize).cast() }
    }
    #[doc = "0x110 - phy_cfg_10"]
    #[inline(always)]
    pub const fn phy_cfg_10(&self) -> &PHY_CFG_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272usize).cast() }
    }
    #[doc = "0x110 - psram_rough_delay_ctrl4"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl4(&self) -> &PSRAM_ROUGH_DELAY_CTRL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272usize).cast() }
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
    pub const fn phy_cfg_1c(&self) -> &PHY_CFG_1C {
        unsafe { &*(self as *const Self).cast::<u8>().add(284usize).cast() }
    }
    #[doc = "0x11c - psram_rough_delay_ctrl7"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl7(&self) -> &PSRAM_ROUGH_DELAY_CTRL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284usize).cast() }
    }
    #[doc = "0x120 - phy_cfg_20"]
    #[inline(always)]
    pub const fn phy_cfg_20(&self) -> &PHY_CFG_20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(288usize).cast() }
    }
    #[doc = "0x120 - psram_rough_delay_ctrl8"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl8(&self) -> &PSRAM_ROUGH_DELAY_CTRL8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(288usize).cast() }
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
    pub const fn psram_rough_delay_ctrla(&self) -> &PSRAM_ROUGH_DELAY_CTRLA {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
    #[doc = "0x12c - phy_cfg_2C"]
    #[inline(always)]
    pub const fn phy_cfg_2c(&self) -> &PHY_CFG_2C {
        unsafe { &*(self as *const Self).cast::<u8>().add(300usize).cast() }
    }
    #[doc = "0x12c - psram_rough_delay_ctrlB"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrlb(&self) -> &PSRAM_ROUGH_DELAY_CTRLB {
        unsafe { &*(self as *const Self).cast::<u8>().add(300usize).cast() }
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
#[doc = "psram_intf_delay_ctrla (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRLA_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRLA = crate::Reg<psram_intf_delay_ctrla::PSRAM_INTF_DELAY_CTRLA_SPEC>;
#[doc = "psram_intf_delay_ctrlA"]
pub mod psram_intf_delay_ctrla;
#[doc = "psram_intf_delay_ctrlb (rw) register accessor: an alias for `Reg<PSRAM_INTF_DELAY_CTRLB_SPEC>`"]
pub type PSRAM_INTF_DELAY_CTRLB = crate::Reg<psram_intf_delay_ctrlb::PSRAM_INTF_DELAY_CTRLB_SPEC>;
#[doc = "psram_intf_delay_ctrlB"]
pub mod psram_intf_delay_ctrlb;
#[doc = "psram_dbg_sel (rw) register accessor: an alias for `Reg<PSRAM_DBG_SEL_SPEC>`"]
pub type PSRAM_DBG_SEL = crate::Reg<psram_dbg_sel::PSRAM_DBG_SEL_SPEC>;
#[doc = "psram_dbg_sel"]
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
#[doc = "psram_rough_delay_ctrla (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRLA_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRLA =
    crate::Reg<psram_rough_delay_ctrla::PSRAM_ROUGH_DELAY_CTRLA_SPEC>;
#[doc = "psram_rough_delay_ctrlA"]
pub mod psram_rough_delay_ctrla;
#[doc = "psram_rough_delay_ctrlb (rw) register accessor: an alias for `Reg<PSRAM_ROUGH_DELAY_CTRLB_SPEC>`"]
pub type PSRAM_ROUGH_DELAY_CTRLB =
    crate::Reg<psram_rough_delay_ctrlb::PSRAM_ROUGH_DELAY_CTRLB_SPEC>;
#[doc = "psram_rough_delay_ctrlB"]
pub mod psram_rough_delay_ctrlb;
#[doc = "uhs_basic (rw) register accessor: an alias for `Reg<UHS_BASIC_SPEC>`"]
pub type UHS_BASIC = crate::Reg<uhs_basic::UHS_BASIC_SPEC>;
#[doc = "UHS_basic"]
pub mod uhs_basic;
#[doc = "uhs_cmd (rw) register accessor: an alias for `Reg<UHS_CMD_SPEC>`"]
pub type UHS_CMD = crate::Reg<uhs_cmd::UHS_CMD_SPEC>;
#[doc = "UHS_cmd"]
pub mod uhs_cmd;
#[doc = "uhs_fifo_thre (rw) register accessor: an alias for `Reg<UHS_FIFO_THRE_SPEC>`"]
pub type UHS_FIFO_THRE = crate::Reg<uhs_fifo_thre::UHS_FIFO_THRE_SPEC>;
#[doc = "UHS_fifo_thre"]
pub mod uhs_fifo_thre;
#[doc = "uhs_manual (rw) register accessor: an alias for `Reg<UHS_MANUAL_SPEC>`"]
pub type UHS_MANUAL = crate::Reg<uhs_manual::UHS_MANUAL_SPEC>;
#[doc = "UHS_manual"]
pub mod uhs_manual;
#[doc = "uhs_auto_fresh_1 (rw) register accessor: an alias for `Reg<UHS_AUTO_FRESH_1_SPEC>`"]
pub type UHS_AUTO_FRESH_1 = crate::Reg<uhs_auto_fresh_1::UHS_AUTO_FRESH_1_SPEC>;
#[doc = "UHS_auto_fresh_1"]
pub mod uhs_auto_fresh_1;
#[doc = "uhs_auto_fresh_2 (rw) register accessor: an alias for `Reg<UHS_AUTO_FRESH_2_SPEC>`"]
pub type UHS_AUTO_FRESH_2 = crate::Reg<uhs_auto_fresh_2::UHS_AUTO_FRESH_2_SPEC>;
#[doc = "UHS_auto_fresh_2"]
pub mod uhs_auto_fresh_2;
#[doc = "uhs_auto_fresh_3 (rw) register accessor: an alias for `Reg<UHS_AUTO_FRESH_3_SPEC>`"]
pub type UHS_AUTO_FRESH_3 = crate::Reg<uhs_auto_fresh_3::UHS_AUTO_FRESH_3_SPEC>;
#[doc = "UHS_auto_fresh_3"]
pub mod uhs_auto_fresh_3;
#[doc = "uhs_auto_fresh_4 (rw) register accessor: an alias for `Reg<UHS_AUTO_FRESH_4_SPEC>`"]
pub type UHS_AUTO_FRESH_4 = crate::Reg<uhs_auto_fresh_4::UHS_AUTO_FRESH_4_SPEC>;
#[doc = "UHS_auto_fresh_4"]
pub mod uhs_auto_fresh_4;
#[doc = "uhs_psram_configure (rw) register accessor: an alias for `Reg<UHS_PSRAM_CONFIGURE_SPEC>`"]
pub type UHS_PSRAM_CONFIGURE = crate::Reg<uhs_psram_configure::UHS_PSRAM_CONFIGURE_SPEC>;
#[doc = "UHS_psram_configure"]
pub mod uhs_psram_configure;
#[doc = "uhs_psram_status (rw) register accessor: an alias for `Reg<UHS_PSRAM_STATUS_SPEC>`"]
pub type UHS_PSRAM_STATUS = crate::Reg<uhs_psram_status::UHS_PSRAM_STATUS_SPEC>;
#[doc = "UHS_psram_status"]
pub mod uhs_psram_status;
#[doc = "uhs_timing_ctrl (rw) register accessor: an alias for `Reg<UHS_TIMING_CTRL_SPEC>`"]
pub type UHS_TIMING_CTRL = crate::Reg<uhs_timing_ctrl::UHS_TIMING_CTRL_SPEC>;
#[doc = "UHS_timing_ctrl"]
pub mod uhs_timing_ctrl;
#[doc = "uhs_rsvd_reg (rw) register accessor: an alias for `Reg<UHS_RSVD_REG_SPEC>`"]
pub type UHS_RSVD_REG = crate::Reg<uhs_rsvd_reg::UHS_RSVD_REG_SPEC>;
#[doc = "UHS_rsvd_reg"]
pub mod uhs_rsvd_reg;
#[doc = "uhs_dbg_sel (rw) register accessor: an alias for `Reg<UHS_DBG_SEL_SPEC>`"]
pub type UHS_DBG_SEL = crate::Reg<uhs_dbg_sel::UHS_DBG_SEL_SPEC>;
#[doc = "UHS_dbg_sel"]
pub mod uhs_dbg_sel;
#[doc = "uhs_dummy_reg (rw) register accessor: an alias for `Reg<UHS_DUMMY_REG_SPEC>`"]
pub type UHS_DUMMY_REG = crate::Reg<uhs_dummy_reg::UHS_DUMMY_REG_SPEC>;
#[doc = "UHS_dummy_reg"]
pub mod uhs_dummy_reg;
#[doc = "phy_cfg_00 (rw) register accessor: an alias for `Reg<PHY_CFG_00_SPEC>`"]
pub type PHY_CFG_00 = crate::Reg<phy_cfg_00::PHY_CFG_00_SPEC>;
#[doc = "phy_cfg_00"]
pub mod phy_cfg_00;
#[doc = "phy_cfg_04 (rw) register accessor: an alias for `Reg<PHY_CFG_04_SPEC>`"]
pub type PHY_CFG_04 = crate::Reg<phy_cfg_04::PHY_CFG_04_SPEC>;
#[doc = "phy_cfg_04"]
pub mod phy_cfg_04;
#[doc = "phy_cfg_08 (rw) register accessor: an alias for `Reg<PHY_CFG_08_SPEC>`"]
pub type PHY_CFG_08 = crate::Reg<phy_cfg_08::PHY_CFG_08_SPEC>;
#[doc = "phy_cfg_08"]
pub mod phy_cfg_08;
#[doc = "phy_cfg_0c (rw) register accessor: an alias for `Reg<PHY_CFG_0C_SPEC>`"]
pub type PHY_CFG_0C = crate::Reg<phy_cfg_0c::PHY_CFG_0C_SPEC>;
#[doc = "phy_cfg_0C"]
pub mod phy_cfg_0c;
#[doc = "phy_cfg_10 (rw) register accessor: an alias for `Reg<PHY_CFG_10_SPEC>`"]
pub type PHY_CFG_10 = crate::Reg<phy_cfg_10::PHY_CFG_10_SPEC>;
#[doc = "phy_cfg_10"]
pub mod phy_cfg_10;
#[doc = "phy_cfg_14 (rw) register accessor: an alias for `Reg<PHY_CFG_14_SPEC>`"]
pub type PHY_CFG_14 = crate::Reg<phy_cfg_14::PHY_CFG_14_SPEC>;
#[doc = "phy_cfg_14"]
pub mod phy_cfg_14;
#[doc = "phy_cfg_18 (rw) register accessor: an alias for `Reg<PHY_CFG_18_SPEC>`"]
pub type PHY_CFG_18 = crate::Reg<phy_cfg_18::PHY_CFG_18_SPEC>;
#[doc = "phy_cfg_18"]
pub mod phy_cfg_18;
#[doc = "phy_cfg_1c (rw) register accessor: an alias for `Reg<PHY_CFG_1C_SPEC>`"]
pub type PHY_CFG_1C = crate::Reg<phy_cfg_1c::PHY_CFG_1C_SPEC>;
#[doc = "phy_cfg_1C"]
pub mod phy_cfg_1c;
#[doc = "phy_cfg_20 (rw) register accessor: an alias for `Reg<PHY_CFG_20_SPEC>`"]
pub type PHY_CFG_20 = crate::Reg<phy_cfg_20::PHY_CFG_20_SPEC>;
#[doc = "phy_cfg_20"]
pub mod phy_cfg_20;
#[doc = "phy_cfg_24 (rw) register accessor: an alias for `Reg<PHY_CFG_24_SPEC>`"]
pub type PHY_CFG_24 = crate::Reg<phy_cfg_24::PHY_CFG_24_SPEC>;
#[doc = "phy_cfg_24"]
pub mod phy_cfg_24;
#[doc = "phy_cfg_28 (rw) register accessor: an alias for `Reg<PHY_CFG_28_SPEC>`"]
pub type PHY_CFG_28 = crate::Reg<phy_cfg_28::PHY_CFG_28_SPEC>;
#[doc = "phy_cfg_28"]
pub mod phy_cfg_28;
#[doc = "phy_cfg_2c (rw) register accessor: an alias for `Reg<PHY_CFG_2C_SPEC>`"]
pub type PHY_CFG_2C = crate::Reg<phy_cfg_2c::PHY_CFG_2C_SPEC>;
#[doc = "phy_cfg_2C"]
pub mod phy_cfg_2c;
#[doc = "phy_cfg_30 (rw) register accessor: an alias for `Reg<PHY_CFG_30_SPEC>`"]
pub type PHY_CFG_30 = crate::Reg<phy_cfg_30::PHY_CFG_30_SPEC>;
#[doc = "phy_cfg_30"]
pub mod phy_cfg_30;
#[doc = "phy_cfg_34 (rw) register accessor: an alias for `Reg<PHY_CFG_34_SPEC>`"]
pub type PHY_CFG_34 = crate::Reg<phy_cfg_34::PHY_CFG_34_SPEC>;
#[doc = "phy_cfg_34"]
pub mod phy_cfg_34;
#[doc = "phy_cfg_38 (rw) register accessor: an alias for `Reg<PHY_CFG_38_SPEC>`"]
pub type PHY_CFG_38 = crate::Reg<phy_cfg_38::PHY_CFG_38_SPEC>;
#[doc = "phy_cfg_38"]
pub mod phy_cfg_38;
#[doc = "phy_cfg_3c (rw) register accessor: an alias for `Reg<PHY_CFG_3C_SPEC>`"]
pub type PHY_CFG_3C = crate::Reg<phy_cfg_3c::PHY_CFG_3C_SPEC>;
#[doc = "phy_cfg_3C"]
pub mod phy_cfg_3c;
#[doc = "phy_cfg_40 (rw) register accessor: an alias for `Reg<PHY_CFG_40_SPEC>`"]
pub type PHY_CFG_40 = crate::Reg<phy_cfg_40::PHY_CFG_40_SPEC>;
#[doc = "phy_cfg_40"]
pub mod phy_cfg_40;
#[doc = "phy_cfg_44 (rw) register accessor: an alias for `Reg<PHY_CFG_44_SPEC>`"]
pub type PHY_CFG_44 = crate::Reg<phy_cfg_44::PHY_CFG_44_SPEC>;
#[doc = "phy_cfg_44"]
pub mod phy_cfg_44;
#[doc = "phy_cfg_48 (rw) register accessor: an alias for `Reg<PHY_CFG_48_SPEC>`"]
pub type PHY_CFG_48 = crate::Reg<phy_cfg_48::PHY_CFG_48_SPEC>;
#[doc = "phy_cfg_48"]
pub mod phy_cfg_48;
#[doc = "phy_cfg_4c (rw) register accessor: an alias for `Reg<PHY_CFG_4C_SPEC>`"]
pub type PHY_CFG_4C = crate::Reg<phy_cfg_4c::PHY_CFG_4C_SPEC>;
#[doc = "phy_cfg_4C"]
pub mod phy_cfg_4c;
#[doc = "phy_cfg_50 (rw) register accessor: an alias for `Reg<PHY_CFG_50_SPEC>`"]
pub type PHY_CFG_50 = crate::Reg<phy_cfg_50::PHY_CFG_50_SPEC>;
#[doc = "phy_cfg_50"]
pub mod phy_cfg_50;
