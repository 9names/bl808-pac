#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDS_CTL"]
    pub pds_ctl: PDS_CTL,
    #[doc = "0x04 - PDS_TIME1"]
    pub pds_time1: PDS_TIME1,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - PDS_INT"]
    pub pds_int: PDS_INT,
    #[doc = "0x10 - PDS_CTL2"]
    pub pds_ctl2: PDS_CTL2,
    #[doc = "0x14 - PDS_CTL3"]
    pub pds_ctl3: PDS_CTL3,
    #[doc = "0x18 - PDS_CTL4"]
    pub pds_ctl4: PDS_CTL4,
    #[doc = "0x1c - pds_stat"]
    pub pds_stat: PDS_STAT,
    #[doc = "0x20 - pds_ram1"]
    pub pds_ram1: PDS_RAM1,
    #[doc = "0x24 - PDS_CTL5"]
    pub pds_ctl5: PDS_CTL5,
    #[doc = "0x28 - PDS_RAM2"]
    pub pds_ram2: PDS_RAM2,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - pds_gpio_i_set"]
    pub pds_gpio_i_set: PDS_GPIO_I_SET,
    #[doc = "0x34 - pds_gpio_pd_set"]
    pub pds_gpio_pd_set: PDS_GPIO_PD_SET,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - pds_gpio_int"]
    pub pds_gpio_int: PDS_GPIO_INT,
    #[doc = "0x44 - pds_gpio_stat"]
    pub pds_gpio_stat: PDS_GPIO_STAT,
    _reserved14: [u8; 0xc8],
    #[doc = "0x110 - cpu_core_cfg0"]
    pub cpu_core_cfg0: CPU_CORE_CFG0,
    #[doc = "0x114 - cpu_core_cfg1"]
    pub cpu_core_cfg1: CPU_CORE_CFG1,
    _reserved16: [u8; 0x14],
    #[doc = "0x12c - cpu_core_cfg7"]
    pub cpu_core_cfg7: CPU_CORE_CFG7,
    #[doc = "0x130 - cpu_core_cfg8"]
    pub cpu_core_cfg8: CPU_CORE_CFG8,
    #[doc = "0x134 - cpu_core_cfg9"]
    pub cpu_core_cfg9: CPU_CORE_CFG9,
    #[doc = "0x138 - cpu_core_cfg10"]
    pub cpu_core_cfg10: CPU_CORE_CFG10,
    _reserved20: [u8; 0x04],
    #[doc = "0x140 - cpu_core_cfg12"]
    pub cpu_core_cfg12: CPU_CORE_CFG12,
    #[doc = "0x144 - cpu_core_cfg13"]
    pub cpu_core_cfg13: CPU_CORE_CFG13,
    #[doc = "0x148 - cpu_core_cfg14"]
    pub cpu_core_cfg14: CPU_CORE_CFG14,
    #[doc = "0x14c - tzc_pds"]
    pub tzc_pds: TZC_PDS,
    _reserved24: [u8; 0x01b0],
    #[doc = "0x300 - rc32m_ctrl0"]
    pub rc32m_ctrl0: RC32M_CTRL0,
    #[doc = "0x304 - rc32m_ctrl1"]
    pub rc32m_ctrl1: RC32M_CTRL1,
    _reserved26: [u8; 0xf8],
    #[doc = "0x400 - pu_rst_clkpll"]
    pub pu_rst_clkpll: PU_RST_CLKPLL,
    _reserved27: [u8; 0xfc],
    #[doc = "0x500 - usb_ctl"]
    pub usb_ctl: USB_CTL,
    #[doc = "0x504 - usb_phy_ctrl"]
    pub usb_phy_ctrl: USB_PHY_CTRL,
    _reserved29: [u8; 0x04f8],
    #[doc = "0xa00 - touch channel, clock, ana config1"]
    pub touch1: TOUCH1,
    #[doc = "0xa04 - touch channel, clock, ana config2"]
    pub touch2: TOUCH2,
    #[doc = "0xa08 - touch data process"]
    pub touch3: TOUCH3,
    #[doc = "0xa0c - Touch_sleep_time"]
    pub touch_sleep_time: TOUCH_SLEEP_TIME,
    #[doc = "0xa10 - touch_data_hystersis"]
    pub touch_data_hystersis: TOUCH_DATA_HYSTERSIS,
    #[doc = "0xa14 - Channel_force_data_0"]
    pub channel_force_data_0: CHANNEL_FORCE_DATA_0,
    #[doc = "0xa18 - Channel_force_data_1"]
    pub channel_force_data_1: CHANNEL_FORCE_DATA_1,
    #[doc = "0xa1c - Channel_force_data_2"]
    pub channel_force_data_2: CHANNEL_FORCE_DATA_2,
    #[doc = "0xa20 - Channel_force_data_3"]
    pub channel_force_data_3: CHANNEL_FORCE_DATA_3,
    #[doc = "0xa24 - Channel_force_data_4"]
    pub channel_force_data_4: CHANNEL_FORCE_DATA_4,
    #[doc = "0xa28 - Channel_force_data_5"]
    pub channel_force_data_5: CHANNEL_FORCE_DATA_5,
    #[doc = "0xa2c - Channel_vth_data_0"]
    pub channel_vth_data_0: CHANNEL_VTH_DATA_0,
    #[doc = "0xa30 - Channel_vth_data_1"]
    pub channel_vth_data_1: CHANNEL_VTH_DATA_1,
    #[doc = "0xa34 - Channel_vth_data_2"]
    pub channel_vth_data_2: CHANNEL_VTH_DATA_2,
    #[doc = "0xa38 - Channel_raw_data_0"]
    pub channel_raw_data_0: CHANNEL_RAW_DATA_0,
    #[doc = "0xa3c - Channel_raw_data_1"]
    pub channel_raw_data_1: CHANNEL_RAW_DATA_1,
    #[doc = "0xa40 - Channel_raw_data_2"]
    pub channel_raw_data_2: CHANNEL_RAW_DATA_2,
    #[doc = "0xa44 - Channel_raw_data_3"]
    pub channel_raw_data_3: CHANNEL_RAW_DATA_3,
    #[doc = "0xa48 - Channel_raw_data_4"]
    pub channel_raw_data_4: CHANNEL_RAW_DATA_4,
    #[doc = "0xa4c - Channel_raw_data_5"]
    pub channel_raw_data_5: CHANNEL_RAW_DATA_5,
    #[doc = "0xa50 - Channel_raw_data_6"]
    pub channel_raw_data_6: CHANNEL_RAW_DATA_6,
    #[doc = "0xa54 - Channel_raw_data_7"]
    pub channel_raw_data_7: CHANNEL_RAW_DATA_7,
    #[doc = "0xa58 - Channel_raw_data_8"]
    pub channel_raw_data_8: CHANNEL_RAW_DATA_8,
    #[doc = "0xa5c - Channel_raw_data_9"]
    pub channel_raw_data_9: CHANNEL_RAW_DATA_9,
    #[doc = "0xa60 - Channel_raw_data_10"]
    pub channel_raw_data_10: CHANNEL_RAW_DATA_10,
    #[doc = "0xa64 - Channel_raw_data_11"]
    pub channel_raw_data_11: CHANNEL_RAW_DATA_11,
    #[doc = "0xa68 - Channel_LTA_data_0"]
    pub channel_lta_data_0: CHANNEL_LTA_DATA_0,
    #[doc = "0xa6c - Channel_LTA_data_1"]
    pub channel_lta_data_1: CHANNEL_LTA_DATA_1,
    #[doc = "0xa70 - Channel_LTA_data_2"]
    pub channel_lta_data_2: CHANNEL_LTA_DATA_2,
    #[doc = "0xa74 - Channel_LTA_data_3"]
    pub channel_lta_data_3: CHANNEL_LTA_DATA_3,
    #[doc = "0xa78 - Channel_LTA_data_4"]
    pub channel_lta_data_4: CHANNEL_LTA_DATA_4,
    #[doc = "0xa7c - Channel_LTA_data_5"]
    pub channel_lta_data_5: CHANNEL_LTA_DATA_5,
    #[doc = "0xa80 - Channel_LTA_data_6"]
    pub channel_lta_data_6: CHANNEL_LTA_DATA_6,
    #[doc = "0xa84 - Channel_LTA_data_7"]
    pub channel_lta_data_7: CHANNEL_LTA_DATA_7,
    #[doc = "0xa88 - Channel_LTA_data_8"]
    pub channel_lta_data_8: CHANNEL_LTA_DATA_8,
    #[doc = "0xa8c - Channel_LTA_data_9"]
    pub channel_lta_data_9: CHANNEL_LTA_DATA_9,
    #[doc = "0xa90 - Channel_LTA_data_10"]
    pub channel_lta_data_10: CHANNEL_LTA_DATA_10,
    #[doc = "0xa94 - Channel_LTA_data_11"]
    pub channel_lta_data_11: CHANNEL_LTA_DATA_11,
    #[doc = "0xa98 - Channel_FLT_data_0"]
    pub channel_flt_data_0: CHANNEL_FLT_DATA_0,
    #[doc = "0xa9c - Channel_FLT_data_1"]
    pub channel_flt_data_1: CHANNEL_FLT_DATA_1,
    #[doc = "0xaa0 - Channel_FLT_data_2"]
    pub channel_flt_data_2: CHANNEL_FLT_DATA_2,
    #[doc = "0xaa4 - Channel_FLT_data_3"]
    pub channel_flt_data_3: CHANNEL_FLT_DATA_3,
    #[doc = "0xaa8 - Channel_FLT_data_4"]
    pub channel_flt_data_4: CHANNEL_FLT_DATA_4,
    #[doc = "0xaac - Channel_FLT_data_5"]
    pub channel_flt_data_5: CHANNEL_FLT_DATA_5,
    #[doc = "0xab0 - Channel_FLT_data_6"]
    pub channel_flt_data_6: CHANNEL_FLT_DATA_6,
    #[doc = "0xab4 - Channel_FLT_data_7"]
    pub channel_flt_data_7: CHANNEL_FLT_DATA_7,
    #[doc = "0xab8 - Channel_FLT_data_8"]
    pub channel_flt_data_8: CHANNEL_FLT_DATA_8,
    #[doc = "0xabc - Channel_FLT_data_9"]
    pub channel_flt_data_9: CHANNEL_FLT_DATA_9,
    #[doc = "0xac0 - Channel_FLT_data_10"]
    pub channel_flt_data_10: CHANNEL_FLT_DATA_10,
    #[doc = "0xac4 - Channel_FLT_data_11"]
    pub channel_flt_data_11: CHANNEL_FLT_DATA_11,
    #[doc = "0xac8 - touch_rsvd"]
    pub touch_rsvd: TOUCH_RSVD,
    #[doc = "0xacc - touch_int_setting"]
    pub touch_int_setting: TOUCH_INT_SETTING,
    #[doc = "0xad0 - touch_int_status"]
    pub touch_int_status: TOUCH_INT_STATUS,
}
#[doc = "pds_ctl (rw) register accessor: an alias for `Reg<PDS_CTL_SPEC>`"]
pub type PDS_CTL = crate::Reg<pds_ctl::PDS_CTL_SPEC>;
#[doc = "PDS_CTL"]
pub mod pds_ctl;
#[doc = "pds_time1 (rw) register accessor: an alias for `Reg<PDS_TIME1_SPEC>`"]
pub type PDS_TIME1 = crate::Reg<pds_time1::PDS_TIME1_SPEC>;
#[doc = "PDS_TIME1"]
pub mod pds_time1;
#[doc = "pds_int (rw) register accessor: an alias for `Reg<PDS_INT_SPEC>`"]
pub type PDS_INT = crate::Reg<pds_int::PDS_INT_SPEC>;
#[doc = "PDS_INT"]
pub mod pds_int;
#[doc = "pds_ctl2 (rw) register accessor: an alias for `Reg<PDS_CTL2_SPEC>`"]
pub type PDS_CTL2 = crate::Reg<pds_ctl2::PDS_CTL2_SPEC>;
#[doc = "PDS_CTL2"]
pub mod pds_ctl2;
#[doc = "pds_ctl3 (rw) register accessor: an alias for `Reg<PDS_CTL3_SPEC>`"]
pub type PDS_CTL3 = crate::Reg<pds_ctl3::PDS_CTL3_SPEC>;
#[doc = "PDS_CTL3"]
pub mod pds_ctl3;
#[doc = "pds_ctl4 (rw) register accessor: an alias for `Reg<PDS_CTL4_SPEC>`"]
pub type PDS_CTL4 = crate::Reg<pds_ctl4::PDS_CTL4_SPEC>;
#[doc = "PDS_CTL4"]
pub mod pds_ctl4;
#[doc = "pds_stat (rw) register accessor: an alias for `Reg<PDS_STAT_SPEC>`"]
pub type PDS_STAT = crate::Reg<pds_stat::PDS_STAT_SPEC>;
#[doc = "pds_stat"]
pub mod pds_stat;
#[doc = "pds_ram1 (rw) register accessor: an alias for `Reg<PDS_RAM1_SPEC>`"]
pub type PDS_RAM1 = crate::Reg<pds_ram1::PDS_RAM1_SPEC>;
#[doc = "pds_ram1"]
pub mod pds_ram1;
#[doc = "pds_ctl5 (rw) register accessor: an alias for `Reg<PDS_CTL5_SPEC>`"]
pub type PDS_CTL5 = crate::Reg<pds_ctl5::PDS_CTL5_SPEC>;
#[doc = "PDS_CTL5"]
pub mod pds_ctl5;
#[doc = "pds_ram2 (rw) register accessor: an alias for `Reg<PDS_RAM2_SPEC>`"]
pub type PDS_RAM2 = crate::Reg<pds_ram2::PDS_RAM2_SPEC>;
#[doc = "PDS_RAM2"]
pub mod pds_ram2;
#[doc = "pds_gpio_i_set (rw) register accessor: an alias for `Reg<PDS_GPIO_I_SET_SPEC>`"]
pub type PDS_GPIO_I_SET = crate::Reg<pds_gpio_i_set::PDS_GPIO_I_SET_SPEC>;
#[doc = "pds_gpio_i_set"]
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
#[doc = "touch_sleep_time (rw) register accessor: an alias for `Reg<TOUCH_SLEEP_TIME_SPEC>`"]
pub type TOUCH_SLEEP_TIME = crate::Reg<touch_sleep_time::TOUCH_SLEEP_TIME_SPEC>;
#[doc = "Touch_sleep_time"]
pub mod touch_sleep_time;
#[doc = "touch_data_hystersis (rw) register accessor: an alias for `Reg<TOUCH_DATA_HYSTERSIS_SPEC>`"]
pub type TOUCH_DATA_HYSTERSIS = crate::Reg<touch_data_hystersis::TOUCH_DATA_HYSTERSIS_SPEC>;
#[doc = "touch_data_hystersis"]
pub mod touch_data_hystersis;
#[doc = "channel_force_data_0 (rw) register accessor: an alias for `Reg<CHANNEL_FORCE_DATA_0_SPEC>`"]
pub type CHANNEL_FORCE_DATA_0 = crate::Reg<channel_force_data_0::CHANNEL_FORCE_DATA_0_SPEC>;
#[doc = "Channel_force_data_0"]
pub mod channel_force_data_0;
#[doc = "channel_force_data_1 (rw) register accessor: an alias for `Reg<CHANNEL_FORCE_DATA_1_SPEC>`"]
pub type CHANNEL_FORCE_DATA_1 = crate::Reg<channel_force_data_1::CHANNEL_FORCE_DATA_1_SPEC>;
#[doc = "Channel_force_data_1"]
pub mod channel_force_data_1;
#[doc = "channel_force_data_2 (rw) register accessor: an alias for `Reg<CHANNEL_FORCE_DATA_2_SPEC>`"]
pub type CHANNEL_FORCE_DATA_2 = crate::Reg<channel_force_data_2::CHANNEL_FORCE_DATA_2_SPEC>;
#[doc = "Channel_force_data_2"]
pub mod channel_force_data_2;
#[doc = "channel_force_data_3 (rw) register accessor: an alias for `Reg<CHANNEL_FORCE_DATA_3_SPEC>`"]
pub type CHANNEL_FORCE_DATA_3 = crate::Reg<channel_force_data_3::CHANNEL_FORCE_DATA_3_SPEC>;
#[doc = "Channel_force_data_3"]
pub mod channel_force_data_3;
#[doc = "channel_force_data_4 (rw) register accessor: an alias for `Reg<CHANNEL_FORCE_DATA_4_SPEC>`"]
pub type CHANNEL_FORCE_DATA_4 = crate::Reg<channel_force_data_4::CHANNEL_FORCE_DATA_4_SPEC>;
#[doc = "Channel_force_data_4"]
pub mod channel_force_data_4;
#[doc = "channel_force_data_5 (rw) register accessor: an alias for `Reg<CHANNEL_FORCE_DATA_5_SPEC>`"]
pub type CHANNEL_FORCE_DATA_5 = crate::Reg<channel_force_data_5::CHANNEL_FORCE_DATA_5_SPEC>;
#[doc = "Channel_force_data_5"]
pub mod channel_force_data_5;
#[doc = "channel_vth_data_0 (rw) register accessor: an alias for `Reg<CHANNEL_VTH_DATA_0_SPEC>`"]
pub type CHANNEL_VTH_DATA_0 = crate::Reg<channel_vth_data_0::CHANNEL_VTH_DATA_0_SPEC>;
#[doc = "Channel_vth_data_0"]
pub mod channel_vth_data_0;
#[doc = "channel_vth_data_1 (rw) register accessor: an alias for `Reg<CHANNEL_VTH_DATA_1_SPEC>`"]
pub type CHANNEL_VTH_DATA_1 = crate::Reg<channel_vth_data_1::CHANNEL_VTH_DATA_1_SPEC>;
#[doc = "Channel_vth_data_1"]
pub mod channel_vth_data_1;
#[doc = "channel_vth_data_2 (rw) register accessor: an alias for `Reg<CHANNEL_VTH_DATA_2_SPEC>`"]
pub type CHANNEL_VTH_DATA_2 = crate::Reg<channel_vth_data_2::CHANNEL_VTH_DATA_2_SPEC>;
#[doc = "Channel_vth_data_2"]
pub mod channel_vth_data_2;
#[doc = "channel_raw_data_0 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_0_SPEC>`"]
pub type CHANNEL_RAW_DATA_0 = crate::Reg<channel_raw_data_0::CHANNEL_RAW_DATA_0_SPEC>;
#[doc = "Channel_raw_data_0"]
pub mod channel_raw_data_0;
#[doc = "channel_raw_data_1 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_1_SPEC>`"]
pub type CHANNEL_RAW_DATA_1 = crate::Reg<channel_raw_data_1::CHANNEL_RAW_DATA_1_SPEC>;
#[doc = "Channel_raw_data_1"]
pub mod channel_raw_data_1;
#[doc = "channel_raw_data_2 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_2_SPEC>`"]
pub type CHANNEL_RAW_DATA_2 = crate::Reg<channel_raw_data_2::CHANNEL_RAW_DATA_2_SPEC>;
#[doc = "Channel_raw_data_2"]
pub mod channel_raw_data_2;
#[doc = "channel_raw_data_3 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_3_SPEC>`"]
pub type CHANNEL_RAW_DATA_3 = crate::Reg<channel_raw_data_3::CHANNEL_RAW_DATA_3_SPEC>;
#[doc = "Channel_raw_data_3"]
pub mod channel_raw_data_3;
#[doc = "channel_raw_data_4 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_4_SPEC>`"]
pub type CHANNEL_RAW_DATA_4 = crate::Reg<channel_raw_data_4::CHANNEL_RAW_DATA_4_SPEC>;
#[doc = "Channel_raw_data_4"]
pub mod channel_raw_data_4;
#[doc = "channel_raw_data_5 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_5_SPEC>`"]
pub type CHANNEL_RAW_DATA_5 = crate::Reg<channel_raw_data_5::CHANNEL_RAW_DATA_5_SPEC>;
#[doc = "Channel_raw_data_5"]
pub mod channel_raw_data_5;
#[doc = "channel_raw_data_6 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_6_SPEC>`"]
pub type CHANNEL_RAW_DATA_6 = crate::Reg<channel_raw_data_6::CHANNEL_RAW_DATA_6_SPEC>;
#[doc = "Channel_raw_data_6"]
pub mod channel_raw_data_6;
#[doc = "channel_raw_data_7 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_7_SPEC>`"]
pub type CHANNEL_RAW_DATA_7 = crate::Reg<channel_raw_data_7::CHANNEL_RAW_DATA_7_SPEC>;
#[doc = "Channel_raw_data_7"]
pub mod channel_raw_data_7;
#[doc = "channel_raw_data_8 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_8_SPEC>`"]
pub type CHANNEL_RAW_DATA_8 = crate::Reg<channel_raw_data_8::CHANNEL_RAW_DATA_8_SPEC>;
#[doc = "Channel_raw_data_8"]
pub mod channel_raw_data_8;
#[doc = "channel_raw_data_9 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_9_SPEC>`"]
pub type CHANNEL_RAW_DATA_9 = crate::Reg<channel_raw_data_9::CHANNEL_RAW_DATA_9_SPEC>;
#[doc = "Channel_raw_data_9"]
pub mod channel_raw_data_9;
#[doc = "channel_raw_data_10 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_10_SPEC>`"]
pub type CHANNEL_RAW_DATA_10 = crate::Reg<channel_raw_data_10::CHANNEL_RAW_DATA_10_SPEC>;
#[doc = "Channel_raw_data_10"]
pub mod channel_raw_data_10;
#[doc = "channel_raw_data_11 (rw) register accessor: an alias for `Reg<CHANNEL_RAW_DATA_11_SPEC>`"]
pub type CHANNEL_RAW_DATA_11 = crate::Reg<channel_raw_data_11::CHANNEL_RAW_DATA_11_SPEC>;
#[doc = "Channel_raw_data_11"]
pub mod channel_raw_data_11;
#[doc = "channel_lta_data_0 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_0_SPEC>`"]
pub type CHANNEL_LTA_DATA_0 = crate::Reg<channel_lta_data_0::CHANNEL_LTA_DATA_0_SPEC>;
#[doc = "Channel_LTA_data_0"]
pub mod channel_lta_data_0;
#[doc = "channel_lta_data_1 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_1_SPEC>`"]
pub type CHANNEL_LTA_DATA_1 = crate::Reg<channel_lta_data_1::CHANNEL_LTA_DATA_1_SPEC>;
#[doc = "Channel_LTA_data_1"]
pub mod channel_lta_data_1;
#[doc = "channel_lta_data_2 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_2_SPEC>`"]
pub type CHANNEL_LTA_DATA_2 = crate::Reg<channel_lta_data_2::CHANNEL_LTA_DATA_2_SPEC>;
#[doc = "Channel_LTA_data_2"]
pub mod channel_lta_data_2;
#[doc = "channel_lta_data_3 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_3_SPEC>`"]
pub type CHANNEL_LTA_DATA_3 = crate::Reg<channel_lta_data_3::CHANNEL_LTA_DATA_3_SPEC>;
#[doc = "Channel_LTA_data_3"]
pub mod channel_lta_data_3;
#[doc = "channel_lta_data_4 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_4_SPEC>`"]
pub type CHANNEL_LTA_DATA_4 = crate::Reg<channel_lta_data_4::CHANNEL_LTA_DATA_4_SPEC>;
#[doc = "Channel_LTA_data_4"]
pub mod channel_lta_data_4;
#[doc = "channel_lta_data_5 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_5_SPEC>`"]
pub type CHANNEL_LTA_DATA_5 = crate::Reg<channel_lta_data_5::CHANNEL_LTA_DATA_5_SPEC>;
#[doc = "Channel_LTA_data_5"]
pub mod channel_lta_data_5;
#[doc = "channel_lta_data_6 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_6_SPEC>`"]
pub type CHANNEL_LTA_DATA_6 = crate::Reg<channel_lta_data_6::CHANNEL_LTA_DATA_6_SPEC>;
#[doc = "Channel_LTA_data_6"]
pub mod channel_lta_data_6;
#[doc = "channel_lta_data_7 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_7_SPEC>`"]
pub type CHANNEL_LTA_DATA_7 = crate::Reg<channel_lta_data_7::CHANNEL_LTA_DATA_7_SPEC>;
#[doc = "Channel_LTA_data_7"]
pub mod channel_lta_data_7;
#[doc = "channel_lta_data_8 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_8_SPEC>`"]
pub type CHANNEL_LTA_DATA_8 = crate::Reg<channel_lta_data_8::CHANNEL_LTA_DATA_8_SPEC>;
#[doc = "Channel_LTA_data_8"]
pub mod channel_lta_data_8;
#[doc = "channel_lta_data_9 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_9_SPEC>`"]
pub type CHANNEL_LTA_DATA_9 = crate::Reg<channel_lta_data_9::CHANNEL_LTA_DATA_9_SPEC>;
#[doc = "Channel_LTA_data_9"]
pub mod channel_lta_data_9;
#[doc = "channel_lta_data_10 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_10_SPEC>`"]
pub type CHANNEL_LTA_DATA_10 = crate::Reg<channel_lta_data_10::CHANNEL_LTA_DATA_10_SPEC>;
#[doc = "Channel_LTA_data_10"]
pub mod channel_lta_data_10;
#[doc = "channel_lta_data_11 (rw) register accessor: an alias for `Reg<CHANNEL_LTA_DATA_11_SPEC>`"]
pub type CHANNEL_LTA_DATA_11 = crate::Reg<channel_lta_data_11::CHANNEL_LTA_DATA_11_SPEC>;
#[doc = "Channel_LTA_data_11"]
pub mod channel_lta_data_11;
#[doc = "channel_flt_data_0 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_0_SPEC>`"]
pub type CHANNEL_FLT_DATA_0 = crate::Reg<channel_flt_data_0::CHANNEL_FLT_DATA_0_SPEC>;
#[doc = "Channel_FLT_data_0"]
pub mod channel_flt_data_0;
#[doc = "channel_flt_data_1 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_1_SPEC>`"]
pub type CHANNEL_FLT_DATA_1 = crate::Reg<channel_flt_data_1::CHANNEL_FLT_DATA_1_SPEC>;
#[doc = "Channel_FLT_data_1"]
pub mod channel_flt_data_1;
#[doc = "channel_flt_data_2 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_2_SPEC>`"]
pub type CHANNEL_FLT_DATA_2 = crate::Reg<channel_flt_data_2::CHANNEL_FLT_DATA_2_SPEC>;
#[doc = "Channel_FLT_data_2"]
pub mod channel_flt_data_2;
#[doc = "channel_flt_data_3 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_3_SPEC>`"]
pub type CHANNEL_FLT_DATA_3 = crate::Reg<channel_flt_data_3::CHANNEL_FLT_DATA_3_SPEC>;
#[doc = "Channel_FLT_data_3"]
pub mod channel_flt_data_3;
#[doc = "channel_flt_data_4 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_4_SPEC>`"]
pub type CHANNEL_FLT_DATA_4 = crate::Reg<channel_flt_data_4::CHANNEL_FLT_DATA_4_SPEC>;
#[doc = "Channel_FLT_data_4"]
pub mod channel_flt_data_4;
#[doc = "channel_flt_data_5 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_5_SPEC>`"]
pub type CHANNEL_FLT_DATA_5 = crate::Reg<channel_flt_data_5::CHANNEL_FLT_DATA_5_SPEC>;
#[doc = "Channel_FLT_data_5"]
pub mod channel_flt_data_5;
#[doc = "channel_flt_data_6 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_6_SPEC>`"]
pub type CHANNEL_FLT_DATA_6 = crate::Reg<channel_flt_data_6::CHANNEL_FLT_DATA_6_SPEC>;
#[doc = "Channel_FLT_data_6"]
pub mod channel_flt_data_6;
#[doc = "channel_flt_data_7 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_7_SPEC>`"]
pub type CHANNEL_FLT_DATA_7 = crate::Reg<channel_flt_data_7::CHANNEL_FLT_DATA_7_SPEC>;
#[doc = "Channel_FLT_data_7"]
pub mod channel_flt_data_7;
#[doc = "channel_flt_data_8 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_8_SPEC>`"]
pub type CHANNEL_FLT_DATA_8 = crate::Reg<channel_flt_data_8::CHANNEL_FLT_DATA_8_SPEC>;
#[doc = "Channel_FLT_data_8"]
pub mod channel_flt_data_8;
#[doc = "channel_flt_data_9 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_9_SPEC>`"]
pub type CHANNEL_FLT_DATA_9 = crate::Reg<channel_flt_data_9::CHANNEL_FLT_DATA_9_SPEC>;
#[doc = "Channel_FLT_data_9"]
pub mod channel_flt_data_9;
#[doc = "channel_flt_data_10 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_10_SPEC>`"]
pub type CHANNEL_FLT_DATA_10 = crate::Reg<channel_flt_data_10::CHANNEL_FLT_DATA_10_SPEC>;
#[doc = "Channel_FLT_data_10"]
pub mod channel_flt_data_10;
#[doc = "channel_flt_data_11 (rw) register accessor: an alias for `Reg<CHANNEL_FLT_DATA_11_SPEC>`"]
pub type CHANNEL_FLT_DATA_11 = crate::Reg<channel_flt_data_11::CHANNEL_FLT_DATA_11_SPEC>;
#[doc = "Channel_FLT_data_11"]
pub mod channel_flt_data_11;
#[doc = "touch_rsvd (rw) register accessor: an alias for `Reg<TOUCH_RSVD_SPEC>`"]
pub type TOUCH_RSVD = crate::Reg<touch_rsvd::TOUCH_RSVD_SPEC>;
#[doc = "touch_rsvd"]
pub mod touch_rsvd;
#[doc = "touch_int_setting (rw) register accessor: an alias for `Reg<TOUCH_INT_SETTING_SPEC>`"]
pub type TOUCH_INT_SETTING = crate::Reg<touch_int_setting::TOUCH_INT_SETTING_SPEC>;
#[doc = "touch_int_setting"]
pub mod touch_int_setting;
#[doc = "touch_int_status (rw) register accessor: an alias for `Reg<TOUCH_INT_STATUS_SPEC>`"]
pub type TOUCH_INT_STATUS = crate::Reg<touch_int_status::TOUCH_INT_STATUS_SPEC>;
#[doc = "touch_int_status"]
pub mod touch_int_status;
