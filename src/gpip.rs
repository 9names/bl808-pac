#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - gpadc_config"]
    pub gpadc_config: GPADC_CONFIG,
    #[doc = "0x04 - gpadc_dma_rdata"]
    pub gpadc_dma_rdata: GPADC_DMA_RDATA,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - gpadc_pir_train"]
    pub gpadc_pir_train: GPADC_PIR_TRAIN,
    _reserved3: [u8; 0x1c],
    #[doc = "0x40 - gpdac_config"]
    pub gpdac_config: GPDAC_CONFIG,
    #[doc = "0x44 - gpdac_dma_config"]
    pub gpdac_dma_config: GPDAC_DMA_CONFIG,
    #[doc = "0x48 - gpdac_dma_wdata"]
    pub gpdac_dma_wdata: GPDAC_DMA_WDATA,
}
#[doc = "gpadc_config (rw) register accessor: an alias for `Reg<GPADC_CONFIG_SPEC>`"]
pub type GPADC_CONFIG = crate::Reg<gpadc_config::GPADC_CONFIG_SPEC>;
#[doc = "gpadc_config"]
pub mod gpadc_config;
#[doc = "gpadc_dma_rdata (rw) register accessor: an alias for `Reg<GPADC_DMA_RDATA_SPEC>`"]
pub type GPADC_DMA_RDATA = crate::Reg<gpadc_dma_rdata::GPADC_DMA_RDATA_SPEC>;
#[doc = "gpadc_dma_rdata"]
pub mod gpadc_dma_rdata;
#[doc = "gpadc_pir_train (rw) register accessor: an alias for `Reg<GPADC_PIR_TRAIN_SPEC>`"]
pub type GPADC_PIR_TRAIN = crate::Reg<gpadc_pir_train::GPADC_PIR_TRAIN_SPEC>;
#[doc = "gpadc_pir_train"]
pub mod gpadc_pir_train;
#[doc = "gpdac_config (rw) register accessor: an alias for `Reg<GPDAC_CONFIG_SPEC>`"]
pub type GPDAC_CONFIG = crate::Reg<gpdac_config::GPDAC_CONFIG_SPEC>;
#[doc = "gpdac_config"]
pub mod gpdac_config;
#[doc = "gpdac_dma_config (rw) register accessor: an alias for `Reg<GPDAC_DMA_CONFIG_SPEC>`"]
pub type GPDAC_DMA_CONFIG = crate::Reg<gpdac_dma_config::GPDAC_DMA_CONFIG_SPEC>;
#[doc = "gpdac_dma_config"]
pub mod gpdac_dma_config;
#[doc = "gpdac_dma_wdata (rw) register accessor: an alias for `Reg<GPDAC_DMA_WDATA_SPEC>`"]
pub type GPDAC_DMA_WDATA = crate::Reg<gpdac_dma_wdata::GPDAC_DMA_WDATA_SPEC>;
#[doc = "gpdac_dma_wdata"]
pub mod gpdac_dma_wdata;
