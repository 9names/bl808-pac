#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDS_CTL"]
    pub pds_stat: PDS_STAT,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - pds_ram1"]
    pub pds_ram1: PDS_RAM1,
    #[doc = "0x24 - PDS_CTL5"]
    pub pds_gpio_i_set: PDS_GPIO_I_SET,
    _reserved3: [u8; 0x0c],
    #[doc = "0x34 - pds_gpio_pd_set"]
    pub pds_gpio_pd_set: PDS_GPIO_PD_SET,
    _reserved4: [u8; 0x08],
    #[doc = "0x40 - pds_gpio_int"]
    pub pds_gpio_int: PDS_GPIO_INT,
    #[doc = "0x44 - pds_gpio_stat"]
    pub pds_gpio_stat: PDS_GPIO_STAT,
    _reserved6: [u8; 0xc8],
    #[doc = "0x110 - cpu_core_cfg0"]
    pub cpu_core_cfg0: CPU_CORE_CFG0,
    #[doc = "0x114 - cpu_core_cfg1"]
    pub cpu_core_cfg1: CPU_CORE_CFG1,
    _reserved8: [u8; 0x14],
    #[doc = "0x12c - cpu_core_cfg7"]
    pub cpu_core_cfg7: CPU_CORE_CFG7,
    #[doc = "0x130 - cpu_core_cfg8"]
    pub cpu_core_cfg8: CPU_CORE_CFG8,
    #[doc = "0x134 - cpu_core_cfg9"]
    pub cpu_core_cfg9: CPU_CORE_CFG9,
    #[doc = "0x138 - cpu_core_cfg10"]
    pub cpu_core_cfg10: CPU_CORE_CFG10,
    _reserved12: [u8; 0x04],
    #[doc = "0x140 - cpu_core_cfg12"]
    pub cpu_core_cfg12: CPU_CORE_CFG12,
    #[doc = "0x144 - cpu_core_cfg13"]
    pub cpu_core_cfg13: CPU_CORE_CFG13,
    #[doc = "0x148 - cpu_core_cfg14"]
    pub cpu_core_cfg14: CPU_CORE_CFG14,
    #[doc = "0x14c - tzc_pds"]
    pub tzc_pds: TZC_PDS,
    _reserved16: [u8; 0x01b0],
    #[doc = "0x300 - rc32m_ctrl0"]
    pub rc32m_ctrl0: RC32M_CTRL0,
    #[doc = "0x304 - rc32m_ctrl1"]
    pub rc32m_ctrl1: RC32M_CTRL1,
    _reserved18: [u8; 0xf8],
    #[doc = "0x400 - pu_rst_clkpll"]
    pub pu_rst_clkpll: PU_RST_CLKPLL,
    _reserved19: [u8; 0xfc],
    #[doc = "0x500 - usb_ctl"]
    pub usb_ctl: USB_CTL,
    #[doc = "0x504 - usb_phy_ctrl"]
    pub usb_phy_ctrl: USB_PHY_CTRL,
    _reserved21: [u8; 0x04f8],
    #[doc = "0xa00 - touch channel, clock, ana config1"]
    pub touch1: TOUCH1,
    #[doc = "0xa04 - touch channel, clock, ana config2"]
    pub touch2: TOUCH2,
    #[doc = "0xa08 - touch data process"]
    pub touch3: TOUCH3,
    #[doc = "0xa0c - Touch_sleep_time"]
    pub touch_data_hystersis: TOUCH_DATA_HYSTERSIS,
    _reserved25: [u8; 0x04],
    #[doc = "0xa14 - Channel_force_data_0"]
    pub touch_rsvd: TOUCH_RSVD,
    _reserved26: [u8; 0xb4],
    #[doc = "0xacc - touch_int_setting"]
    pub touch_int_setting: TOUCH_INT_SETTING,
}
#[doc = "pds_stat (rw) register accessor: an alias for `Reg<PDS_STAT_SPEC>`"]
pub type PDS_STAT = crate::Reg<pds_stat::PDS_STAT_SPEC>;
#[doc = "PDS_CTL"]
pub mod pds_stat;
#[doc = "pds_ram1 (rw) register accessor: an alias for `Reg<PDS_RAM1_SPEC>`"]
pub type PDS_RAM1 = crate::Reg<pds_ram1::PDS_RAM1_SPEC>;
#[doc = "pds_ram1"]
pub mod pds_ram1;
#[doc = "pds_gpio_i_set (rw) register accessor: an alias for `Reg<PDS_GPIO_I_SET_SPEC>`"]
pub type PDS_GPIO_I_SET = crate::Reg<pds_gpio_i_set::PDS_GPIO_I_SET_SPEC>;
#[doc = "PDS_CTL5"]
pub mod pds_gpio_i_set;
#[doc = "pds_gpio_pd_set (rw) register accessor: an alias for `Reg<PDS_GPIO_PD_SET_SPEC>`"]
pub type PDS_GPIO_PD_SET = crate::Reg<pds_gpio_pd_set::PDS_GPIO_PD_SET_SPEC>;
#[doc = "pds_gpio_pd_set"]
pub mod pds_gpio_pd_set;
#[doc = "pds_gpio_int (rw) register accessor: an alias for `Reg<PDS_GPIO_INT_SPEC>`"]
pub type PDS_GPIO_INT = crate::Reg<pds_gpio_int::PDS_GPIO_INT_SPEC>;
#[doc = "pds_gpio_int"]
pub mod pds_gpio_int;
#[doc = "pds_gpio_stat (rw) register accessor: an alias for `Reg<PDS_GPIO_STAT_SPEC>`"]
pub type PDS_GPIO_STAT = crate::Reg<pds_gpio_stat::PDS_GPIO_STAT_SPEC>;
#[doc = "pds_gpio_stat"]
pub mod pds_gpio_stat;
#[doc = "cpu_core_cfg0 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG0_SPEC>`"]
pub type CPU_CORE_CFG0 = crate::Reg<cpu_core_cfg0::CPU_CORE_CFG0_SPEC>;
#[doc = "cpu_core_cfg0"]
pub mod cpu_core_cfg0;
#[doc = "cpu_core_cfg1 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG1_SPEC>`"]
pub type CPU_CORE_CFG1 = crate::Reg<cpu_core_cfg1::CPU_CORE_CFG1_SPEC>;
#[doc = "cpu_core_cfg1"]
pub mod cpu_core_cfg1;
#[doc = "cpu_core_cfg7 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG7_SPEC>`"]
pub type CPU_CORE_CFG7 = crate::Reg<cpu_core_cfg7::CPU_CORE_CFG7_SPEC>;
#[doc = "cpu_core_cfg7"]
pub mod cpu_core_cfg7;
#[doc = "cpu_core_cfg8 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG8_SPEC>`"]
pub type CPU_CORE_CFG8 = crate::Reg<cpu_core_cfg8::CPU_CORE_CFG8_SPEC>;
#[doc = "cpu_core_cfg8"]
pub mod cpu_core_cfg8;
#[doc = "cpu_core_cfg9 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG9_SPEC>`"]
pub type CPU_CORE_CFG9 = crate::Reg<cpu_core_cfg9::CPU_CORE_CFG9_SPEC>;
#[doc = "cpu_core_cfg9"]
pub mod cpu_core_cfg9;
#[doc = "cpu_core_cfg10 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG10_SPEC>`"]
pub type CPU_CORE_CFG10 = crate::Reg<cpu_core_cfg10::CPU_CORE_CFG10_SPEC>;
#[doc = "cpu_core_cfg10"]
pub mod cpu_core_cfg10;
#[doc = "cpu_core_cfg12 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG12_SPEC>`"]
pub type CPU_CORE_CFG12 = crate::Reg<cpu_core_cfg12::CPU_CORE_CFG12_SPEC>;
#[doc = "cpu_core_cfg12"]
pub mod cpu_core_cfg12;
#[doc = "cpu_core_cfg13 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG13_SPEC>`"]
pub type CPU_CORE_CFG13 = crate::Reg<cpu_core_cfg13::CPU_CORE_CFG13_SPEC>;
#[doc = "cpu_core_cfg13"]
pub mod cpu_core_cfg13;
#[doc = "cpu_core_cfg14 (rw) register accessor: an alias for `Reg<CPU_CORE_CFG14_SPEC>`"]
pub type CPU_CORE_CFG14 = crate::Reg<cpu_core_cfg14::CPU_CORE_CFG14_SPEC>;
#[doc = "cpu_core_cfg14"]
pub mod cpu_core_cfg14;
#[doc = "tzc_pds (rw) register accessor: an alias for `Reg<TZC_PDS_SPEC>`"]
pub type TZC_PDS = crate::Reg<tzc_pds::TZC_PDS_SPEC>;
#[doc = "tzc_pds"]
pub mod tzc_pds;
#[doc = "rc32m_ctrl0 (rw) register accessor: an alias for `Reg<RC32M_CTRL0_SPEC>`"]
pub type RC32M_CTRL0 = crate::Reg<rc32m_ctrl0::RC32M_CTRL0_SPEC>;
#[doc = "rc32m_ctrl0"]
pub mod rc32m_ctrl0;
#[doc = "rc32m_ctrl1 (rw) register accessor: an alias for `Reg<RC32M_CTRL1_SPEC>`"]
pub type RC32M_CTRL1 = crate::Reg<rc32m_ctrl1::RC32M_CTRL1_SPEC>;
#[doc = "rc32m_ctrl1"]
pub mod rc32m_ctrl1;
#[doc = "pu_rst_clkpll (rw) register accessor: an alias for `Reg<PU_RST_CLKPLL_SPEC>`"]
pub type PU_RST_CLKPLL = crate::Reg<pu_rst_clkpll::PU_RST_CLKPLL_SPEC>;
#[doc = "pu_rst_clkpll"]
pub mod pu_rst_clkpll;
#[doc = "usb_ctl (rw) register accessor: an alias for `Reg<USB_CTL_SPEC>`"]
pub type USB_CTL = crate::Reg<usb_ctl::USB_CTL_SPEC>;
#[doc = "usb_ctl"]
pub mod usb_ctl;
#[doc = "usb_phy_ctrl (rw) register accessor: an alias for `Reg<USB_PHY_CTRL_SPEC>`"]
pub type USB_PHY_CTRL = crate::Reg<usb_phy_ctrl::USB_PHY_CTRL_SPEC>;
#[doc = "usb_phy_ctrl"]
pub mod usb_phy_ctrl;
#[doc = "touch1 (rw) register accessor: an alias for `Reg<TOUCH1_SPEC>`"]
pub type TOUCH1 = crate::Reg<touch1::TOUCH1_SPEC>;
#[doc = "touch channel, clock, ana config1"]
pub mod touch1;
#[doc = "touch2 (rw) register accessor: an alias for `Reg<TOUCH2_SPEC>`"]
pub type TOUCH2 = crate::Reg<touch2::TOUCH2_SPEC>;
#[doc = "touch channel, clock, ana config2"]
pub mod touch2;
#[doc = "touch3 (rw) register accessor: an alias for `Reg<TOUCH3_SPEC>`"]
pub type TOUCH3 = crate::Reg<touch3::TOUCH3_SPEC>;
#[doc = "touch data process"]
pub mod touch3;
#[doc = "touch_data_hystersis (rw) register accessor: an alias for `Reg<TOUCH_DATA_HYSTERSIS_SPEC>`"]
pub type TOUCH_DATA_HYSTERSIS = crate::Reg<touch_data_hystersis::TOUCH_DATA_HYSTERSIS_SPEC>;
#[doc = "Touch_sleep_time"]
pub mod touch_data_hystersis;
#[doc = "touch_rsvd (rw) register accessor: an alias for `Reg<TOUCH_RSVD_SPEC>`"]
pub type TOUCH_RSVD = crate::Reg<touch_rsvd::TOUCH_RSVD_SPEC>;
#[doc = "Channel_force_data_0"]
pub mod touch_rsvd;
#[doc = "touch_int_setting (rw) register accessor: an alias for `Reg<TOUCH_INT_SETTING_SPEC>`"]
pub type TOUCH_INT_SETTING = crate::Reg<touch_int_setting::TOUCH_INT_SETTING_SPEC>;
#[doc = "touch_int_setting"]
pub mod touch_int_setting;
