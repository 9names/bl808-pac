#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cci_cfg"]
    pub cci_cfg: CCI_CFG,
    #[doc = "0x04 - cci_addr"]
    pub cci_addr: CCI_ADDR,
    #[doc = "0x08 - cci_wdata"]
    pub cci_wdata: CCI_WDATA,
    #[doc = "0x0c - cci_rdata"]
    pub cci_rdata: CCI_RDATA,
    #[doc = "0x10 - cci_ctl"]
    pub cci_ctl: CCI_CTL,
    _reserved5: [u8; 0x073c],
    #[doc = "0x750 - audio_pll_cfg0"]
    pub audio_pll_cfg0: AUDIO_PLL_CFG0,
    #[doc = "0x754 - audio_pll_cfg1"]
    pub audio_pll_cfg1: AUDIO_PLL_CFG1,
    #[doc = "0x758 - audio_pll_cfg2"]
    pub audio_pll_cfg2: AUDIO_PLL_CFG2,
    #[doc = "0x75c - audio_pll_cfg3"]
    pub audio_pll_cfg3: AUDIO_PLL_CFG3,
    #[doc = "0x760 - audio_pll_cfg4"]
    pub audio_pll_cfg4: AUDIO_PLL_CFG4,
    #[doc = "0x764 - audio_pll_cfg5"]
    pub audio_pll_cfg5: AUDIO_PLL_CFG5,
    #[doc = "0x768 - audio_pll_cfg6"]
    pub audio_pll_cfg6: AUDIO_PLL_CFG6,
    #[doc = "0x76c - audio_pll_cfg7"]
    pub audio_pll_cfg7: AUDIO_PLL_CFG7,
    #[doc = "0x770 - audio_pll_cfg8"]
    pub audio_pll_cfg8: AUDIO_PLL_CFG8,
    #[doc = "0x774 - audio_pll_cfg9"]
    pub audio_pll_cfg9: AUDIO_PLL_CFG9,
    #[doc = "0x778 - audio_pll_cfg10"]
    pub audio_pll_cfg10: AUDIO_PLL_CFG10,
    #[doc = "0x77c - audio_pll_cfg11"]
    pub audio_pll_cfg11: AUDIO_PLL_CFG11,
    _reserved17: [u8; 0x50],
    #[doc = "0x7d0 - cpu_pll_cfg0"]
    pub cpu_pll_cfg0: CPU_PLL_CFG0,
    #[doc = "0x7d4 - cpu_pll_cfg1"]
    pub cpu_pll_cfg1: CPU_PLL_CFG1,
    #[doc = "0x7d8 - cpu_pll_cfg2"]
    pub cpu_pll_cfg2: CPU_PLL_CFG2,
    #[doc = "0x7dc - cpu_pll_cfg3"]
    pub cpu_pll_cfg3: CPU_PLL_CFG3,
    #[doc = "0x7e0 - cpu_pll_cfg4"]
    pub cpu_pll_cfg4: CPU_PLL_CFG4,
    #[doc = "0x7e4 - cpu_pll_cfg5"]
    pub cpu_pll_cfg5: CPU_PLL_CFG5,
    #[doc = "0x7e8 - cpu_pll_cfg6"]
    pub cpu_pll_cfg6: CPU_PLL_CFG6,
    #[doc = "0x7ec - cpu_pll_cfg7"]
    pub cpu_pll_cfg7: CPU_PLL_CFG7,
    #[doc = "0x7f0 - cpu_pll_cfg8"]
    pub cpu_pll_cfg8: CPU_PLL_CFG8,
    #[doc = "0x7f4 - cpu_pll_cfg9"]
    pub cpu_pll_cfg9: CPU_PLL_CFG9,
    #[doc = "0x7f8 - cpu_pll_cfg10"]
    pub cpu_pll_cfg10: CPU_PLL_CFG10,
}
#[doc = "cci_cfg (rw) register accessor: an alias for `Reg<CCI_CFG_SPEC>`"]
pub type CCI_CFG = crate::Reg<cci_cfg::CCI_CFG_SPEC>;
#[doc = "cci_cfg"]
pub mod cci_cfg;
#[doc = "cci_addr (rw) register accessor: an alias for `Reg<CCI_ADDR_SPEC>`"]
pub type CCI_ADDR = crate::Reg<cci_addr::CCI_ADDR_SPEC>;
#[doc = "cci_addr"]
pub mod cci_addr;
#[doc = "cci_wdata (rw) register accessor: an alias for `Reg<CCI_WDATA_SPEC>`"]
pub type CCI_WDATA = crate::Reg<cci_wdata::CCI_WDATA_SPEC>;
#[doc = "cci_wdata"]
pub mod cci_wdata;
#[doc = "cci_rdata (rw) register accessor: an alias for `Reg<CCI_RDATA_SPEC>`"]
pub type CCI_RDATA = crate::Reg<cci_rdata::CCI_RDATA_SPEC>;
#[doc = "cci_rdata"]
pub mod cci_rdata;
#[doc = "cci_ctl (rw) register accessor: an alias for `Reg<CCI_CTL_SPEC>`"]
pub type CCI_CTL = crate::Reg<cci_ctl::CCI_CTL_SPEC>;
#[doc = "cci_ctl"]
pub mod cci_ctl;
#[doc = "audio_pll_cfg0 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG0_SPEC>`"]
pub type AUDIO_PLL_CFG0 = crate::Reg<audio_pll_cfg0::AUDIO_PLL_CFG0_SPEC>;
#[doc = "audio_pll_cfg0"]
pub mod audio_pll_cfg0;
#[doc = "audio_pll_cfg1 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG1_SPEC>`"]
pub type AUDIO_PLL_CFG1 = crate::Reg<audio_pll_cfg1::AUDIO_PLL_CFG1_SPEC>;
#[doc = "audio_pll_cfg1"]
pub mod audio_pll_cfg1;
#[doc = "audio_pll_cfg2 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG2_SPEC>`"]
pub type AUDIO_PLL_CFG2 = crate::Reg<audio_pll_cfg2::AUDIO_PLL_CFG2_SPEC>;
#[doc = "audio_pll_cfg2"]
pub mod audio_pll_cfg2;
#[doc = "audio_pll_cfg3 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG3_SPEC>`"]
pub type AUDIO_PLL_CFG3 = crate::Reg<audio_pll_cfg3::AUDIO_PLL_CFG3_SPEC>;
#[doc = "audio_pll_cfg3"]
pub mod audio_pll_cfg3;
#[doc = "audio_pll_cfg4 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG4_SPEC>`"]
pub type AUDIO_PLL_CFG4 = crate::Reg<audio_pll_cfg4::AUDIO_PLL_CFG4_SPEC>;
#[doc = "audio_pll_cfg4"]
pub mod audio_pll_cfg4;
#[doc = "audio_pll_cfg5 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG5_SPEC>`"]
pub type AUDIO_PLL_CFG5 = crate::Reg<audio_pll_cfg5::AUDIO_PLL_CFG5_SPEC>;
#[doc = "audio_pll_cfg5"]
pub mod audio_pll_cfg5;
#[doc = "audio_pll_cfg6 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG6_SPEC>`"]
pub type AUDIO_PLL_CFG6 = crate::Reg<audio_pll_cfg6::AUDIO_PLL_CFG6_SPEC>;
#[doc = "audio_pll_cfg6"]
pub mod audio_pll_cfg6;
#[doc = "audio_pll_cfg7 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG7_SPEC>`"]
pub type AUDIO_PLL_CFG7 = crate::Reg<audio_pll_cfg7::AUDIO_PLL_CFG7_SPEC>;
#[doc = "audio_pll_cfg7"]
pub mod audio_pll_cfg7;
#[doc = "audio_pll_cfg8 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG8_SPEC>`"]
pub type AUDIO_PLL_CFG8 = crate::Reg<audio_pll_cfg8::AUDIO_PLL_CFG8_SPEC>;
#[doc = "audio_pll_cfg8"]
pub mod audio_pll_cfg8;
#[doc = "audio_pll_cfg9 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG9_SPEC>`"]
pub type AUDIO_PLL_CFG9 = crate::Reg<audio_pll_cfg9::AUDIO_PLL_CFG9_SPEC>;
#[doc = "audio_pll_cfg9"]
pub mod audio_pll_cfg9;
#[doc = "audio_pll_cfg10 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG10_SPEC>`"]
pub type AUDIO_PLL_CFG10 = crate::Reg<audio_pll_cfg10::AUDIO_PLL_CFG10_SPEC>;
#[doc = "audio_pll_cfg10"]
pub mod audio_pll_cfg10;
#[doc = "audio_pll_cfg11 (rw) register accessor: an alias for `Reg<AUDIO_PLL_CFG11_SPEC>`"]
pub type AUDIO_PLL_CFG11 = crate::Reg<audio_pll_cfg11::AUDIO_PLL_CFG11_SPEC>;
#[doc = "audio_pll_cfg11"]
pub mod audio_pll_cfg11;
#[doc = "cpu_pll_cfg0 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG0_SPEC>`"]
pub type CPU_PLL_CFG0 = crate::Reg<cpu_pll_cfg0::CPU_PLL_CFG0_SPEC>;
#[doc = "cpu_pll_cfg0"]
pub mod cpu_pll_cfg0;
#[doc = "cpu_pll_cfg1 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG1_SPEC>`"]
pub type CPU_PLL_CFG1 = crate::Reg<cpu_pll_cfg1::CPU_PLL_CFG1_SPEC>;
#[doc = "cpu_pll_cfg1"]
pub mod cpu_pll_cfg1;
#[doc = "cpu_pll_cfg2 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG2_SPEC>`"]
pub type CPU_PLL_CFG2 = crate::Reg<cpu_pll_cfg2::CPU_PLL_CFG2_SPEC>;
#[doc = "cpu_pll_cfg2"]
pub mod cpu_pll_cfg2;
#[doc = "cpu_pll_cfg3 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG3_SPEC>`"]
pub type CPU_PLL_CFG3 = crate::Reg<cpu_pll_cfg3::CPU_PLL_CFG3_SPEC>;
#[doc = "cpu_pll_cfg3"]
pub mod cpu_pll_cfg3;
#[doc = "cpu_pll_cfg4 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG4_SPEC>`"]
pub type CPU_PLL_CFG4 = crate::Reg<cpu_pll_cfg4::CPU_PLL_CFG4_SPEC>;
#[doc = "cpu_pll_cfg4"]
pub mod cpu_pll_cfg4;
#[doc = "cpu_pll_cfg5 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG5_SPEC>`"]
pub type CPU_PLL_CFG5 = crate::Reg<cpu_pll_cfg5::CPU_PLL_CFG5_SPEC>;
#[doc = "cpu_pll_cfg5"]
pub mod cpu_pll_cfg5;
#[doc = "cpu_pll_cfg6 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG6_SPEC>`"]
pub type CPU_PLL_CFG6 = crate::Reg<cpu_pll_cfg6::CPU_PLL_CFG6_SPEC>;
#[doc = "cpu_pll_cfg6"]
pub mod cpu_pll_cfg6;
#[doc = "cpu_pll_cfg7 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG7_SPEC>`"]
pub type CPU_PLL_CFG7 = crate::Reg<cpu_pll_cfg7::CPU_PLL_CFG7_SPEC>;
#[doc = "cpu_pll_cfg7"]
pub mod cpu_pll_cfg7;
#[doc = "cpu_pll_cfg8 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG8_SPEC>`"]
pub type CPU_PLL_CFG8 = crate::Reg<cpu_pll_cfg8::CPU_PLL_CFG8_SPEC>;
#[doc = "cpu_pll_cfg8"]
pub mod cpu_pll_cfg8;
#[doc = "cpu_pll_cfg9 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG9_SPEC>`"]
pub type CPU_PLL_CFG9 = crate::Reg<cpu_pll_cfg9::CPU_PLL_CFG9_SPEC>;
#[doc = "cpu_pll_cfg9"]
pub mod cpu_pll_cfg9;
#[doc = "cpu_pll_cfg10 (rw) register accessor: an alias for `Reg<CPU_PLL_CFG10_SPEC>`"]
pub type CPU_PLL_CFG10 = crate::Reg<cpu_pll_cfg10::CPU_PLL_CFG10_SPEC>;
#[doc = "cpu_pll_cfg10"]
pub mod cpu_pll_cfg10;
