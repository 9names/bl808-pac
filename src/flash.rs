#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_io_dly_0: [u8; 0x04],
    _reserved_1_io_dly_1: [u8; 0x04],
    _reserved_2_io_dly_2: [u8; 0x04],
    _reserved_3_io_dly_3: [u8; 0x04],
    _reserved_4_io_dly_4: [u8; 0x04],
    _reserved_5_sf: [u8; 0x04],
    _reserved_6_sf: [u8; 0x04],
    _reserved_7_sf: [u8; 0x04],
    _reserved_8_sf: [u8; 0x04],
    _reserved_9_sf: [u8; 0x04],
    _reserved_10_sf_: [u8; 0x04],
    _reserved_11_sf: [u8; 0x04],
    _reserved_12_sf: [u8; 0x04],
    #[doc = "0x34 - sf_if_io_dly_1"]
    pub sf_if_io_dly_1: SF_IF_IO_DLY_1,
    #[doc = "0x38 - sf_if_io_dly_2"]
    pub sf_if_io_dly_2: SF_IF_IO_DLY_2,
    #[doc = "0x3c - sf_if_io_dly_3"]
    pub sf_if_io_dly_3: SF_IF_IO_DLY_3,
    #[doc = "0x40 - sf_if_io_dly_4"]
    pub sf_if_io_dly_4: SF_IF_IO_DLY_4,
    #[doc = "0x44 - sf_reserved"]
    pub sf_reserved: SF_RESERVED,
    #[doc = "0x48 - sf2_if_io_dly_0"]
    pub sf2_if_io_dly_0: SF2_IF_IO_DLY_0,
    #[doc = "0x4c - sf2_if_io_dly_1"]
    pub sf2_if_io_dly_1: SF2_IF_IO_DLY_1,
    #[doc = "0x50 - sf2_if_io_dly_2"]
    pub sf2_if_io_dly_2: SF2_IF_IO_DLY_2,
    #[doc = "0x54 - sf2_if_io_dly_3"]
    pub sf2_if_io_dly_3: SF2_IF_IO_DLY_3,
    #[doc = "0x58 - sf2_if_io_dly_4"]
    pub sf2_if_io_dly_4: SF2_IF_IO_DLY_4,
    #[doc = "0x5c - sf3_if_io_dly_0"]
    pub sf3_if_io_dly_0: SF3_IF_IO_DLY_0,
    #[doc = "0x60 - sf3_if_io_dly_1"]
    pub sf3_if_io_dly_1: SF3_IF_IO_DLY_1,
    #[doc = "0x64 - sf3_if_io_dly_2"]
    pub sf3_if_io_dly_2: SF3_IF_IO_DLY_2,
    #[doc = "0x68 - sf3_if_io_dly_3"]
    pub sf3_if_io_dly_3: SF3_IF_IO_DLY_3,
    #[doc = "0x6c - sf3_if_io_dly_4"]
    pub sf3_if_io_dly_4: SF3_IF_IO_DLY_4,
    #[doc = "0x70 - sf_ctrl_2"]
    pub sf_ctrl_2: SF_CTRL_2,
    #[doc = "0x74 - sf_ctrl_3"]
    pub sf_ctrl_3: SF_CTRL_3,
    #[doc = "0x78 - sf_if_iahb_3"]
    pub sf_if_iahb_3: SF_IF_IAHB_3,
    #[doc = "0x7c - sf_if_iahb_4"]
    pub sf_if_iahb_4: SF_IF_IAHB_4,
    #[doc = "0x80 - sf_if_iahb_5"]
    pub sf_if_iahb_5: SF_IF_IAHB_5,
    #[doc = "0x84 - sf_if_iahb_6"]
    pub sf_if_iahb_6: SF_IF_IAHB_6,
    #[doc = "0x88 - sf_if_iahb_7"]
    pub sf_if_iahb_7: SF_IF_IAHB_7,
    #[doc = "0x8c - sf_if_iahb_8"]
    pub sf_if_iahb_8: SF_IF_IAHB_8,
    #[doc = "0x90 - sf_if_iahb_9"]
    pub sf_if_iahb_9: SF_IF_IAHB_9,
    #[doc = "0x94 - sf_if_iahb_10"]
    pub sf_if_iahb_10: SF_IF_IAHB_10,
    #[doc = "0x98 - sf_if_iahb_11"]
    pub sf_if_iahb_11: SF_IF_IAHB_11,
    #[doc = "0x9c - sf_if_iahb_12"]
    pub sf_if_iahb_12: SF_IF_IAHB_12,
    #[doc = "0xa0 - sf_id0_offset"]
    pub sf_id0_offset: SF_ID0_OFFSET,
    #[doc = "0xa4 - sf_id1_offset"]
    pub sf_id1_offset: SF_ID1_OFFSET,
    #[doc = "0xa8 - sf_bk2_id0_offset"]
    pub sf_bk2_id0_offset: SF_BK2_ID0_OFFSET,
    #[doc = "0xac - sf_bk2_id1_offset"]
    pub sf_bk2_id1_offset: SF_BK2_ID1_OFFSET,
    #[doc = "0xb0 - sf_dbg"]
    pub sf_dbg: SF_DBG,
    _reserved45: [u8; 0x0c],
    #[doc = "0xc0 - sf_if2_ctrl_0"]
    pub sf_if2_ctrl_0: SF_IF2_CTRL_0,
    #[doc = "0xc4 - sf_if2_ctrl_1"]
    pub sf_if2_ctrl_1: SF_IF2_CTRL_1,
    #[doc = "0xc8 - sf_if2_sahb_0"]
    pub sf_if2_sahb_0: SF_IF2_SAHB_0,
    #[doc = "0xcc - sf_if2_sahb_1"]
    pub sf_if2_sahb_1: SF_IF2_SAHB_1,
    #[doc = "0xd0 - sf_if2_sahb_2"]
    pub sf_if2_sahb_2: SF_IF2_SAHB_2,
    _reserved50: [u8; 0x2c],
    #[doc = "0x100 - sf_ctrl_prot_en_rd"]
    pub sf_ctrl_prot_en_rd: SF_CTRL_PROT_EN_RD,
    #[doc = "0x104 - sf_ctrl_prot_en"]
    pub sf_ctrl_prot_en: SF_CTRL_PROT_EN,
    _reserved52: [u8; 0xf8],
    #[doc = "0x200 - sf_aes_key_r0_0"]
    pub sf_aes_key_r0_0: SF_AES_KEY_R0_0,
    #[doc = "0x204 - sf_aes_key_r0_1"]
    pub sf_aes_key_r0_1: SF_AES_KEY_R0_1,
    #[doc = "0x208 - sf_aes_key_r0_2"]
    pub sf_aes_key_r0_2: SF_AES_KEY_R0_2,
    #[doc = "0x20c - sf_aes_key_r0_3"]
    pub sf_aes_key_r0_3: SF_AES_KEY_R0_3,
    #[doc = "0x210 - sf_aes_key_r0_4"]
    pub sf_aes_key_r0_4: SF_AES_KEY_R0_4,
    #[doc = "0x214 - sf_aes_key_r0_5"]
    pub sf_aes_key_r0_5: SF_AES_KEY_R0_5,
    #[doc = "0x218 - sf_aes_key_r0_6"]
    pub sf_aes_key_r0_6: SF_AES_KEY_R0_6,
    #[doc = "0x21c - sf_aes_key_r0_7"]
    pub sf_aes_key_r0_7: SF_AES_KEY_R0_7,
    #[doc = "0x220 - sf_aes_iv_r0_w0"]
    pub sf_aes_iv_r0_w0: SF_AES_IV_R0_W0,
    #[doc = "0x224 - sf_aes_iv_r0_w1"]
    pub sf_aes_iv_r0_w1: SF_AES_IV_R0_W1,
    #[doc = "0x228 - sf_aes_iv_r0_w2"]
    pub sf_aes_iv_r0_w2: SF_AES_IV_R0_W2,
    #[doc = "0x22c - sf_aes_iv_r0_w3"]
    pub sf_aes_iv_r0_w3: SF_AES_IV_R0_W3,
    #[doc = "0x230 - sf_aes_r0_start"]
    pub sf_aes_r0_start: SF_AES_R0_START,
    #[doc = "0x234 - sf_aes_r0_end"]
    pub sf_aes_r0_end: SF_AES_R0_END,
    _reserved66: [u8; 0x48],
    #[doc = "0x280 - sf_aes_key_r1_0"]
    pub sf_aes_key_r1_0: SF_AES_KEY_R1_0,
    #[doc = "0x284 - sf_aes_key_r1_1"]
    pub sf_aes_key_r1_1: SF_AES_KEY_R1_1,
    #[doc = "0x288 - sf_aes_key_r1_2"]
    pub sf_aes_key_r1_2: SF_AES_KEY_R1_2,
    #[doc = "0x28c - sf_aes_key_r1_3"]
    pub sf_aes_key_r1_3: SF_AES_KEY_R1_3,
    #[doc = "0x290 - sf_aes_key_r1_4"]
    pub sf_aes_key_r1_4: SF_AES_KEY_R1_4,
    #[doc = "0x294 - sf_aes_key_r1_5"]
    pub sf_aes_key_r1_5: SF_AES_KEY_R1_5,
    #[doc = "0x298 - sf_aes_key_r1_6"]
    pub sf_aes_key_r1_6: SF_AES_KEY_R1_6,
    #[doc = "0x29c - sf_aes_key_r1_7"]
    pub sf_aes_key_r1_7: SF_AES_KEY_R1_7,
    #[doc = "0x2a0 - sf_aes_iv_r1_w0"]
    pub sf_aes_iv_r1_w0: SF_AES_IV_R1_W0,
    #[doc = "0x2a4 - sf_aes_iv_r1_w1"]
    pub sf_aes_iv_r1_w1: SF_AES_IV_R1_W1,
    #[doc = "0x2a8 - sf_aes_iv_r1_w2"]
    pub sf_aes_iv_r1_w2: SF_AES_IV_R1_W2,
    #[doc = "0x2ac - sf_aes_iv_r1_w3"]
    pub sf_aes_iv_r1_w3: SF_AES_IV_R1_W3,
    #[doc = "0x2b0 - sf_aes_r1_start"]
    pub sf_aes_r1_start: SF_AES_R1_START,
    #[doc = "0x2b4 - sf_aes_r1_end"]
    pub sf_aes_r1_end: SF_AES_R1_END,
    _reserved80: [u8; 0x48],
    #[doc = "0x300 - sf_aes_key_r2_0"]
    pub sf_aes_key_r2_0: SF_AES_KEY_R2_0,
    #[doc = "0x304 - sf_aes_key_r2_1"]
    pub sf_aes_key_r2_1: SF_AES_KEY_R2_1,
    #[doc = "0x308 - sf_aes_key_r2_2"]
    pub sf_aes_key_r2_2: SF_AES_KEY_R2_2,
    #[doc = "0x30c - sf_aes_key_r2_3"]
    pub sf_aes_key_r2_3: SF_AES_KEY_R2_3,
    #[doc = "0x310 - sf_aes_key_r2_4"]
    pub sf_aes_key_r2_4: SF_AES_KEY_R2_4,
    #[doc = "0x314 - sf_aes_key_r2_5"]
    pub sf_aes_key_r2_5: SF_AES_KEY_R2_5,
    #[doc = "0x318 - sf_aes_key_r2_6"]
    pub sf_aes_key_r2_6: SF_AES_KEY_R2_6,
    #[doc = "0x31c - sf_aes_key_r2_7"]
    pub sf_aes_key_r2_7: SF_AES_KEY_R2_7,
    #[doc = "0x320 - sf_aes_iv_r2_w0"]
    pub sf_aes_iv_r2_w0: SF_AES_IV_R2_W0,
    #[doc = "0x324 - sf_aes_iv_r2_w1"]
    pub sf_aes_iv_r2_w1: SF_AES_IV_R2_W1,
    #[doc = "0x328 - sf_aes_iv_r2_w2"]
    pub sf_aes_iv_r2_w2: SF_AES_IV_R2_W2,
    #[doc = "0x32c - sf_aes_iv_r2_w3"]
    pub sf_aes_iv_r2_w3: SF_AES_IV_R2_W3,
    #[doc = "0x330 - sf_aes_r2_start"]
    pub sf_aes_r2_start: SF_AES_R2_START,
    #[doc = "0x334 - sf_aes_r2_end"]
    pub sf_aes_r2_end: SF_AES_R2_END,
}
impl RegisterBlock {
    #[doc = "0x00 - sf_aes_key_0"]
    #[inline(always)]
    pub const fn sf_aes_key_0(&self) -> &SF_AES_KEY_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - if_io_dly_0"]
    #[inline(always)]
    pub const fn io_dly_0(&self) -> &IO_DLY_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - sf_if_sahb_0"]
    #[inline(always)]
    pub const fn if_sahb_0(&self) -> &IF_SAHB_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - sf_ctrl_0"]
    #[inline(always)]
    pub const fn sf_ctrl_0(&self) -> &SF_CTRL_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - sf_aes_key_1"]
    #[inline(always)]
    pub const fn sf_aes_key_1(&self) -> &SF_AES_KEY_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - if_io_dly_1"]
    #[inline(always)]
    pub const fn io_dly_1(&self) -> &IO_DLY_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - sf_if_sahb_1"]
    #[inline(always)]
    pub const fn if_sahb_1(&self) -> &IF_SAHB_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - sf_ctrl_1"]
    #[inline(always)]
    pub const fn sf_ctrl_1(&self) -> &SF_CTRL_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - sf_aes_key_2"]
    #[inline(always)]
    pub const fn sf_aes_key_2(&self) -> &SF_AES_KEY_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - if_io_dly_2"]
    #[inline(always)]
    pub const fn io_dly_2(&self) -> &IO_DLY_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - sf_if_sahb_2"]
    #[inline(always)]
    pub const fn if_sahb_2(&self) -> &IF_SAHB_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - sf_if_sahb_0"]
    #[inline(always)]
    pub const fn sf_if_sahb_0(&self) -> &SF_IF_SAHB_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0c - sf_aes_key_3"]
    #[inline(always)]
    pub const fn sf_aes_key_3(&self) -> &SF_AES_KEY_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - if_io_dly_3"]
    #[inline(always)]
    pub const fn io_dly_3(&self) -> &IO_DLY_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - sf_if_sahb_1"]
    #[inline(always)]
    pub const fn sf_if_sahb_1(&self) -> &SF_IF_SAHB_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x10 - sf_aes_key_4"]
    #[inline(always)]
    pub const fn sf_aes_key_4(&self) -> &SF_AES_KEY_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - if_io_dly_4"]
    #[inline(always)]
    pub const fn io_dly_4(&self) -> &IO_DLY_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - sf_if_sahb_2"]
    #[inline(always)]
    pub const fn sf_if_sahb_2(&self) -> &SF_IF_SAHB_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x14 - sf_aes_key_5"]
    #[inline(always)]
    pub const fn sf_aes_key_5(&self) -> &SF_AES_KEY_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - sf_if_iahb_0"]
    #[inline(always)]
    pub const fn sf_if_iahb_0(&self) -> &SF_IF_IAHB_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x18 - sf_aes_key_6"]
    #[inline(always)]
    pub const fn sf_aes_key_6(&self) -> &SF_AES_KEY_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - sf_if_iahb_1"]
    #[inline(always)]
    pub const fn sf_if_iahb_1(&self) -> &SF_IF_IAHB_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - sf_aes_key_7"]
    #[inline(always)]
    pub const fn sf_aes_key_7(&self) -> &SF_AES_KEY_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - sf_if_iahb_2"]
    #[inline(always)]
    pub const fn sf_if_iahb_2(&self) -> &SF_IF_IAHB_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x20 - sf_aes_iv_w0"]
    #[inline(always)]
    pub const fn sf_aes_iv_w0(&self) -> &SF_AES_IV_W0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - sf_if_status_0"]
    #[inline(always)]
    pub const fn sf_if_status_0(&self) -> &SF_IF_STATUS_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x24 - sf_aes_iv_w1"]
    #[inline(always)]
    pub const fn sf_aes_iv_w1(&self) -> &SF_AES_IV_W1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x24 - sf_if_status_1"]
    #[inline(always)]
    pub const fn sf_if_status_1(&self) -> &SF_IF_STATUS_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x28 - sf_aes_iv_w2"]
    #[inline(always)]
    pub const fn sf_aes_iv_w2(&self) -> &SF_AES_IV_W2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - sf_aes"]
    #[inline(always)]
    pub const fn sf_aes(&self) -> &SF_AES {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x2c - sf_aes_iv_w3"]
    #[inline(always)]
    pub const fn sf_aes_iv_w3(&self) -> &SF_AES_IV_W3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2c - sf_ahb2sif_status"]
    #[inline(always)]
    pub const fn sf_ahb2sif_status(&self) -> &SF_AHB2SIF_STATUS {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x30 - sf_aes_start"]
    #[inline(always)]
    pub const fn sf_aes_start(&self) -> &SF_AES_START {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - sf_if_io_dly_0"]
    #[inline(always)]
    pub const fn sf_if_io_dly_0(&self) -> &SF_IF_IO_DLY_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
}
#[doc = "sf_ctrl_0 (rw) register accessor: an alias for `Reg<SF_CTRL_0_SPEC>`"]
pub type SF_CTRL_0 = crate::Reg<sf_ctrl_0::SF_CTRL_0_SPEC>;
#[doc = "sf_ctrl_0"]
pub mod sf_ctrl_0;
#[doc = "sf_ctrl_1 (rw) register accessor: an alias for `Reg<SF_CTRL_1_SPEC>`"]
pub type SF_CTRL_1 = crate::Reg<sf_ctrl_1::SF_CTRL_1_SPEC>;
#[doc = "sf_ctrl_1"]
pub mod sf_ctrl_1;
#[doc = "sf_if_sahb_0 (rw) register accessor: an alias for `Reg<SF_IF_SAHB_0_SPEC>`"]
pub type SF_IF_SAHB_0 = crate::Reg<sf_if_sahb_0::SF_IF_SAHB_0_SPEC>;
#[doc = "sf_if_sahb_0"]
pub mod sf_if_sahb_0;
#[doc = "sf_if_sahb_1 (rw) register accessor: an alias for `Reg<SF_IF_SAHB_1_SPEC>`"]
pub type SF_IF_SAHB_1 = crate::Reg<sf_if_sahb_1::SF_IF_SAHB_1_SPEC>;
#[doc = "sf_if_sahb_1"]
pub mod sf_if_sahb_1;
#[doc = "sf_if_sahb_2 (rw) register accessor: an alias for `Reg<SF_IF_SAHB_2_SPEC>`"]
pub type SF_IF_SAHB_2 = crate::Reg<sf_if_sahb_2::SF_IF_SAHB_2_SPEC>;
#[doc = "sf_if_sahb_2"]
pub mod sf_if_sahb_2;
#[doc = "sf_if_iahb_0 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_0_SPEC>`"]
pub type SF_IF_IAHB_0 = crate::Reg<sf_if_iahb_0::SF_IF_IAHB_0_SPEC>;
#[doc = "sf_if_iahb_0"]
pub mod sf_if_iahb_0;
#[doc = "sf_if_iahb_1 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_1_SPEC>`"]
pub type SF_IF_IAHB_1 = crate::Reg<sf_if_iahb_1::SF_IF_IAHB_1_SPEC>;
#[doc = "sf_if_iahb_1"]
pub mod sf_if_iahb_1;
#[doc = "sf_if_iahb_2 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_2_SPEC>`"]
pub type SF_IF_IAHB_2 = crate::Reg<sf_if_iahb_2::SF_IF_IAHB_2_SPEC>;
#[doc = "sf_if_iahb_2"]
pub mod sf_if_iahb_2;
#[doc = "sf_if_status_0 (rw) register accessor: an alias for `Reg<SF_IF_STATUS_0_SPEC>`"]
pub type SF_IF_STATUS_0 = crate::Reg<sf_if_status_0::SF_IF_STATUS_0_SPEC>;
#[doc = "sf_if_status_0"]
pub mod sf_if_status_0;
#[doc = "sf_if_status_1 (rw) register accessor: an alias for `Reg<SF_IF_STATUS_1_SPEC>`"]
pub type SF_IF_STATUS_1 = crate::Reg<sf_if_status_1::SF_IF_STATUS_1_SPEC>;
#[doc = "sf_if_status_1"]
pub mod sf_if_status_1;
#[doc = "sf_aes (rw) register accessor: an alias for `Reg<SF_AES_SPEC>`"]
pub type SF_AES = crate::Reg<sf_aes::SF_AES_SPEC>;
#[doc = "sf_aes"]
pub mod sf_aes;
#[doc = "sf_ahb2sif_status (rw) register accessor: an alias for `Reg<SF_AHB2SIF_STATUS_SPEC>`"]
pub type SF_AHB2SIF_STATUS = crate::Reg<sf_ahb2sif_status::SF_AHB2SIF_STATUS_SPEC>;
#[doc = "sf_ahb2sif_status"]
pub mod sf_ahb2sif_status;
#[doc = "sf_if_io_dly_0 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_0_SPEC>`"]
pub type SF_IF_IO_DLY_0 = crate::Reg<sf_if_io_dly_0::SF_IF_IO_DLY_0_SPEC>;
#[doc = "sf_if_io_dly_0"]
pub mod sf_if_io_dly_0;
#[doc = "sf_if_io_dly_1 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_1_SPEC>`"]
pub type SF_IF_IO_DLY_1 = crate::Reg<sf_if_io_dly_1::SF_IF_IO_DLY_1_SPEC>;
#[doc = "sf_if_io_dly_1"]
pub mod sf_if_io_dly_1;
#[doc = "sf_if_io_dly_2 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_2_SPEC>`"]
pub type SF_IF_IO_DLY_2 = crate::Reg<sf_if_io_dly_2::SF_IF_IO_DLY_2_SPEC>;
#[doc = "sf_if_io_dly_2"]
pub mod sf_if_io_dly_2;
#[doc = "sf_if_io_dly_3 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_3_SPEC>`"]
pub type SF_IF_IO_DLY_3 = crate::Reg<sf_if_io_dly_3::SF_IF_IO_DLY_3_SPEC>;
#[doc = "sf_if_io_dly_3"]
pub mod sf_if_io_dly_3;
#[doc = "sf_if_io_dly_4 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_4_SPEC>`"]
pub type SF_IF_IO_DLY_4 = crate::Reg<sf_if_io_dly_4::SF_IF_IO_DLY_4_SPEC>;
#[doc = "sf_if_io_dly_4"]
pub mod sf_if_io_dly_4;
#[doc = "sf_reserved (rw) register accessor: an alias for `Reg<SF_RESERVED_SPEC>`"]
pub type SF_RESERVED = crate::Reg<sf_reserved::SF_RESERVED_SPEC>;
#[doc = "sf_reserved"]
pub mod sf_reserved;
#[doc = "sf2_if_io_dly_0 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_0_SPEC>`"]
pub type SF2_IF_IO_DLY_0 = crate::Reg<sf2_if_io_dly_0::SF2_IF_IO_DLY_0_SPEC>;
#[doc = "sf2_if_io_dly_0"]
pub mod sf2_if_io_dly_0;
#[doc = "sf2_if_io_dly_1 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_1_SPEC>`"]
pub type SF2_IF_IO_DLY_1 = crate::Reg<sf2_if_io_dly_1::SF2_IF_IO_DLY_1_SPEC>;
#[doc = "sf2_if_io_dly_1"]
pub mod sf2_if_io_dly_1;
#[doc = "sf2_if_io_dly_2 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_2_SPEC>`"]
pub type SF2_IF_IO_DLY_2 = crate::Reg<sf2_if_io_dly_2::SF2_IF_IO_DLY_2_SPEC>;
#[doc = "sf2_if_io_dly_2"]
pub mod sf2_if_io_dly_2;
#[doc = "sf2_if_io_dly_3 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_3_SPEC>`"]
pub type SF2_IF_IO_DLY_3 = crate::Reg<sf2_if_io_dly_3::SF2_IF_IO_DLY_3_SPEC>;
#[doc = "sf2_if_io_dly_3"]
pub mod sf2_if_io_dly_3;
#[doc = "sf2_if_io_dly_4 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_4_SPEC>`"]
pub type SF2_IF_IO_DLY_4 = crate::Reg<sf2_if_io_dly_4::SF2_IF_IO_DLY_4_SPEC>;
#[doc = "sf2_if_io_dly_4"]
pub mod sf2_if_io_dly_4;
#[doc = "sf3_if_io_dly_0 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_0_SPEC>`"]
pub type SF3_IF_IO_DLY_0 = crate::Reg<sf3_if_io_dly_0::SF3_IF_IO_DLY_0_SPEC>;
#[doc = "sf3_if_io_dly_0"]
pub mod sf3_if_io_dly_0;
#[doc = "sf3_if_io_dly_1 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_1_SPEC>`"]
pub type SF3_IF_IO_DLY_1 = crate::Reg<sf3_if_io_dly_1::SF3_IF_IO_DLY_1_SPEC>;
#[doc = "sf3_if_io_dly_1"]
pub mod sf3_if_io_dly_1;
#[doc = "sf3_if_io_dly_2 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_2_SPEC>`"]
pub type SF3_IF_IO_DLY_2 = crate::Reg<sf3_if_io_dly_2::SF3_IF_IO_DLY_2_SPEC>;
#[doc = "sf3_if_io_dly_2"]
pub mod sf3_if_io_dly_2;
#[doc = "sf3_if_io_dly_3 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_3_SPEC>`"]
pub type SF3_IF_IO_DLY_3 = crate::Reg<sf3_if_io_dly_3::SF3_IF_IO_DLY_3_SPEC>;
#[doc = "sf3_if_io_dly_3"]
pub mod sf3_if_io_dly_3;
#[doc = "sf3_if_io_dly_4 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_4_SPEC>`"]
pub type SF3_IF_IO_DLY_4 = crate::Reg<sf3_if_io_dly_4::SF3_IF_IO_DLY_4_SPEC>;
#[doc = "sf3_if_io_dly_4"]
pub mod sf3_if_io_dly_4;
#[doc = "sf_ctrl_2 (rw) register accessor: an alias for `Reg<SF_CTRL_2_SPEC>`"]
pub type SF_CTRL_2 = crate::Reg<sf_ctrl_2::SF_CTRL_2_SPEC>;
#[doc = "sf_ctrl_2"]
pub mod sf_ctrl_2;
#[doc = "sf_ctrl_3 (rw) register accessor: an alias for `Reg<SF_CTRL_3_SPEC>`"]
pub type SF_CTRL_3 = crate::Reg<sf_ctrl_3::SF_CTRL_3_SPEC>;
#[doc = "sf_ctrl_3"]
pub mod sf_ctrl_3;
#[doc = "sf_if_iahb_3 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_3_SPEC>`"]
pub type SF_IF_IAHB_3 = crate::Reg<sf_if_iahb_3::SF_IF_IAHB_3_SPEC>;
#[doc = "sf_if_iahb_3"]
pub mod sf_if_iahb_3;
#[doc = "sf_if_iahb_4 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_4_SPEC>`"]
pub type SF_IF_IAHB_4 = crate::Reg<sf_if_iahb_4::SF_IF_IAHB_4_SPEC>;
#[doc = "sf_if_iahb_4"]
pub mod sf_if_iahb_4;
#[doc = "sf_if_iahb_5 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_5_SPEC>`"]
pub type SF_IF_IAHB_5 = crate::Reg<sf_if_iahb_5::SF_IF_IAHB_5_SPEC>;
#[doc = "sf_if_iahb_5"]
pub mod sf_if_iahb_5;
#[doc = "sf_if_iahb_6 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_6_SPEC>`"]
pub type SF_IF_IAHB_6 = crate::Reg<sf_if_iahb_6::SF_IF_IAHB_6_SPEC>;
#[doc = "sf_if_iahb_6"]
pub mod sf_if_iahb_6;
#[doc = "sf_if_iahb_7 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_7_SPEC>`"]
pub type SF_IF_IAHB_7 = crate::Reg<sf_if_iahb_7::SF_IF_IAHB_7_SPEC>;
#[doc = "sf_if_iahb_7"]
pub mod sf_if_iahb_7;
#[doc = "sf_if_iahb_8 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_8_SPEC>`"]
pub type SF_IF_IAHB_8 = crate::Reg<sf_if_iahb_8::SF_IF_IAHB_8_SPEC>;
#[doc = "sf_if_iahb_8"]
pub mod sf_if_iahb_8;
#[doc = "sf_if_iahb_9 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_9_SPEC>`"]
pub type SF_IF_IAHB_9 = crate::Reg<sf_if_iahb_9::SF_IF_IAHB_9_SPEC>;
#[doc = "sf_if_iahb_9"]
pub mod sf_if_iahb_9;
#[doc = "sf_if_iahb_10 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_10_SPEC>`"]
pub type SF_IF_IAHB_10 = crate::Reg<sf_if_iahb_10::SF_IF_IAHB_10_SPEC>;
#[doc = "sf_if_iahb_10"]
pub mod sf_if_iahb_10;
#[doc = "sf_if_iahb_11 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_11_SPEC>`"]
pub type SF_IF_IAHB_11 = crate::Reg<sf_if_iahb_11::SF_IF_IAHB_11_SPEC>;
#[doc = "sf_if_iahb_11"]
pub mod sf_if_iahb_11;
#[doc = "sf_if_iahb_12 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_12_SPEC>`"]
pub type SF_IF_IAHB_12 = crate::Reg<sf_if_iahb_12::SF_IF_IAHB_12_SPEC>;
#[doc = "sf_if_iahb_12"]
pub mod sf_if_iahb_12;
#[doc = "sf_id0_offset (rw) register accessor: an alias for `Reg<SF_ID0_OFFSET_SPEC>`"]
pub type SF_ID0_OFFSET = crate::Reg<sf_id0_offset::SF_ID0_OFFSET_SPEC>;
#[doc = "sf_id0_offset"]
pub mod sf_id0_offset;
#[doc = "sf_id1_offset (rw) register accessor: an alias for `Reg<SF_ID1_OFFSET_SPEC>`"]
pub type SF_ID1_OFFSET = crate::Reg<sf_id1_offset::SF_ID1_OFFSET_SPEC>;
#[doc = "sf_id1_offset"]
pub mod sf_id1_offset;
#[doc = "sf_bk2_id0_offset (rw) register accessor: an alias for `Reg<SF_BK2_ID0_OFFSET_SPEC>`"]
pub type SF_BK2_ID0_OFFSET = crate::Reg<sf_bk2_id0_offset::SF_BK2_ID0_OFFSET_SPEC>;
#[doc = "sf_bk2_id0_offset"]
pub mod sf_bk2_id0_offset;
#[doc = "sf_bk2_id1_offset (rw) register accessor: an alias for `Reg<SF_BK2_ID1_OFFSET_SPEC>`"]
pub type SF_BK2_ID1_OFFSET = crate::Reg<sf_bk2_id1_offset::SF_BK2_ID1_OFFSET_SPEC>;
#[doc = "sf_bk2_id1_offset"]
pub mod sf_bk2_id1_offset;
#[doc = "sf_dbg (rw) register accessor: an alias for `Reg<SF_DBG_SPEC>`"]
pub type SF_DBG = crate::Reg<sf_dbg::SF_DBG_SPEC>;
#[doc = "sf_dbg"]
pub mod sf_dbg;
#[doc = "sf_if2_ctrl_0 (rw) register accessor: an alias for `Reg<SF_IF2_CTRL_0_SPEC>`"]
pub type SF_IF2_CTRL_0 = crate::Reg<sf_if2_ctrl_0::SF_IF2_CTRL_0_SPEC>;
#[doc = "sf_if2_ctrl_0"]
pub mod sf_if2_ctrl_0;
#[doc = "sf_if2_ctrl_1 (rw) register accessor: an alias for `Reg<SF_IF2_CTRL_1_SPEC>`"]
pub type SF_IF2_CTRL_1 = crate::Reg<sf_if2_ctrl_1::SF_IF2_CTRL_1_SPEC>;
#[doc = "sf_if2_ctrl_1"]
pub mod sf_if2_ctrl_1;
#[doc = "sf_if2_sahb_0 (rw) register accessor: an alias for `Reg<SF_IF2_SAHB_0_SPEC>`"]
pub type SF_IF2_SAHB_0 = crate::Reg<sf_if2_sahb_0::SF_IF2_SAHB_0_SPEC>;
#[doc = "sf_if2_sahb_0"]
pub mod sf_if2_sahb_0;
#[doc = "sf_if2_sahb_1 (rw) register accessor: an alias for `Reg<SF_IF2_SAHB_1_SPEC>`"]
pub type SF_IF2_SAHB_1 = crate::Reg<sf_if2_sahb_1::SF_IF2_SAHB_1_SPEC>;
#[doc = "sf_if2_sahb_1"]
pub mod sf_if2_sahb_1;
#[doc = "sf_if2_sahb_2 (rw) register accessor: an alias for `Reg<SF_IF2_SAHB_2_SPEC>`"]
pub type SF_IF2_SAHB_2 = crate::Reg<sf_if2_sahb_2::SF_IF2_SAHB_2_SPEC>;
#[doc = "sf_if2_sahb_2"]
pub mod sf_if2_sahb_2;
#[doc = "sf_ctrl_prot_en_rd (rw) register accessor: an alias for `Reg<SF_CTRL_PROT_EN_RD_SPEC>`"]
pub type SF_CTRL_PROT_EN_RD = crate::Reg<sf_ctrl_prot_en_rd::SF_CTRL_PROT_EN_RD_SPEC>;
#[doc = "sf_ctrl_prot_en_rd"]
pub mod sf_ctrl_prot_en_rd;
#[doc = "sf_ctrl_prot_en (rw) register accessor: an alias for `Reg<SF_CTRL_PROT_EN_SPEC>`"]
pub type SF_CTRL_PROT_EN = crate::Reg<sf_ctrl_prot_en::SF_CTRL_PROT_EN_SPEC>;
#[doc = "sf_ctrl_prot_en"]
pub mod sf_ctrl_prot_en;
#[doc = "sf_aes_key_r0_0 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_0_SPEC>`"]
pub type SF_AES_KEY_R0_0 = crate::Reg<sf_aes_key_r0_0::SF_AES_KEY_R0_0_SPEC>;
#[doc = "sf_aes_key_r0_0"]
pub mod sf_aes_key_r0_0;
#[doc = "sf_aes_key_r0_1 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_1_SPEC>`"]
pub type SF_AES_KEY_R0_1 = crate::Reg<sf_aes_key_r0_1::SF_AES_KEY_R0_1_SPEC>;
#[doc = "sf_aes_key_r0_1"]
pub mod sf_aes_key_r0_1;
#[doc = "sf_aes_key_r0_2 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_2_SPEC>`"]
pub type SF_AES_KEY_R0_2 = crate::Reg<sf_aes_key_r0_2::SF_AES_KEY_R0_2_SPEC>;
#[doc = "sf_aes_key_r0_2"]
pub mod sf_aes_key_r0_2;
#[doc = "sf_aes_key_r0_3 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_3_SPEC>`"]
pub type SF_AES_KEY_R0_3 = crate::Reg<sf_aes_key_r0_3::SF_AES_KEY_R0_3_SPEC>;
#[doc = "sf_aes_key_r0_3"]
pub mod sf_aes_key_r0_3;
#[doc = "sf_aes_key_r0_4 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_4_SPEC>`"]
pub type SF_AES_KEY_R0_4 = crate::Reg<sf_aes_key_r0_4::SF_AES_KEY_R0_4_SPEC>;
#[doc = "sf_aes_key_r0_4"]
pub mod sf_aes_key_r0_4;
#[doc = "sf_aes_key_r0_5 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_5_SPEC>`"]
pub type SF_AES_KEY_R0_5 = crate::Reg<sf_aes_key_r0_5::SF_AES_KEY_R0_5_SPEC>;
#[doc = "sf_aes_key_r0_5"]
pub mod sf_aes_key_r0_5;
#[doc = "sf_aes_key_r0_6 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_6_SPEC>`"]
pub type SF_AES_KEY_R0_6 = crate::Reg<sf_aes_key_r0_6::SF_AES_KEY_R0_6_SPEC>;
#[doc = "sf_aes_key_r0_6"]
pub mod sf_aes_key_r0_6;
#[doc = "sf_aes_key_r0_7 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_7_SPEC>`"]
pub type SF_AES_KEY_R0_7 = crate::Reg<sf_aes_key_r0_7::SF_AES_KEY_R0_7_SPEC>;
#[doc = "sf_aes_key_r0_7"]
pub mod sf_aes_key_r0_7;
#[doc = "sf_aes_iv_r0_w0 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W0_SPEC>`"]
pub type SF_AES_IV_R0_W0 = crate::Reg<sf_aes_iv_r0_w0::SF_AES_IV_R0_W0_SPEC>;
#[doc = "sf_aes_iv_r0_w0"]
pub mod sf_aes_iv_r0_w0;
#[doc = "sf_aes_iv_r0_w1 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W1_SPEC>`"]
pub type SF_AES_IV_R0_W1 = crate::Reg<sf_aes_iv_r0_w1::SF_AES_IV_R0_W1_SPEC>;
#[doc = "sf_aes_iv_r0_w1"]
pub mod sf_aes_iv_r0_w1;
#[doc = "sf_aes_iv_r0_w2 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W2_SPEC>`"]
pub type SF_AES_IV_R0_W2 = crate::Reg<sf_aes_iv_r0_w2::SF_AES_IV_R0_W2_SPEC>;
#[doc = "sf_aes_iv_r0_w2"]
pub mod sf_aes_iv_r0_w2;
#[doc = "sf_aes_iv_r0_w3 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W3_SPEC>`"]
pub type SF_AES_IV_R0_W3 = crate::Reg<sf_aes_iv_r0_w3::SF_AES_IV_R0_W3_SPEC>;
#[doc = "sf_aes_iv_r0_w3"]
pub mod sf_aes_iv_r0_w3;
#[doc = "sf_aes_r0_start (rw) register accessor: an alias for `Reg<SF_AES_R0_START_SPEC>`"]
pub type SF_AES_R0_START = crate::Reg<sf_aes_r0_start::SF_AES_R0_START_SPEC>;
#[doc = "sf_aes_r0_start"]
pub mod sf_aes_r0_start;
#[doc = "sf_aes_r0_end (rw) register accessor: an alias for `Reg<SF_AES_R0_END_SPEC>`"]
pub type SF_AES_R0_END = crate::Reg<sf_aes_r0_end::SF_AES_R0_END_SPEC>;
#[doc = "sf_aes_r0_end"]
pub mod sf_aes_r0_end;
#[doc = "sf_aes_key_r1_0 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_0_SPEC>`"]
pub type SF_AES_KEY_R1_0 = crate::Reg<sf_aes_key_r1_0::SF_AES_KEY_R1_0_SPEC>;
#[doc = "sf_aes_key_r1_0"]
pub mod sf_aes_key_r1_0;
#[doc = "sf_aes_key_r1_1 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_1_SPEC>`"]
pub type SF_AES_KEY_R1_1 = crate::Reg<sf_aes_key_r1_1::SF_AES_KEY_R1_1_SPEC>;
#[doc = "sf_aes_key_r1_1"]
pub mod sf_aes_key_r1_1;
#[doc = "sf_aes_key_r1_2 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_2_SPEC>`"]
pub type SF_AES_KEY_R1_2 = crate::Reg<sf_aes_key_r1_2::SF_AES_KEY_R1_2_SPEC>;
#[doc = "sf_aes_key_r1_2"]
pub mod sf_aes_key_r1_2;
#[doc = "sf_aes_key_r1_3 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_3_SPEC>`"]
pub type SF_AES_KEY_R1_3 = crate::Reg<sf_aes_key_r1_3::SF_AES_KEY_R1_3_SPEC>;
#[doc = "sf_aes_key_r1_3"]
pub mod sf_aes_key_r1_3;
#[doc = "sf_aes_key_r1_4 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_4_SPEC>`"]
pub type SF_AES_KEY_R1_4 = crate::Reg<sf_aes_key_r1_4::SF_AES_KEY_R1_4_SPEC>;
#[doc = "sf_aes_key_r1_4"]
pub mod sf_aes_key_r1_4;
#[doc = "sf_aes_key_r1_5 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_5_SPEC>`"]
pub type SF_AES_KEY_R1_5 = crate::Reg<sf_aes_key_r1_5::SF_AES_KEY_R1_5_SPEC>;
#[doc = "sf_aes_key_r1_5"]
pub mod sf_aes_key_r1_5;
#[doc = "sf_aes_key_r1_6 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_6_SPEC>`"]
pub type SF_AES_KEY_R1_6 = crate::Reg<sf_aes_key_r1_6::SF_AES_KEY_R1_6_SPEC>;
#[doc = "sf_aes_key_r1_6"]
pub mod sf_aes_key_r1_6;
#[doc = "sf_aes_key_r1_7 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_7_SPEC>`"]
pub type SF_AES_KEY_R1_7 = crate::Reg<sf_aes_key_r1_7::SF_AES_KEY_R1_7_SPEC>;
#[doc = "sf_aes_key_r1_7"]
pub mod sf_aes_key_r1_7;
#[doc = "sf_aes_iv_r1_w0 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W0_SPEC>`"]
pub type SF_AES_IV_R1_W0 = crate::Reg<sf_aes_iv_r1_w0::SF_AES_IV_R1_W0_SPEC>;
#[doc = "sf_aes_iv_r1_w0"]
pub mod sf_aes_iv_r1_w0;
#[doc = "sf_aes_iv_r1_w1 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W1_SPEC>`"]
pub type SF_AES_IV_R1_W1 = crate::Reg<sf_aes_iv_r1_w1::SF_AES_IV_R1_W1_SPEC>;
#[doc = "sf_aes_iv_r1_w1"]
pub mod sf_aes_iv_r1_w1;
#[doc = "sf_aes_iv_r1_w2 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W2_SPEC>`"]
pub type SF_AES_IV_R1_W2 = crate::Reg<sf_aes_iv_r1_w2::SF_AES_IV_R1_W2_SPEC>;
#[doc = "sf_aes_iv_r1_w2"]
pub mod sf_aes_iv_r1_w2;
#[doc = "sf_aes_iv_r1_w3 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W3_SPEC>`"]
pub type SF_AES_IV_R1_W3 = crate::Reg<sf_aes_iv_r1_w3::SF_AES_IV_R1_W3_SPEC>;
#[doc = "sf_aes_iv_r1_w3"]
pub mod sf_aes_iv_r1_w3;
#[doc = "sf_aes_r1_start (rw) register accessor: an alias for `Reg<SF_AES_R1_START_SPEC>`"]
pub type SF_AES_R1_START = crate::Reg<sf_aes_r1_start::SF_AES_R1_START_SPEC>;
#[doc = "sf_aes_r1_start"]
pub mod sf_aes_r1_start;
#[doc = "sf_aes_r1_end (rw) register accessor: an alias for `Reg<SF_AES_R1_END_SPEC>`"]
pub type SF_AES_R1_END = crate::Reg<sf_aes_r1_end::SF_AES_R1_END_SPEC>;
#[doc = "sf_aes_r1_end"]
pub mod sf_aes_r1_end;
#[doc = "sf_aes_key_r2_0 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_0_SPEC>`"]
pub type SF_AES_KEY_R2_0 = crate::Reg<sf_aes_key_r2_0::SF_AES_KEY_R2_0_SPEC>;
#[doc = "sf_aes_key_r2_0"]
pub mod sf_aes_key_r2_0;
#[doc = "sf_aes_key_r2_1 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_1_SPEC>`"]
pub type SF_AES_KEY_R2_1 = crate::Reg<sf_aes_key_r2_1::SF_AES_KEY_R2_1_SPEC>;
#[doc = "sf_aes_key_r2_1"]
pub mod sf_aes_key_r2_1;
#[doc = "sf_aes_key_r2_2 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_2_SPEC>`"]
pub type SF_AES_KEY_R2_2 = crate::Reg<sf_aes_key_r2_2::SF_AES_KEY_R2_2_SPEC>;
#[doc = "sf_aes_key_r2_2"]
pub mod sf_aes_key_r2_2;
#[doc = "sf_aes_key_r2_3 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_3_SPEC>`"]
pub type SF_AES_KEY_R2_3 = crate::Reg<sf_aes_key_r2_3::SF_AES_KEY_R2_3_SPEC>;
#[doc = "sf_aes_key_r2_3"]
pub mod sf_aes_key_r2_3;
#[doc = "sf_aes_key_r2_4 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_4_SPEC>`"]
pub type SF_AES_KEY_R2_4 = crate::Reg<sf_aes_key_r2_4::SF_AES_KEY_R2_4_SPEC>;
#[doc = "sf_aes_key_r2_4"]
pub mod sf_aes_key_r2_4;
#[doc = "sf_aes_key_r2_5 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_5_SPEC>`"]
pub type SF_AES_KEY_R2_5 = crate::Reg<sf_aes_key_r2_5::SF_AES_KEY_R2_5_SPEC>;
#[doc = "sf_aes_key_r2_5"]
pub mod sf_aes_key_r2_5;
#[doc = "sf_aes_key_r2_6 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_6_SPEC>`"]
pub type SF_AES_KEY_R2_6 = crate::Reg<sf_aes_key_r2_6::SF_AES_KEY_R2_6_SPEC>;
#[doc = "sf_aes_key_r2_6"]
pub mod sf_aes_key_r2_6;
#[doc = "sf_aes_key_r2_7 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_7_SPEC>`"]
pub type SF_AES_KEY_R2_7 = crate::Reg<sf_aes_key_r2_7::SF_AES_KEY_R2_7_SPEC>;
#[doc = "sf_aes_key_r2_7"]
pub mod sf_aes_key_r2_7;
#[doc = "sf_aes_iv_r2_w0 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W0_SPEC>`"]
pub type SF_AES_IV_R2_W0 = crate::Reg<sf_aes_iv_r2_w0::SF_AES_IV_R2_W0_SPEC>;
#[doc = "sf_aes_iv_r2_w0"]
pub mod sf_aes_iv_r2_w0;
#[doc = "sf_aes_iv_r2_w1 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W1_SPEC>`"]
pub type SF_AES_IV_R2_W1 = crate::Reg<sf_aes_iv_r2_w1::SF_AES_IV_R2_W1_SPEC>;
#[doc = "sf_aes_iv_r2_w1"]
pub mod sf_aes_iv_r2_w1;
#[doc = "sf_aes_iv_r2_w2 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W2_SPEC>`"]
pub type SF_AES_IV_R2_W2 = crate::Reg<sf_aes_iv_r2_w2::SF_AES_IV_R2_W2_SPEC>;
#[doc = "sf_aes_iv_r2_w2"]
pub mod sf_aes_iv_r2_w2;
#[doc = "sf_aes_iv_r2_w3 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W3_SPEC>`"]
pub type SF_AES_IV_R2_W3 = crate::Reg<sf_aes_iv_r2_w3::SF_AES_IV_R2_W3_SPEC>;
#[doc = "sf_aes_iv_r2_w3"]
pub mod sf_aes_iv_r2_w3;
#[doc = "sf_aes_r2_start (rw) register accessor: an alias for `Reg<SF_AES_R2_START_SPEC>`"]
pub type SF_AES_R2_START = crate::Reg<sf_aes_r2_start::SF_AES_R2_START_SPEC>;
#[doc = "sf_aes_r2_start"]
pub mod sf_aes_r2_start;
#[doc = "sf_aes_r2_end (rw) register accessor: an alias for `Reg<SF_AES_R2_END_SPEC>`"]
pub type SF_AES_R2_END = crate::Reg<sf_aes_r2_end::SF_AES_R2_END_SPEC>;
#[doc = "sf_aes_r2_end"]
pub mod sf_aes_r2_end;
#[doc = "if_sahb_0 (rw) register accessor: an alias for `Reg<IF_SAHB_0_SPEC>`"]
pub type IF_SAHB_0 = crate::Reg<if_sahb_0::IF_SAHB_0_SPEC>;
#[doc = "sf_if_sahb_0"]
pub mod if_sahb_0;
#[doc = "if_sahb_1 (rw) register accessor: an alias for `Reg<IF_SAHB_1_SPEC>`"]
pub type IF_SAHB_1 = crate::Reg<if_sahb_1::IF_SAHB_1_SPEC>;
#[doc = "sf_if_sahb_1"]
pub mod if_sahb_1;
#[doc = "if_sahb_2 (rw) register accessor: an alias for `Reg<IF_SAHB_2_SPEC>`"]
pub type IF_SAHB_2 = crate::Reg<if_sahb_2::IF_SAHB_2_SPEC>;
#[doc = "sf_if_sahb_2"]
pub mod if_sahb_2;
#[doc = "io_dly_0 (rw) register accessor: an alias for `Reg<IO_DLY_0_SPEC>`"]
pub type IO_DLY_0 = crate::Reg<io_dly_0::IO_DLY_0_SPEC>;
#[doc = "if_io_dly_0"]
pub mod io_dly_0;
#[doc = "io_dly_1 (rw) register accessor: an alias for `Reg<IO_DLY_1_SPEC>`"]
pub type IO_DLY_1 = crate::Reg<io_dly_1::IO_DLY_1_SPEC>;
#[doc = "if_io_dly_1"]
pub mod io_dly_1;
#[doc = "io_dly_2 (rw) register accessor: an alias for `Reg<IO_DLY_2_SPEC>`"]
pub type IO_DLY_2 = crate::Reg<io_dly_2::IO_DLY_2_SPEC>;
#[doc = "if_io_dly_2"]
pub mod io_dly_2;
#[doc = "io_dly_3 (rw) register accessor: an alias for `Reg<IO_DLY_3_SPEC>`"]
pub type IO_DLY_3 = crate::Reg<io_dly_3::IO_DLY_3_SPEC>;
#[doc = "if_io_dly_3"]
pub mod io_dly_3;
#[doc = "io_dly_4 (rw) register accessor: an alias for `Reg<IO_DLY_4_SPEC>`"]
pub type IO_DLY_4 = crate::Reg<io_dly_4::IO_DLY_4_SPEC>;
#[doc = "if_io_dly_4"]
pub mod io_dly_4;
#[doc = "sf_aes_key_0 (rw) register accessor: an alias for `Reg<SF_AES_KEY_0_SPEC>`"]
pub type SF_AES_KEY_0 = crate::Reg<sf_aes_key_0::SF_AES_KEY_0_SPEC>;
#[doc = "sf_aes_key_0"]
pub mod sf_aes_key_0;
#[doc = "sf_aes_key_1 (rw) register accessor: an alias for `Reg<SF_AES_KEY_1_SPEC>`"]
pub type SF_AES_KEY_1 = crate::Reg<sf_aes_key_1::SF_AES_KEY_1_SPEC>;
#[doc = "sf_aes_key_1"]
pub mod sf_aes_key_1;
#[doc = "sf_aes_key_2 (rw) register accessor: an alias for `Reg<SF_AES_KEY_2_SPEC>`"]
pub type SF_AES_KEY_2 = crate::Reg<sf_aes_key_2::SF_AES_KEY_2_SPEC>;
#[doc = "sf_aes_key_2"]
pub mod sf_aes_key_2;
#[doc = "sf_aes_key_3 (rw) register accessor: an alias for `Reg<SF_AES_KEY_3_SPEC>`"]
pub type SF_AES_KEY_3 = crate::Reg<sf_aes_key_3::SF_AES_KEY_3_SPEC>;
#[doc = "sf_aes_key_3"]
pub mod sf_aes_key_3;
#[doc = "sf_aes_key_4 (rw) register accessor: an alias for `Reg<SF_AES_KEY_4_SPEC>`"]
pub type SF_AES_KEY_4 = crate::Reg<sf_aes_key_4::SF_AES_KEY_4_SPEC>;
#[doc = "sf_aes_key_4"]
pub mod sf_aes_key_4;
#[doc = "sf_aes_key_5 (rw) register accessor: an alias for `Reg<SF_AES_KEY_5_SPEC>`"]
pub type SF_AES_KEY_5 = crate::Reg<sf_aes_key_5::SF_AES_KEY_5_SPEC>;
#[doc = "sf_aes_key_5"]
pub mod sf_aes_key_5;
#[doc = "sf_aes_key_6 (rw) register accessor: an alias for `Reg<SF_AES_KEY_6_SPEC>`"]
pub type SF_AES_KEY_6 = crate::Reg<sf_aes_key_6::SF_AES_KEY_6_SPEC>;
#[doc = "sf_aes_key_6"]
pub mod sf_aes_key_6;
#[doc = "sf_aes_key_7 (rw) register accessor: an alias for `Reg<SF_AES_KEY_7_SPEC>`"]
pub type SF_AES_KEY_7 = crate::Reg<sf_aes_key_7::SF_AES_KEY_7_SPEC>;
#[doc = "sf_aes_key_7"]
pub mod sf_aes_key_7;
#[doc = "sf_aes_iv_w0 (rw) register accessor: an alias for `Reg<SF_AES_IV_W0_SPEC>`"]
pub type SF_AES_IV_W0 = crate::Reg<sf_aes_iv_w0::SF_AES_IV_W0_SPEC>;
#[doc = "sf_aes_iv_w0"]
pub mod sf_aes_iv_w0;
#[doc = "sf_aes_iv_w1 (rw) register accessor: an alias for `Reg<SF_AES_IV_W1_SPEC>`"]
pub type SF_AES_IV_W1 = crate::Reg<sf_aes_iv_w1::SF_AES_IV_W1_SPEC>;
#[doc = "sf_aes_iv_w1"]
pub mod sf_aes_iv_w1;
#[doc = "sf_aes_iv_w2 (rw) register accessor: an alias for `Reg<SF_AES_IV_W2_SPEC>`"]
pub type SF_AES_IV_W2 = crate::Reg<sf_aes_iv_w2::SF_AES_IV_W2_SPEC>;
#[doc = "sf_aes_iv_w2"]
pub mod sf_aes_iv_w2;
#[doc = "sf_aes_iv_w3 (rw) register accessor: an alias for `Reg<SF_AES_IV_W3_SPEC>`"]
pub type SF_AES_IV_W3 = crate::Reg<sf_aes_iv_w3::SF_AES_IV_W3_SPEC>;
#[doc = "sf_aes_iv_w3"]
pub mod sf_aes_iv_w3;
#[doc = "sf_aes_start (rw) register accessor: an alias for `Reg<SF_AES_START_SPEC>`"]
pub type SF_AES_START = crate::Reg<sf_aes_start::SF_AES_START_SPEC>;
#[doc = "sf_aes_start"]
pub mod sf_aes_start;
