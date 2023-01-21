#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - gpadc_config"]
    pub gpadc_pir_train: GPADC_PIR_TRAIN,
}
#[doc = "gpadc_pir_train (rw) register accessor: an alias for `Reg<GPADC_PIR_TRAIN_SPEC>`"]
pub type GPADC_PIR_TRAIN = crate::Reg<gpadc_pir_train::GPADC_PIR_TRAIN_SPEC>;
#[doc = "gpadc_config"]
pub mod gpadc_pir_train;
