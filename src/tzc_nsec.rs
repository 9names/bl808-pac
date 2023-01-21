#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - tzc_rom_tzsrg_ctrl"]
    pub tzc_rom_tzsrg_ctrl: TZC_ROM_TZSRG_CTRL,
    #[doc = "0x44 - tzc_rom_tzsrg_adr_mask"]
    pub tzc_rom_tzsrg_adr_mask: TZC_ROM_TZSRG_ADR_MASK,
    #[doc = "0x48 - tzc_rom_tzsrg_r0"]
    pub tzc_rom_tzsrg_r0: TZC_ROM_TZSRG_R0,
    #[doc = "0x4c - tzc_rom_tzsrg_r1"]
    pub tzc_rom_tzsrg_r1: TZC_ROM_TZSRG_R1,
    #[doc = "0x50 - tzc_rom_tzsrg_r2"]
    pub tzc_rom_tzsrg_r2: TZC_ROM_TZSRG_R2,
    #[doc = "0x54 - tzc_rom_tzsrg_r3"]
    pub tzc_rom_tzsrg_r3: TZC_ROM_TZSRG_R3,
    _reserved6: [u8; 0xa8],
    #[doc = "0x100 - tzc_bmx_tzmid"]
    pub tzc_bmx_tzmid: TZC_BMX_TZMID,
    #[doc = "0x104 - tzc_bmx_tzmid_lock"]
    pub tzc_bmx_tzmid_lock: TZC_BMX_TZMID_LOCK,
    #[doc = "0x108 - tzc_bmx_s0"]
    pub tzc_bmx_s0: TZC_BMX_S0,
    #[doc = "0x10c - tzc_bmx_s1"]
    pub tzc_bmx_s1: TZC_BMX_S1,
    #[doc = "0x110 - tzc_bmx_s2"]
    pub tzc_bmx_s2: TZC_BMX_S2,
    #[doc = "0x114 - tzc_bmx_s_lock"]
    pub tzc_bmx_s_lock: TZC_BMX_S_LOCK,
    _reserved12: [u8; 0x28],
    #[doc = "0x140 - tzc_ocram_tzsrg_ctrl"]
    pub tzc_ocram_tzsrg_ctrl: TZC_OCRAM_TZSRG_CTRL,
    #[doc = "0x144 - tzc_ocram_tzsrg_adr_mask"]
    pub tzc_ocram_tzsrg_adr_mask: TZC_OCRAM_TZSRG_ADR_MASK,
    #[doc = "0x148 - tzc_ocram_tzsrg_r0"]
    pub tzc_ocram_tzsrg_r0: TZC_OCRAM_TZSRG_R0,
    #[doc = "0x14c - tzc_ocram_tzsrg_r1"]
    pub tzc_ocram_tzsrg_r1: TZC_OCRAM_TZSRG_R1,
    #[doc = "0x150 - tzc_ocram_tzsrg_r2"]
    pub tzc_ocram_tzsrg_r2: TZC_OCRAM_TZSRG_R2,
    #[doc = "0x154 - tzc_ocram_tzsrg_r3"]
    pub tzc_ocram_tzsrg_r3: TZC_OCRAM_TZSRG_R3,
    _reserved18: [u8; 0x28],
    #[doc = "0x180 - tzc_wram_tzsrg_ctrl"]
    pub tzc_wram_tzsrg_ctrl: TZC_WRAM_TZSRG_CTRL,
    #[doc = "0x184 - tzc_wram_tzsrg_adr_mask"]
    pub tzc_wram_tzsrg_adr_mask: TZC_WRAM_TZSRG_ADR_MASK,
    #[doc = "0x188 - tzc_wram_tzsrg_r0"]
    pub tzc_wram_tzsrg_r0: TZC_WRAM_TZSRG_R0,
    #[doc = "0x18c - tzc_wram_tzsrg_r1"]
    pub tzc_wram_tzsrg_r1: TZC_WRAM_TZSRG_R1,
    #[doc = "0x190 - tzc_wram_tzsrg_r2"]
    pub tzc_wram_tzsrg_r2: TZC_WRAM_TZSRG_R2,
    #[doc = "0x194 - tzc_wram_tzsrg_r3"]
    pub tzc_wram_tzsrg_r3: TZC_WRAM_TZSRG_R3,
    #[doc = "0x198 - tzc_wifi_dbg"]
    pub tzc_wifi_dbg: TZC_WIFI_DBG,
    _reserved25: [u8; 0xa4],
    #[doc = "0x240 - tzc_pdm_ctrl"]
    pub tzc_pdm_ctrl: TZC_PDM_CTRL,
    #[doc = "0x244 - tzc_uart_ctrl"]
    pub tzc_uart_ctrl: TZC_UART_CTRL,
    #[doc = "0x248 - tzc_i2c_ctrl"]
    pub tzc_i2c_ctrl: TZC_I2C_CTRL,
    #[doc = "0x24c - tzc_timer_ctrl"]
    pub tzc_timer_ctrl: TZC_TIMER_CTRL,
    #[doc = "0x250 - tzc_i2s_ctrl"]
    pub tzc_i2s_ctrl: TZC_I2S_CTRL,
    _reserved30: [u8; 0x2c],
    #[doc = "0x280 - tzc_sf_tzsrg_ctrl"]
    pub tzc_sf_tzsrg_ctrl: TZC_SF_TZSRG_CTRL,
    #[doc = "0x284 - tzc_sf_tzsrg_adr_mask"]
    pub tzc_sf_tzsrg_adr_mask: TZC_SF_TZSRG_ADR_MASK,
    #[doc = "0x288 - tzc_sf_tzsrg_r0"]
    pub tzc_sf_tzsrg_r0: TZC_SF_TZSRG_R0,
    #[doc = "0x28c - tzc_sf_tzsrg_r1"]
    pub tzc_sf_tzsrg_r1: TZC_SF_TZSRG_R1,
    #[doc = "0x290 - tzc_sf_tzsrg_r2"]
    pub tzc_sf_tzsrg_r2: TZC_SF_TZSRG_R2,
    #[doc = "0x294 - tzc_sf_tzsrg_r3"]
    pub tzc_sf_tzsrg_r3: TZC_SF_TZSRG_R3,
    #[doc = "0x298 - tzc_sf_tzsrg_msb"]
    pub tzc_sf_tzsrg_msb: TZC_SF_TZSRG_MSB,
    _reserved37: [u8; 0x64],
    #[doc = "0x300 - tzc_mm_bmx_tzmid"]
    pub tzc_mm_bmx_tzmid: TZC_MM_BMX_TZMID,
    #[doc = "0x304 - tzc_mm_bmx_tzmid_lock"]
    pub tzc_mm_bmx_tzmid_lock: TZC_MM_BMX_TZMID_LOCK,
    #[doc = "0x308 - tzc_mm_bmx_s0"]
    pub tzc_mm_bmx_s0: TZC_MM_BMX_S0,
    #[doc = "0x30c - tzc_mm_bmx_s1"]
    pub tzc_mm_bmx_s1: TZC_MM_BMX_S1,
    #[doc = "0x310 - tzc_mm_bmx_s2"]
    pub tzc_mm_bmx_s2: TZC_MM_BMX_S2,
    #[doc = "0x314 - tzc_mm_bmx_s_lock0"]
    pub tzc_mm_bmx_s_lock0: TZC_MM_BMX_S_LOCK0,
    #[doc = "0x318 - tzc_mm_bmx_s_lock1"]
    pub tzc_mm_bmx_s_lock1: TZC_MM_BMX_S_LOCK1,
    _reserved44: [u8; 0x24],
    #[doc = "0x340 - tzc_l2sram_tzsrg_ctrl"]
    pub tzc_l2sram_tzsrg_ctrl: TZC_L2SRAM_TZSRG_CTRL,
    #[doc = "0x344 - tzc_l2sram_tzsrg_adr_mask"]
    pub tzc_l2sram_tzsrg_adr_mask: TZC_L2SRAM_TZSRG_ADR_MASK,
    #[doc = "0x348 - tzc_l2sram_tzsrg_r0"]
    pub tzc_l2sram_tzsrg_r0: TZC_L2SRAM_TZSRG_R0,
    #[doc = "0x34c - tzc_l2sram_tzsrg_r1"]
    pub tzc_l2sram_tzsrg_r1: TZC_L2SRAM_TZSRG_R1,
    #[doc = "0x350 - tzc_l2sram_tzsrg_r2"]
    pub tzc_l2sram_tzsrg_r2: TZC_L2SRAM_TZSRG_R2,
    #[doc = "0x354 - tzc_l2sram_tzsrg_r3"]
    pub tzc_l2sram_tzsrg_r3: TZC_L2SRAM_TZSRG_R3,
    _reserved50: [u8; 0x08],
    #[doc = "0x360 - tzc_vram_tzsrg_ctrl"]
    pub tzc_vram_tzsrg_ctrl: TZC_VRAM_TZSRG_CTRL,
    #[doc = "0x364 - tzc_vram_tzsrg_adr_mask"]
    pub tzc_vram_tzsrg_adr_mask: TZC_VRAM_TZSRG_ADR_MASK,
    #[doc = "0x368 - tzc_vram_tzsrg_r0"]
    pub tzc_vram_tzsrg_r0: TZC_VRAM_TZSRG_R0,
    #[doc = "0x36c - tzc_vram_tzsrg_r1"]
    pub tzc_vram_tzsrg_r1: TZC_VRAM_TZSRG_R1,
    #[doc = "0x370 - tzc_vram_tzsrg_r2"]
    pub tzc_vram_tzsrg_r2: TZC_VRAM_TZSRG_R2,
    #[doc = "0x374 - tzc_vram_tzsrg_r3"]
    pub tzc_vram_tzsrg_r3: TZC_VRAM_TZSRG_R3,
    _reserved56: [u8; 0x08],
    #[doc = "0x380 - tzc_psrama_tzsrg_ctrl"]
    pub tzc_psrama_tzsrg_ctrl: TZC_PSRAMA_TZSRG_CTRL,
    #[doc = "0x384 - tzc_psrama_tzsrg_adr_mask"]
    pub tzc_psrama_tzsrg_adr_mask: TZC_PSRAMA_TZSRG_ADR_MASK,
    #[doc = "0x388 - tzc_psrama_tzsrg_r0"]
    pub tzc_psrama_tzsrg_r0: TZC_PSRAMA_TZSRG_R0,
    #[doc = "0x38c - tzc_psrama_tzsrg_r1"]
    pub tzc_psrama_tzsrg_r1: TZC_PSRAMA_TZSRG_R1,
    #[doc = "0x390 - tzc_psrama_tzsrg_r2"]
    pub tzc_psrama_tzsrg_r2: TZC_PSRAMA_TZSRG_R2,
    #[doc = "0x394 - tzc_psrama_tzsrg_r3"]
    pub tzc_psrama_tzsrg_r3: TZC_PSRAMA_TZSRG_R3,
    _reserved62: [u8; 0x08],
    #[doc = "0x3a0 - tzc_psramb_tzsrg_ctrl"]
    pub tzc_psramb_tzsrg_ctrl: TZC_PSRAMB_TZSRG_CTRL,
    #[doc = "0x3a4 - tzc_psramb_tzsrg_adr_mask"]
    pub tzc_psramb_tzsrg_adr_mask: TZC_PSRAMB_TZSRG_ADR_MASK,
    #[doc = "0x3a8 - tzc_psramb_tzsrg_r0"]
    pub tzc_psramb_tzsrg_r0: TZC_PSRAMB_TZSRG_R0,
    #[doc = "0x3ac - tzc_psramb_tzsrg_r1"]
    pub tzc_psramb_tzsrg_r1: TZC_PSRAMB_TZSRG_R1,
    #[doc = "0x3b0 - tzc_psramb_tzsrg_r2"]
    pub tzc_psramb_tzsrg_r2: TZC_PSRAMB_TZSRG_R2,
    #[doc = "0x3b4 - tzc_psramb_tzsrg_r3"]
    pub tzc_psramb_tzsrg_r3: TZC_PSRAMB_TZSRG_R3,
    _reserved68: [u8; 0x08],
    #[doc = "0x3c0 - tzc_xram_tzsrg_ctrl"]
    pub tzc_xram_tzsrg_ctrl: TZC_XRAM_TZSRG_CTRL,
    #[doc = "0x3c4 - tzc_xram_tzsrg_adr_mask"]
    pub tzc_xram_tzsrg_adr_mask: TZC_XRAM_TZSRG_ADR_MASK,
    #[doc = "0x3c8 - tzc_xram_tzsrg_r0"]
    pub tzc_xram_tzsrg_r0: TZC_XRAM_TZSRG_R0,
    #[doc = "0x3cc - tzc_xram_tzsrg_r1"]
    pub tzc_xram_tzsrg_r1: TZC_XRAM_TZSRG_R1,
    #[doc = "0x3d0 - tzc_xram_tzsrg_r2"]
    pub tzc_xram_tzsrg_r2: TZC_XRAM_TZSRG_R2,
    #[doc = "0x3d4 - tzc_xram_tzsrg_r3"]
    pub tzc_xram_tzsrg_r3: TZC_XRAM_TZSRG_R3,
    _reserved74: [u8; 0x0b28],
    #[doc = "0xf00 - tzc_glb_ctrl_0"]
    pub tzc_glb_ctrl_0: TZC_GLB_CTRL_0,
    #[doc = "0xf04 - tzc_glb_ctrl_1"]
    pub tzc_glb_ctrl_1: TZC_GLB_CTRL_1,
    #[doc = "0xf08 - tzc_glb_ctrl_2"]
    pub tzc_glb_ctrl_2: TZC_GLB_CTRL_2,
    _reserved77: [u8; 0x14],
    #[doc = "0xf20 - tzc_mm_ctrl_0"]
    pub tzc_mm_ctrl_0: TZC_MM_CTRL_0,
    #[doc = "0xf24 - tzc_mm_ctrl_1"]
    pub tzc_mm_ctrl_1: TZC_MM_CTRL_1,
    #[doc = "0xf28 - tzc_mm_ctrl_2"]
    pub tzc_mm_ctrl_2: TZC_MM_CTRL_2,
    _reserved80: [u8; 0x14],
    #[doc = "0xf40 - tzc_se_ctrl_0"]
    pub tzc_se_ctrl_0: TZC_SE_CTRL_0,
    #[doc = "0xf44 - tzc_se_ctrl_1"]
    pub tzc_se_ctrl_1: TZC_SE_CTRL_1,
}
#[doc = "tzc_rom_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_ROM_TZSRG_CTRL_SPEC>`"]
pub type TZC_ROM_TZSRG_CTRL = crate::Reg<tzc_rom_tzsrg_ctrl::TZC_ROM_TZSRG_CTRL_SPEC>;
#[doc = "tzc_rom_tzsrg_ctrl"]
pub mod tzc_rom_tzsrg_ctrl;
#[doc = "tzc_rom_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_ROM_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_ROM_TZSRG_ADR_MASK = crate::Reg<tzc_rom_tzsrg_adr_mask::TZC_ROM_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_rom_tzsrg_adr_mask"]
pub mod tzc_rom_tzsrg_adr_mask;
#[doc = "tzc_rom_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_ROM_TZSRG_R0_SPEC>`"]
pub type TZC_ROM_TZSRG_R0 = crate::Reg<tzc_rom_tzsrg_r0::TZC_ROM_TZSRG_R0_SPEC>;
#[doc = "tzc_rom_tzsrg_r0"]
pub mod tzc_rom_tzsrg_r0;
#[doc = "tzc_rom_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_ROM_TZSRG_R1_SPEC>`"]
pub type TZC_ROM_TZSRG_R1 = crate::Reg<tzc_rom_tzsrg_r1::TZC_ROM_TZSRG_R1_SPEC>;
#[doc = "tzc_rom_tzsrg_r1"]
pub mod tzc_rom_tzsrg_r1;
#[doc = "tzc_rom_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_ROM_TZSRG_R2_SPEC>`"]
pub type TZC_ROM_TZSRG_R2 = crate::Reg<tzc_rom_tzsrg_r2::TZC_ROM_TZSRG_R2_SPEC>;
#[doc = "tzc_rom_tzsrg_r2"]
pub mod tzc_rom_tzsrg_r2;
#[doc = "tzc_rom_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_ROM_TZSRG_R3_SPEC>`"]
pub type TZC_ROM_TZSRG_R3 = crate::Reg<tzc_rom_tzsrg_r3::TZC_ROM_TZSRG_R3_SPEC>;
#[doc = "tzc_rom_tzsrg_r3"]
pub mod tzc_rom_tzsrg_r3;
#[doc = "tzc_bmx_tzmid (rw) register accessor: an alias for `Reg<TZC_BMX_TZMID_SPEC>`"]
pub type TZC_BMX_TZMID = crate::Reg<tzc_bmx_tzmid::TZC_BMX_TZMID_SPEC>;
#[doc = "tzc_bmx_tzmid"]
pub mod tzc_bmx_tzmid;
#[doc = "tzc_bmx_tzmid_lock (rw) register accessor: an alias for `Reg<TZC_BMX_TZMID_LOCK_SPEC>`"]
pub type TZC_BMX_TZMID_LOCK = crate::Reg<tzc_bmx_tzmid_lock::TZC_BMX_TZMID_LOCK_SPEC>;
#[doc = "tzc_bmx_tzmid_lock"]
pub mod tzc_bmx_tzmid_lock;
#[doc = "tzc_bmx_s0 (rw) register accessor: an alias for `Reg<TZC_BMX_S0_SPEC>`"]
pub type TZC_BMX_S0 = crate::Reg<tzc_bmx_s0::TZC_BMX_S0_SPEC>;
#[doc = "tzc_bmx_s0"]
pub mod tzc_bmx_s0;
#[doc = "tzc_bmx_s1 (rw) register accessor: an alias for `Reg<TZC_BMX_S1_SPEC>`"]
pub type TZC_BMX_S1 = crate::Reg<tzc_bmx_s1::TZC_BMX_S1_SPEC>;
#[doc = "tzc_bmx_s1"]
pub mod tzc_bmx_s1;
#[doc = "tzc_bmx_s2 (rw) register accessor: an alias for `Reg<TZC_BMX_S2_SPEC>`"]
pub type TZC_BMX_S2 = crate::Reg<tzc_bmx_s2::TZC_BMX_S2_SPEC>;
#[doc = "tzc_bmx_s2"]
pub mod tzc_bmx_s2;
#[doc = "tzc_bmx_s_lock (rw) register accessor: an alias for `Reg<TZC_BMX_S_LOCK_SPEC>`"]
pub type TZC_BMX_S_LOCK = crate::Reg<tzc_bmx_s_lock::TZC_BMX_S_LOCK_SPEC>;
#[doc = "tzc_bmx_s_lock"]
pub mod tzc_bmx_s_lock;
#[doc = "tzc_ocram_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_OCRAM_TZSRG_CTRL_SPEC>`"]
pub type TZC_OCRAM_TZSRG_CTRL = crate::Reg<tzc_ocram_tzsrg_ctrl::TZC_OCRAM_TZSRG_CTRL_SPEC>;
#[doc = "tzc_ocram_tzsrg_ctrl"]
pub mod tzc_ocram_tzsrg_ctrl;
#[doc = "tzc_ocram_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_OCRAM_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_OCRAM_TZSRG_ADR_MASK =
    crate::Reg<tzc_ocram_tzsrg_adr_mask::TZC_OCRAM_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_ocram_tzsrg_adr_mask"]
pub mod tzc_ocram_tzsrg_adr_mask;
#[doc = "tzc_ocram_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_OCRAM_TZSRG_R0_SPEC>`"]
pub type TZC_OCRAM_TZSRG_R0 = crate::Reg<tzc_ocram_tzsrg_r0::TZC_OCRAM_TZSRG_R0_SPEC>;
#[doc = "tzc_ocram_tzsrg_r0"]
pub mod tzc_ocram_tzsrg_r0;
#[doc = "tzc_ocram_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_OCRAM_TZSRG_R1_SPEC>`"]
pub type TZC_OCRAM_TZSRG_R1 = crate::Reg<tzc_ocram_tzsrg_r1::TZC_OCRAM_TZSRG_R1_SPEC>;
#[doc = "tzc_ocram_tzsrg_r1"]
pub mod tzc_ocram_tzsrg_r1;
#[doc = "tzc_ocram_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_OCRAM_TZSRG_R2_SPEC>`"]
pub type TZC_OCRAM_TZSRG_R2 = crate::Reg<tzc_ocram_tzsrg_r2::TZC_OCRAM_TZSRG_R2_SPEC>;
#[doc = "tzc_ocram_tzsrg_r2"]
pub mod tzc_ocram_tzsrg_r2;
#[doc = "tzc_ocram_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_OCRAM_TZSRG_R3_SPEC>`"]
pub type TZC_OCRAM_TZSRG_R3 = crate::Reg<tzc_ocram_tzsrg_r3::TZC_OCRAM_TZSRG_R3_SPEC>;
#[doc = "tzc_ocram_tzsrg_r3"]
pub mod tzc_ocram_tzsrg_r3;
#[doc = "tzc_wram_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_WRAM_TZSRG_CTRL_SPEC>`"]
pub type TZC_WRAM_TZSRG_CTRL = crate::Reg<tzc_wram_tzsrg_ctrl::TZC_WRAM_TZSRG_CTRL_SPEC>;
#[doc = "tzc_wram_tzsrg_ctrl"]
pub mod tzc_wram_tzsrg_ctrl;
#[doc = "tzc_wram_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_WRAM_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_WRAM_TZSRG_ADR_MASK =
    crate::Reg<tzc_wram_tzsrg_adr_mask::TZC_WRAM_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_wram_tzsrg_adr_mask"]
pub mod tzc_wram_tzsrg_adr_mask;
#[doc = "tzc_wram_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_WRAM_TZSRG_R0_SPEC>`"]
pub type TZC_WRAM_TZSRG_R0 = crate::Reg<tzc_wram_tzsrg_r0::TZC_WRAM_TZSRG_R0_SPEC>;
#[doc = "tzc_wram_tzsrg_r0"]
pub mod tzc_wram_tzsrg_r0;
#[doc = "tzc_wram_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_WRAM_TZSRG_R1_SPEC>`"]
pub type TZC_WRAM_TZSRG_R1 = crate::Reg<tzc_wram_tzsrg_r1::TZC_WRAM_TZSRG_R1_SPEC>;
#[doc = "tzc_wram_tzsrg_r1"]
pub mod tzc_wram_tzsrg_r1;
#[doc = "tzc_wram_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_WRAM_TZSRG_R2_SPEC>`"]
pub type TZC_WRAM_TZSRG_R2 = crate::Reg<tzc_wram_tzsrg_r2::TZC_WRAM_TZSRG_R2_SPEC>;
#[doc = "tzc_wram_tzsrg_r2"]
pub mod tzc_wram_tzsrg_r2;
#[doc = "tzc_wram_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_WRAM_TZSRG_R3_SPEC>`"]
pub type TZC_WRAM_TZSRG_R3 = crate::Reg<tzc_wram_tzsrg_r3::TZC_WRAM_TZSRG_R3_SPEC>;
#[doc = "tzc_wram_tzsrg_r3"]
pub mod tzc_wram_tzsrg_r3;
#[doc = "tzc_wifi_dbg (rw) register accessor: an alias for `Reg<TZC_WIFI_DBG_SPEC>`"]
pub type TZC_WIFI_DBG = crate::Reg<tzc_wifi_dbg::TZC_WIFI_DBG_SPEC>;
#[doc = "tzc_wifi_dbg"]
pub mod tzc_wifi_dbg;
#[doc = "tzc_pdm_ctrl (rw) register accessor: an alias for `Reg<TZC_PDM_CTRL_SPEC>`"]
pub type TZC_PDM_CTRL = crate::Reg<tzc_pdm_ctrl::TZC_PDM_CTRL_SPEC>;
#[doc = "tzc_pdm_ctrl"]
pub mod tzc_pdm_ctrl;
#[doc = "tzc_uart_ctrl (rw) register accessor: an alias for `Reg<TZC_UART_CTRL_SPEC>`"]
pub type TZC_UART_CTRL = crate::Reg<tzc_uart_ctrl::TZC_UART_CTRL_SPEC>;
#[doc = "tzc_uart_ctrl"]
pub mod tzc_uart_ctrl;
#[doc = "tzc_i2c_ctrl (rw) register accessor: an alias for `Reg<TZC_I2C_CTRL_SPEC>`"]
pub type TZC_I2C_CTRL = crate::Reg<tzc_i2c_ctrl::TZC_I2C_CTRL_SPEC>;
#[doc = "tzc_i2c_ctrl"]
pub mod tzc_i2c_ctrl;
#[doc = "tzc_timer_ctrl (rw) register accessor: an alias for `Reg<TZC_TIMER_CTRL_SPEC>`"]
pub type TZC_TIMER_CTRL = crate::Reg<tzc_timer_ctrl::TZC_TIMER_CTRL_SPEC>;
#[doc = "tzc_timer_ctrl"]
pub mod tzc_timer_ctrl;
#[doc = "tzc_i2s_ctrl (rw) register accessor: an alias for `Reg<TZC_I2S_CTRL_SPEC>`"]
pub type TZC_I2S_CTRL = crate::Reg<tzc_i2s_ctrl::TZC_I2S_CTRL_SPEC>;
#[doc = "tzc_i2s_ctrl"]
pub mod tzc_i2s_ctrl;
#[doc = "tzc_sf_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_SF_TZSRG_CTRL_SPEC>`"]
pub type TZC_SF_TZSRG_CTRL = crate::Reg<tzc_sf_tzsrg_ctrl::TZC_SF_TZSRG_CTRL_SPEC>;
#[doc = "tzc_sf_tzsrg_ctrl"]
pub mod tzc_sf_tzsrg_ctrl;
#[doc = "tzc_sf_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_SF_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_SF_TZSRG_ADR_MASK = crate::Reg<tzc_sf_tzsrg_adr_mask::TZC_SF_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_sf_tzsrg_adr_mask"]
pub mod tzc_sf_tzsrg_adr_mask;
#[doc = "tzc_sf_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_SF_TZSRG_R0_SPEC>`"]
pub type TZC_SF_TZSRG_R0 = crate::Reg<tzc_sf_tzsrg_r0::TZC_SF_TZSRG_R0_SPEC>;
#[doc = "tzc_sf_tzsrg_r0"]
pub mod tzc_sf_tzsrg_r0;
#[doc = "tzc_sf_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_SF_TZSRG_R1_SPEC>`"]
pub type TZC_SF_TZSRG_R1 = crate::Reg<tzc_sf_tzsrg_r1::TZC_SF_TZSRG_R1_SPEC>;
#[doc = "tzc_sf_tzsrg_r1"]
pub mod tzc_sf_tzsrg_r1;
#[doc = "tzc_sf_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_SF_TZSRG_R2_SPEC>`"]
pub type TZC_SF_TZSRG_R2 = crate::Reg<tzc_sf_tzsrg_r2::TZC_SF_TZSRG_R2_SPEC>;
#[doc = "tzc_sf_tzsrg_r2"]
pub mod tzc_sf_tzsrg_r2;
#[doc = "tzc_sf_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_SF_TZSRG_R3_SPEC>`"]
pub type TZC_SF_TZSRG_R3 = crate::Reg<tzc_sf_tzsrg_r3::TZC_SF_TZSRG_R3_SPEC>;
#[doc = "tzc_sf_tzsrg_r3"]
pub mod tzc_sf_tzsrg_r3;
#[doc = "tzc_sf_tzsrg_msb (rw) register accessor: an alias for `Reg<TZC_SF_TZSRG_MSB_SPEC>`"]
pub type TZC_SF_TZSRG_MSB = crate::Reg<tzc_sf_tzsrg_msb::TZC_SF_TZSRG_MSB_SPEC>;
#[doc = "tzc_sf_tzsrg_msb"]
pub mod tzc_sf_tzsrg_msb;
#[doc = "tzc_mm_bmx_tzmid (rw) register accessor: an alias for `Reg<TZC_MM_BMX_TZMID_SPEC>`"]
pub type TZC_MM_BMX_TZMID = crate::Reg<tzc_mm_bmx_tzmid::TZC_MM_BMX_TZMID_SPEC>;
#[doc = "tzc_mm_bmx_tzmid"]
pub mod tzc_mm_bmx_tzmid;
#[doc = "tzc_mm_bmx_tzmid_lock (rw) register accessor: an alias for `Reg<TZC_MM_BMX_TZMID_LOCK_SPEC>`"]
pub type TZC_MM_BMX_TZMID_LOCK = crate::Reg<tzc_mm_bmx_tzmid_lock::TZC_MM_BMX_TZMID_LOCK_SPEC>;
#[doc = "tzc_mm_bmx_tzmid_lock"]
pub mod tzc_mm_bmx_tzmid_lock;
#[doc = "tzc_mm_bmx_s0 (rw) register accessor: an alias for `Reg<TZC_MM_BMX_S0_SPEC>`"]
pub type TZC_MM_BMX_S0 = crate::Reg<tzc_mm_bmx_s0::TZC_MM_BMX_S0_SPEC>;
#[doc = "tzc_mm_bmx_s0"]
pub mod tzc_mm_bmx_s0;
#[doc = "tzc_mm_bmx_s1 (rw) register accessor: an alias for `Reg<TZC_MM_BMX_S1_SPEC>`"]
pub type TZC_MM_BMX_S1 = crate::Reg<tzc_mm_bmx_s1::TZC_MM_BMX_S1_SPEC>;
#[doc = "tzc_mm_bmx_s1"]
pub mod tzc_mm_bmx_s1;
#[doc = "tzc_mm_bmx_s2 (rw) register accessor: an alias for `Reg<TZC_MM_BMX_S2_SPEC>`"]
pub type TZC_MM_BMX_S2 = crate::Reg<tzc_mm_bmx_s2::TZC_MM_BMX_S2_SPEC>;
#[doc = "tzc_mm_bmx_s2"]
pub mod tzc_mm_bmx_s2;
#[doc = "tzc_mm_bmx_s_lock0 (rw) register accessor: an alias for `Reg<TZC_MM_BMX_S_LOCK0_SPEC>`"]
pub type TZC_MM_BMX_S_LOCK0 = crate::Reg<tzc_mm_bmx_s_lock0::TZC_MM_BMX_S_LOCK0_SPEC>;
#[doc = "tzc_mm_bmx_s_lock0"]
pub mod tzc_mm_bmx_s_lock0;
#[doc = "tzc_mm_bmx_s_lock1 (rw) register accessor: an alias for `Reg<TZC_MM_BMX_S_LOCK1_SPEC>`"]
pub type TZC_MM_BMX_S_LOCK1 = crate::Reg<tzc_mm_bmx_s_lock1::TZC_MM_BMX_S_LOCK1_SPEC>;
#[doc = "tzc_mm_bmx_s_lock1"]
pub mod tzc_mm_bmx_s_lock1;
#[doc = "tzc_l2sram_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_L2SRAM_TZSRG_CTRL_SPEC>`"]
pub type TZC_L2SRAM_TZSRG_CTRL = crate::Reg<tzc_l2sram_tzsrg_ctrl::TZC_L2SRAM_TZSRG_CTRL_SPEC>;
#[doc = "tzc_l2sram_tzsrg_ctrl"]
pub mod tzc_l2sram_tzsrg_ctrl;
#[doc = "tzc_l2sram_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_L2SRAM_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_L2SRAM_TZSRG_ADR_MASK =
    crate::Reg<tzc_l2sram_tzsrg_adr_mask::TZC_L2SRAM_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_l2sram_tzsrg_adr_mask"]
pub mod tzc_l2sram_tzsrg_adr_mask;
#[doc = "tzc_l2sram_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_L2SRAM_TZSRG_R0_SPEC>`"]
pub type TZC_L2SRAM_TZSRG_R0 = crate::Reg<tzc_l2sram_tzsrg_r0::TZC_L2SRAM_TZSRG_R0_SPEC>;
#[doc = "tzc_l2sram_tzsrg_r0"]
pub mod tzc_l2sram_tzsrg_r0;
#[doc = "tzc_l2sram_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_L2SRAM_TZSRG_R1_SPEC>`"]
pub type TZC_L2SRAM_TZSRG_R1 = crate::Reg<tzc_l2sram_tzsrg_r1::TZC_L2SRAM_TZSRG_R1_SPEC>;
#[doc = "tzc_l2sram_tzsrg_r1"]
pub mod tzc_l2sram_tzsrg_r1;
#[doc = "tzc_l2sram_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_L2SRAM_TZSRG_R2_SPEC>`"]
pub type TZC_L2SRAM_TZSRG_R2 = crate::Reg<tzc_l2sram_tzsrg_r2::TZC_L2SRAM_TZSRG_R2_SPEC>;
#[doc = "tzc_l2sram_tzsrg_r2"]
pub mod tzc_l2sram_tzsrg_r2;
#[doc = "tzc_l2sram_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_L2SRAM_TZSRG_R3_SPEC>`"]
pub type TZC_L2SRAM_TZSRG_R3 = crate::Reg<tzc_l2sram_tzsrg_r3::TZC_L2SRAM_TZSRG_R3_SPEC>;
#[doc = "tzc_l2sram_tzsrg_r3"]
pub mod tzc_l2sram_tzsrg_r3;
#[doc = "tzc_vram_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_VRAM_TZSRG_CTRL_SPEC>`"]
pub type TZC_VRAM_TZSRG_CTRL = crate::Reg<tzc_vram_tzsrg_ctrl::TZC_VRAM_TZSRG_CTRL_SPEC>;
#[doc = "tzc_vram_tzsrg_ctrl"]
pub mod tzc_vram_tzsrg_ctrl;
#[doc = "tzc_vram_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_VRAM_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_VRAM_TZSRG_ADR_MASK =
    crate::Reg<tzc_vram_tzsrg_adr_mask::TZC_VRAM_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_vram_tzsrg_adr_mask"]
pub mod tzc_vram_tzsrg_adr_mask;
#[doc = "tzc_vram_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_VRAM_TZSRG_R0_SPEC>`"]
pub type TZC_VRAM_TZSRG_R0 = crate::Reg<tzc_vram_tzsrg_r0::TZC_VRAM_TZSRG_R0_SPEC>;
#[doc = "tzc_vram_tzsrg_r0"]
pub mod tzc_vram_tzsrg_r0;
#[doc = "tzc_vram_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_VRAM_TZSRG_R1_SPEC>`"]
pub type TZC_VRAM_TZSRG_R1 = crate::Reg<tzc_vram_tzsrg_r1::TZC_VRAM_TZSRG_R1_SPEC>;
#[doc = "tzc_vram_tzsrg_r1"]
pub mod tzc_vram_tzsrg_r1;
#[doc = "tzc_vram_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_VRAM_TZSRG_R2_SPEC>`"]
pub type TZC_VRAM_TZSRG_R2 = crate::Reg<tzc_vram_tzsrg_r2::TZC_VRAM_TZSRG_R2_SPEC>;
#[doc = "tzc_vram_tzsrg_r2"]
pub mod tzc_vram_tzsrg_r2;
#[doc = "tzc_vram_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_VRAM_TZSRG_R3_SPEC>`"]
pub type TZC_VRAM_TZSRG_R3 = crate::Reg<tzc_vram_tzsrg_r3::TZC_VRAM_TZSRG_R3_SPEC>;
#[doc = "tzc_vram_tzsrg_r3"]
pub mod tzc_vram_tzsrg_r3;
#[doc = "tzc_psrama_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_PSRAMA_TZSRG_CTRL_SPEC>`"]
pub type TZC_PSRAMA_TZSRG_CTRL = crate::Reg<tzc_psrama_tzsrg_ctrl::TZC_PSRAMA_TZSRG_CTRL_SPEC>;
#[doc = "tzc_psrama_tzsrg_ctrl"]
pub mod tzc_psrama_tzsrg_ctrl;
#[doc = "tzc_psrama_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_PSRAMA_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_PSRAMA_TZSRG_ADR_MASK =
    crate::Reg<tzc_psrama_tzsrg_adr_mask::TZC_PSRAMA_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_psrama_tzsrg_adr_mask"]
pub mod tzc_psrama_tzsrg_adr_mask;
#[doc = "tzc_psrama_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_PSRAMA_TZSRG_R0_SPEC>`"]
pub type TZC_PSRAMA_TZSRG_R0 = crate::Reg<tzc_psrama_tzsrg_r0::TZC_PSRAMA_TZSRG_R0_SPEC>;
#[doc = "tzc_psrama_tzsrg_r0"]
pub mod tzc_psrama_tzsrg_r0;
#[doc = "tzc_psrama_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_PSRAMA_TZSRG_R1_SPEC>`"]
pub type TZC_PSRAMA_TZSRG_R1 = crate::Reg<tzc_psrama_tzsrg_r1::TZC_PSRAMA_TZSRG_R1_SPEC>;
#[doc = "tzc_psrama_tzsrg_r1"]
pub mod tzc_psrama_tzsrg_r1;
#[doc = "tzc_psrama_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_PSRAMA_TZSRG_R2_SPEC>`"]
pub type TZC_PSRAMA_TZSRG_R2 = crate::Reg<tzc_psrama_tzsrg_r2::TZC_PSRAMA_TZSRG_R2_SPEC>;
#[doc = "tzc_psrama_tzsrg_r2"]
pub mod tzc_psrama_tzsrg_r2;
#[doc = "tzc_psrama_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_PSRAMA_TZSRG_R3_SPEC>`"]
pub type TZC_PSRAMA_TZSRG_R3 = crate::Reg<tzc_psrama_tzsrg_r3::TZC_PSRAMA_TZSRG_R3_SPEC>;
#[doc = "tzc_psrama_tzsrg_r3"]
pub mod tzc_psrama_tzsrg_r3;
#[doc = "tzc_psramb_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_PSRAMB_TZSRG_CTRL_SPEC>`"]
pub type TZC_PSRAMB_TZSRG_CTRL = crate::Reg<tzc_psramb_tzsrg_ctrl::TZC_PSRAMB_TZSRG_CTRL_SPEC>;
#[doc = "tzc_psramb_tzsrg_ctrl"]
pub mod tzc_psramb_tzsrg_ctrl;
#[doc = "tzc_psramb_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_PSRAMB_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_PSRAMB_TZSRG_ADR_MASK =
    crate::Reg<tzc_psramb_tzsrg_adr_mask::TZC_PSRAMB_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_psramb_tzsrg_adr_mask"]
pub mod tzc_psramb_tzsrg_adr_mask;
#[doc = "tzc_psramb_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_PSRAMB_TZSRG_R0_SPEC>`"]
pub type TZC_PSRAMB_TZSRG_R0 = crate::Reg<tzc_psramb_tzsrg_r0::TZC_PSRAMB_TZSRG_R0_SPEC>;
#[doc = "tzc_psramb_tzsrg_r0"]
pub mod tzc_psramb_tzsrg_r0;
#[doc = "tzc_psramb_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_PSRAMB_TZSRG_R1_SPEC>`"]
pub type TZC_PSRAMB_TZSRG_R1 = crate::Reg<tzc_psramb_tzsrg_r1::TZC_PSRAMB_TZSRG_R1_SPEC>;
#[doc = "tzc_psramb_tzsrg_r1"]
pub mod tzc_psramb_tzsrg_r1;
#[doc = "tzc_psramb_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_PSRAMB_TZSRG_R2_SPEC>`"]
pub type TZC_PSRAMB_TZSRG_R2 = crate::Reg<tzc_psramb_tzsrg_r2::TZC_PSRAMB_TZSRG_R2_SPEC>;
#[doc = "tzc_psramb_tzsrg_r2"]
pub mod tzc_psramb_tzsrg_r2;
#[doc = "tzc_psramb_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_PSRAMB_TZSRG_R3_SPEC>`"]
pub type TZC_PSRAMB_TZSRG_R3 = crate::Reg<tzc_psramb_tzsrg_r3::TZC_PSRAMB_TZSRG_R3_SPEC>;
#[doc = "tzc_psramb_tzsrg_r3"]
pub mod tzc_psramb_tzsrg_r3;
#[doc = "tzc_xram_tzsrg_ctrl (rw) register accessor: an alias for `Reg<TZC_XRAM_TZSRG_CTRL_SPEC>`"]
pub type TZC_XRAM_TZSRG_CTRL = crate::Reg<tzc_xram_tzsrg_ctrl::TZC_XRAM_TZSRG_CTRL_SPEC>;
#[doc = "tzc_xram_tzsrg_ctrl"]
pub mod tzc_xram_tzsrg_ctrl;
#[doc = "tzc_xram_tzsrg_adr_mask (rw) register accessor: an alias for `Reg<TZC_XRAM_TZSRG_ADR_MASK_SPEC>`"]
pub type TZC_XRAM_TZSRG_ADR_MASK =
    crate::Reg<tzc_xram_tzsrg_adr_mask::TZC_XRAM_TZSRG_ADR_MASK_SPEC>;
#[doc = "tzc_xram_tzsrg_adr_mask"]
pub mod tzc_xram_tzsrg_adr_mask;
#[doc = "tzc_xram_tzsrg_r0 (rw) register accessor: an alias for `Reg<TZC_XRAM_TZSRG_R0_SPEC>`"]
pub type TZC_XRAM_TZSRG_R0 = crate::Reg<tzc_xram_tzsrg_r0::TZC_XRAM_TZSRG_R0_SPEC>;
#[doc = "tzc_xram_tzsrg_r0"]
pub mod tzc_xram_tzsrg_r0;
#[doc = "tzc_xram_tzsrg_r1 (rw) register accessor: an alias for `Reg<TZC_XRAM_TZSRG_R1_SPEC>`"]
pub type TZC_XRAM_TZSRG_R1 = crate::Reg<tzc_xram_tzsrg_r1::TZC_XRAM_TZSRG_R1_SPEC>;
#[doc = "tzc_xram_tzsrg_r1"]
pub mod tzc_xram_tzsrg_r1;
#[doc = "tzc_xram_tzsrg_r2 (rw) register accessor: an alias for `Reg<TZC_XRAM_TZSRG_R2_SPEC>`"]
pub type TZC_XRAM_TZSRG_R2 = crate::Reg<tzc_xram_tzsrg_r2::TZC_XRAM_TZSRG_R2_SPEC>;
#[doc = "tzc_xram_tzsrg_r2"]
pub mod tzc_xram_tzsrg_r2;
#[doc = "tzc_xram_tzsrg_r3 (rw) register accessor: an alias for `Reg<TZC_XRAM_TZSRG_R3_SPEC>`"]
pub type TZC_XRAM_TZSRG_R3 = crate::Reg<tzc_xram_tzsrg_r3::TZC_XRAM_TZSRG_R3_SPEC>;
#[doc = "tzc_xram_tzsrg_r3"]
pub mod tzc_xram_tzsrg_r3;
#[doc = "tzc_glb_ctrl_0 (rw) register accessor: an alias for `Reg<TZC_GLB_CTRL_0_SPEC>`"]
pub type TZC_GLB_CTRL_0 = crate::Reg<tzc_glb_ctrl_0::TZC_GLB_CTRL_0_SPEC>;
#[doc = "tzc_glb_ctrl_0"]
pub mod tzc_glb_ctrl_0;
#[doc = "tzc_glb_ctrl_1 (rw) register accessor: an alias for `Reg<TZC_GLB_CTRL_1_SPEC>`"]
pub type TZC_GLB_CTRL_1 = crate::Reg<tzc_glb_ctrl_1::TZC_GLB_CTRL_1_SPEC>;
#[doc = "tzc_glb_ctrl_1"]
pub mod tzc_glb_ctrl_1;
#[doc = "tzc_glb_ctrl_2 (rw) register accessor: an alias for `Reg<TZC_GLB_CTRL_2_SPEC>`"]
pub type TZC_GLB_CTRL_2 = crate::Reg<tzc_glb_ctrl_2::TZC_GLB_CTRL_2_SPEC>;
#[doc = "tzc_glb_ctrl_2"]
pub mod tzc_glb_ctrl_2;
#[doc = "tzc_mm_ctrl_0 (rw) register accessor: an alias for `Reg<TZC_MM_CTRL_0_SPEC>`"]
pub type TZC_MM_CTRL_0 = crate::Reg<tzc_mm_ctrl_0::TZC_MM_CTRL_0_SPEC>;
#[doc = "tzc_mm_ctrl_0"]
pub mod tzc_mm_ctrl_0;
#[doc = "tzc_mm_ctrl_1 (rw) register accessor: an alias for `Reg<TZC_MM_CTRL_1_SPEC>`"]
pub type TZC_MM_CTRL_1 = crate::Reg<tzc_mm_ctrl_1::TZC_MM_CTRL_1_SPEC>;
#[doc = "tzc_mm_ctrl_1"]
pub mod tzc_mm_ctrl_1;
#[doc = "tzc_mm_ctrl_2 (rw) register accessor: an alias for `Reg<TZC_MM_CTRL_2_SPEC>`"]
pub type TZC_MM_CTRL_2 = crate::Reg<tzc_mm_ctrl_2::TZC_MM_CTRL_2_SPEC>;
#[doc = "tzc_mm_ctrl_2"]
pub mod tzc_mm_ctrl_2;
#[doc = "tzc_se_ctrl_0 (rw) register accessor: an alias for `Reg<TZC_SE_CTRL_0_SPEC>`"]
pub type TZC_SE_CTRL_0 = crate::Reg<tzc_se_ctrl_0::TZC_SE_CTRL_0_SPEC>;
#[doc = "tzc_se_ctrl_0"]
pub mod tzc_se_ctrl_0;
#[doc = "tzc_se_ctrl_1 (rw) register accessor: an alias for `Reg<TZC_SE_CTRL_1_SPEC>`"]
pub type TZC_SE_CTRL_1 = crate::Reg<tzc_se_ctrl_1::TZC_SE_CTRL_1_SPEC>;
#[doc = "tzc_se_ctrl_1"]
pub mod tzc_se_ctrl_1;
