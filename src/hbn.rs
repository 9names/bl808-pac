#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HBN_CTL"]
    pub rc32k_ctrl0: RC32K_CTRL0,
    _reserved1: [u8; 0x0200],
    #[doc = "0x204 - xtal32k"]
    pub xtal32k: XTAL32K,
    #[doc = "0x208 - rtc_rst_ctrl"]
    pub rtc_rst_ctrl: RTC_RST_CTRL,
}
#[doc = "rc32k_ctrl0 (rw) register accessor: an alias for `Reg<RC32K_CTRL0_SPEC>`"]
pub type RC32K_CTRL0 = crate::Reg<rc32k_ctrl0::RC32K_CTRL0_SPEC>;
#[doc = "HBN_CTL"]
pub mod rc32k_ctrl0;
#[doc = "xtal32k (rw) register accessor: an alias for `Reg<XTAL32K_SPEC>`"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "xtal32k"]
pub mod xtal32k;
#[doc = "rtc_rst_ctrl (rw) register accessor: an alias for `Reg<RTC_RST_CTRL_SPEC>`"]
pub type RTC_RST_CTRL = crate::Reg<rtc_rst_ctrl::RTC_RST_CTRL_SPEC>;
#[doc = "rtc_rst_ctrl"]
pub mod rtc_rst_ctrl;
