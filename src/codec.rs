#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - codec_bus_ctrl"]
    pub codec_bus_ctrl: CODEC_BUS_CTRL,
    #[doc = "0x04 - codec_qos_ctrl"]
    pub codec_qos_ctrl: CODEC_QOS_CTRL,
    #[doc = "0x08 - codec_bus_thre"]
    pub codec_bus_thre: CODEC_BUS_THRE,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - codec_bus_dec_err"]
    pub codec_bus_dec_err: CODEC_BUS_DEC_ERR,
    #[doc = "0x14 - codec_bus_dec_err_addr"]
    pub codec_bus_dec_err_addr: CODEC_BUS_DEC_ERR_ADDR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - blai_lmtr_rd"]
    pub blai_lmtr_rd: BLAI_LMTR_RD,
    #[doc = "0x24 - blai_lmtr_wr"]
    pub blai_lmtr_wr: BLAI_LMTR_WR,
    #[doc = "0x28 - id_selection"]
    pub id_selection: ID_SELECTION,
    _reserved8: [u8; 0xd0],
    #[doc = "0xfc - CODEC_MISC_Dummy"]
    pub codec_misc_dummy: CODEC_MISC_DUMMY,
}
#[doc = "codec_bus_ctrl (rw) register accessor: an alias for `Reg<CODEC_BUS_CTRL_SPEC>`"]
pub type CODEC_BUS_CTRL = crate::Reg<codec_bus_ctrl::CODEC_BUS_CTRL_SPEC>;
#[doc = "codec_bus_ctrl"]
pub mod codec_bus_ctrl;
#[doc = "codec_qos_ctrl (rw) register accessor: an alias for `Reg<CODEC_QOS_CTRL_SPEC>`"]
pub type CODEC_QOS_CTRL = crate::Reg<codec_qos_ctrl::CODEC_QOS_CTRL_SPEC>;
#[doc = "codec_qos_ctrl"]
pub mod codec_qos_ctrl;
#[doc = "codec_bus_thre (rw) register accessor: an alias for `Reg<CODEC_BUS_THRE_SPEC>`"]
pub type CODEC_BUS_THRE = crate::Reg<codec_bus_thre::CODEC_BUS_THRE_SPEC>;
#[doc = "codec_bus_thre"]
pub mod codec_bus_thre;
#[doc = "codec_bus_dec_err (rw) register accessor: an alias for `Reg<CODEC_BUS_DEC_ERR_SPEC>`"]
pub type CODEC_BUS_DEC_ERR = crate::Reg<codec_bus_dec_err::CODEC_BUS_DEC_ERR_SPEC>;
#[doc = "codec_bus_dec_err"]
pub mod codec_bus_dec_err;
#[doc = "codec_bus_dec_err_addr (rw) register accessor: an alias for `Reg<CODEC_BUS_DEC_ERR_ADDR_SPEC>`"]
pub type CODEC_BUS_DEC_ERR_ADDR = crate::Reg<codec_bus_dec_err_addr::CODEC_BUS_DEC_ERR_ADDR_SPEC>;
#[doc = "codec_bus_dec_err_addr"]
pub mod codec_bus_dec_err_addr;
#[doc = "blai_lmtr_rd (rw) register accessor: an alias for `Reg<BLAI_LMTR_RD_SPEC>`"]
pub type BLAI_LMTR_RD = crate::Reg<blai_lmtr_rd::BLAI_LMTR_RD_SPEC>;
#[doc = "blai_lmtr_rd"]
pub mod blai_lmtr_rd;
#[doc = "blai_lmtr_wr (rw) register accessor: an alias for `Reg<BLAI_LMTR_WR_SPEC>`"]
pub type BLAI_LMTR_WR = crate::Reg<blai_lmtr_wr::BLAI_LMTR_WR_SPEC>;
#[doc = "blai_lmtr_wr"]
pub mod blai_lmtr_wr;
#[doc = "id_selection (rw) register accessor: an alias for `Reg<ID_SELECTION_SPEC>`"]
pub type ID_SELECTION = crate::Reg<id_selection::ID_SELECTION_SPEC>;
#[doc = "id_selection"]
pub mod id_selection;
#[doc = "codec_misc_dummy (rw) register accessor: an alias for `Reg<CODEC_MISC_DUMMY_SPEC>`"]
pub type CODEC_MISC_DUMMY = crate::Reg<codec_misc_dummy::CODEC_MISC_DUMMY_SPEC>;
#[doc = "CODEC_MISC_Dummy"]
pub mod codec_misc_dummy;
