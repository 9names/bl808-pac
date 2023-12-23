#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Set Write Register"]
    pub cpu1_ipc_iswr: CPU1_IPC_ISWR,
    #[doc = "0x04 - Interrupt raw status Register"]
    pub cpu1_ipc_irsrr: CPU1_IPC_IRSRR,
    #[doc = "0x08 - Interrupt Clear Register"]
    pub cpu1_ipc_icr: CPU1_IPC_ICR,
    #[doc = "0x0c - Interrupt Unmask Set Register"]
    pub cpu1_ipc_iusr: CPU1_IPC_IUSR,
    #[doc = "0x10 - Interrupt Unmask Clear Register"]
    pub cpu1_ipc_iucr: CPU1_IPC_IUCR,
    #[doc = "0x14 - Interrupt Line Sel Low Register"]
    pub cpu1_ipc_ilslr: CPU1_IPC_ILSLR,
    #[doc = "0x18 - Interrupt Line Sel High Register"]
    pub cpu1_ipc_ilshr: CPU1_IPC_ILSHR,
    #[doc = "0x1c - Interrupt status Register"]
    pub cpu1_ipc_isr: CPU1_IPC_ISR,
    #[doc = "0x20 - Interrupt Set Write Register"]
    pub cpu0_ipc_iswr: CPU0_IPC_ISWR,
    #[doc = "0x24 - Interrupt raw status Register"]
    pub cpu0_ipc_irsrr: CPU0_IPC_IRSRR,
    #[doc = "0x28 - Interrupt Clear Register"]
    pub cpu0_ipc_icr: CPU0_IPC_ICR,
    #[doc = "0x2c - Interrupt Unmask Set Register"]
    pub cpu0_ipc_iusr: CPU0_IPC_IUSR,
    #[doc = "0x30 - Interrupt Unmask Clear Register"]
    pub cpu0_ipc_iucr: CPU0_IPC_IUCR,
    #[doc = "0x34 - Interrupt Line Sel Low Register"]
    pub cpu0_ipc_ilslr: CPU0_IPC_ILSLR,
    #[doc = "0x38 - Interrupt Line Sel High Register"]
    pub cpu0_ipc_ilshr: CPU0_IPC_ILSHR,
    #[doc = "0x3c - Interrupt status Register"]
    pub cpu0_ipc_isr: CPU0_IPC_ISR,
}
#[doc = "cpu1_ipc_iswr (rw) register accessor: an alias for `Reg<CPU1_IPC_ISWR_SPEC>`"]
pub type CPU1_IPC_ISWR = crate::Reg<cpu1_ipc_iswr::CPU1_IPC_ISWR_SPEC>;
#[doc = "Interrupt Set Write Register"]
pub mod cpu1_ipc_iswr;
#[doc = "cpu1_ipc_irsrr (rw) register accessor: an alias for `Reg<CPU1_IPC_IRSRR_SPEC>`"]
pub type CPU1_IPC_IRSRR = crate::Reg<cpu1_ipc_irsrr::CPU1_IPC_IRSRR_SPEC>;
#[doc = "Interrupt raw status Register"]
pub mod cpu1_ipc_irsrr;
#[doc = "cpu1_ipc_icr (rw) register accessor: an alias for `Reg<CPU1_IPC_ICR_SPEC>`"]
pub type CPU1_IPC_ICR = crate::Reg<cpu1_ipc_icr::CPU1_IPC_ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod cpu1_ipc_icr;
#[doc = "cpu1_ipc_iusr (rw) register accessor: an alias for `Reg<CPU1_IPC_IUSR_SPEC>`"]
pub type CPU1_IPC_IUSR = crate::Reg<cpu1_ipc_iusr::CPU1_IPC_IUSR_SPEC>;
#[doc = "Interrupt Unmask Set Register"]
pub mod cpu1_ipc_iusr;
#[doc = "cpu1_ipc_iucr (rw) register accessor: an alias for `Reg<CPU1_IPC_IUCR_SPEC>`"]
pub type CPU1_IPC_IUCR = crate::Reg<cpu1_ipc_iucr::CPU1_IPC_IUCR_SPEC>;
#[doc = "Interrupt Unmask Clear Register"]
pub mod cpu1_ipc_iucr;
#[doc = "cpu1_ipc_ilslr (rw) register accessor: an alias for `Reg<CPU1_IPC_ILSLR_SPEC>`"]
pub type CPU1_IPC_ILSLR = crate::Reg<cpu1_ipc_ilslr::CPU1_IPC_ILSLR_SPEC>;
#[doc = "Interrupt Line Sel Low Register"]
pub mod cpu1_ipc_ilslr;
#[doc = "cpu1_ipc_ilshr (rw) register accessor: an alias for `Reg<CPU1_IPC_ILSHR_SPEC>`"]
pub type CPU1_IPC_ILSHR = crate::Reg<cpu1_ipc_ilshr::CPU1_IPC_ILSHR_SPEC>;
#[doc = "Interrupt Line Sel High Register"]
pub mod cpu1_ipc_ilshr;
#[doc = "cpu1_ipc_isr (rw) register accessor: an alias for `Reg<CPU1_IPC_ISR_SPEC>`"]
pub type CPU1_IPC_ISR = crate::Reg<cpu1_ipc_isr::CPU1_IPC_ISR_SPEC>;
#[doc = "Interrupt status Register"]
pub mod cpu1_ipc_isr;
#[doc = "cpu0_ipc_iswr (rw) register accessor: an alias for `Reg<CPU0_IPC_ISWR_SPEC>`"]
pub type CPU0_IPC_ISWR = crate::Reg<cpu0_ipc_iswr::CPU0_IPC_ISWR_SPEC>;
#[doc = "Interrupt Set Write Register"]
pub mod cpu0_ipc_iswr;
#[doc = "cpu0_ipc_irsrr (rw) register accessor: an alias for `Reg<CPU0_IPC_IRSRR_SPEC>`"]
pub type CPU0_IPC_IRSRR = crate::Reg<cpu0_ipc_irsrr::CPU0_IPC_IRSRR_SPEC>;
#[doc = "Interrupt raw status Register"]
pub mod cpu0_ipc_irsrr;
#[doc = "cpu0_ipc_icr (rw) register accessor: an alias for `Reg<CPU0_IPC_ICR_SPEC>`"]
pub type CPU0_IPC_ICR = crate::Reg<cpu0_ipc_icr::CPU0_IPC_ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod cpu0_ipc_icr;
#[doc = "cpu0_ipc_iusr (rw) register accessor: an alias for `Reg<CPU0_IPC_IUSR_SPEC>`"]
pub type CPU0_IPC_IUSR = crate::Reg<cpu0_ipc_iusr::CPU0_IPC_IUSR_SPEC>;
#[doc = "Interrupt Unmask Set Register"]
pub mod cpu0_ipc_iusr;
#[doc = "cpu0_ipc_iucr (rw) register accessor: an alias for `Reg<CPU0_IPC_IUCR_SPEC>`"]
pub type CPU0_IPC_IUCR = crate::Reg<cpu0_ipc_iucr::CPU0_IPC_IUCR_SPEC>;
#[doc = "Interrupt Unmask Clear Register"]
pub mod cpu0_ipc_iucr;
#[doc = "cpu0_ipc_ilslr (rw) register accessor: an alias for `Reg<CPU0_IPC_ILSLR_SPEC>`"]
pub type CPU0_IPC_ILSLR = crate::Reg<cpu0_ipc_ilslr::CPU0_IPC_ILSLR_SPEC>;
#[doc = "Interrupt Line Sel Low Register"]
pub mod cpu0_ipc_ilslr;
#[doc = "cpu0_ipc_ilshr (rw) register accessor: an alias for `Reg<CPU0_IPC_ILSHR_SPEC>`"]
pub type CPU0_IPC_ILSHR = crate::Reg<cpu0_ipc_ilshr::CPU0_IPC_ILSHR_SPEC>;
#[doc = "Interrupt Line Sel High Register"]
pub mod cpu0_ipc_ilshr;
#[doc = "cpu0_ipc_isr (rw) register accessor: an alias for `Reg<CPU0_IPC_ISR_SPEC>`"]
pub type CPU0_IPC_ISR = crate::Reg<cpu0_ipc_isr::CPU0_IPC_ISR_SPEC>;
#[doc = "Interrupt status Register"]
pub mod cpu0_ipc_isr;
