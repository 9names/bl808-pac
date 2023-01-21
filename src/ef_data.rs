#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ef_cfg_0"]
    pub ef_cfg_0: EF_CFG_0,
    #[doc = "0x04 - ef_dbg_pwd_low"]
    pub ef_dbg_pwd_low: EF_DBG_PWD_LOW,
    #[doc = "0x08 - ef_dbg_pwd_high"]
    pub ef_dbg_pwd_high: EF_DBG_PWD_HIGH,
    #[doc = "0x0c - ef_dbg_pwd2_low"]
    pub ef_dbg_pwd2_low: EF_DBG_PWD2_LOW,
    #[doc = "0x10 - ef_dbg_pwd2_high"]
    pub ef_dbg_pwd2_high: EF_DBG_PWD2_HIGH,
    #[doc = "0x14 - ef_wifi_mac_low"]
    pub ef_wifi_mac_low: EF_WIFI_MAC_LOW,
    #[doc = "0x18 - ef_wifi_mac_high"]
    pub ef_wifi_mac_high: EF_WIFI_MAC_HIGH,
    #[doc = "0x1c - ef_key_slot_0_w0"]
    pub ef_key_slot_0_w0: EF_KEY_SLOT_0_W0,
    #[doc = "0x20 - ef_key_slot_0_w1"]
    pub ef_key_slot_0_w1: EF_KEY_SLOT_0_W1,
    #[doc = "0x24 - ef_key_slot_0_w2"]
    pub ef_key_slot_0_w2: EF_KEY_SLOT_0_W2,
    #[doc = "0x28 - ef_key_slot_0_w3"]
    pub ef_key_slot_0_w3: EF_KEY_SLOT_0_W3,
    #[doc = "0x2c - ef_key_slot_1_w0"]
    pub ef_key_slot_1_w0: EF_KEY_SLOT_1_W0,
    #[doc = "0x30 - ef_key_slot_1_w1"]
    pub ef_key_slot_1_w1: EF_KEY_SLOT_1_W1,
    #[doc = "0x34 - ef_key_slot_1_w2"]
    pub ef_key_slot_1_w2: EF_KEY_SLOT_1_W2,
    #[doc = "0x38 - ef_key_slot_1_w3"]
    pub ef_key_slot_1_w3: EF_KEY_SLOT_1_W3,
    #[doc = "0x3c - ef_key_slot_2_w0"]
    pub ef_key_slot_2_w0: EF_KEY_SLOT_2_W0,
    #[doc = "0x40 - ef_key_slot_2_w1"]
    pub ef_key_slot_2_w1: EF_KEY_SLOT_2_W1,
    #[doc = "0x44 - ef_key_slot_2_w2"]
    pub ef_key_slot_2_w2: EF_KEY_SLOT_2_W2,
    #[doc = "0x48 - ef_key_slot_2_w3"]
    pub ef_key_slot_2_w3: EF_KEY_SLOT_2_W3,
    #[doc = "0x4c - ef_key_slot_3_w0"]
    pub ef_key_slot_3_w0: EF_KEY_SLOT_3_W0,
    #[doc = "0x50 - ef_key_slot_3_w1"]
    pub ef_key_slot_3_w1: EF_KEY_SLOT_3_W1,
    #[doc = "0x54 - ef_key_slot_3_w2"]
    pub ef_key_slot_3_w2: EF_KEY_SLOT_3_W2,
    #[doc = "0x58 - ef_key_slot_3_w3"]
    pub ef_key_slot_3_w3: EF_KEY_SLOT_3_W3,
    #[doc = "0x5c - ef_sw_usage_0"]
    pub ef_sw_usage_0: EF_SW_USAGE_0,
    #[doc = "0x60 - ef_sw_usage_1"]
    pub ef_sw_usage_1: EF_SW_USAGE_1,
    #[doc = "0x64 - ef_sw_usage_2"]
    pub ef_sw_usage_2: EF_SW_USAGE_2,
    #[doc = "0x68 - ef_sw_usage_3"]
    pub ef_sw_usage_3: EF_SW_USAGE_3,
    #[doc = "0x6c - ef_key_slot_11_w0"]
    pub ef_key_slot_11_w0: EF_KEY_SLOT_11_W0,
    #[doc = "0x70 - ef_key_slot_11_w1"]
    pub ef_key_slot_11_w1: EF_KEY_SLOT_11_W1,
    #[doc = "0x74 - ef_key_slot_11_w2"]
    pub ef_key_slot_11_w2: EF_KEY_SLOT_11_W2,
    #[doc = "0x78 - ef_key_slot_11_w3"]
    pub ef_key_slot_11_w3: EF_KEY_SLOT_11_W3,
    #[doc = "0x7c - ef_data_0_lock"]
    pub ef_data_0_lock: EF_DATA_0_LOCK,
    #[doc = "0x80 - ef_key_slot_4_w0"]
    pub ef_key_slot_4_w0: EF_KEY_SLOT_4_W0,
    #[doc = "0x84 - ef_key_slot_4_w1"]
    pub ef_key_slot_4_w1: EF_KEY_SLOT_4_W1,
    #[doc = "0x88 - ef_key_slot_4_w2"]
    pub ef_key_slot_4_w2: EF_KEY_SLOT_4_W2,
    #[doc = "0x8c - ef_key_slot_4_w3"]
    pub ef_key_slot_4_w3: EF_KEY_SLOT_4_W3,
    #[doc = "0x90 - ef_key_slot_5_w0"]
    pub ef_key_slot_5_w0: EF_KEY_SLOT_5_W0,
    #[doc = "0x94 - ef_key_slot_5_w1"]
    pub ef_key_slot_5_w1: EF_KEY_SLOT_5_W1,
    #[doc = "0x98 - ef_key_slot_5_w2"]
    pub ef_key_slot_5_w2: EF_KEY_SLOT_5_W2,
    #[doc = "0x9c - ef_key_slot_5_w3"]
    pub ef_key_slot_5_w3: EF_KEY_SLOT_5_W3,
    #[doc = "0xa0 - ef_key_slot_6_w0"]
    pub ef_key_slot_6_w0: EF_KEY_SLOT_6_W0,
    #[doc = "0xa4 - ef_key_slot_6_w1"]
    pub ef_key_slot_6_w1: EF_KEY_SLOT_6_W1,
    #[doc = "0xa8 - ef_key_slot_6_w2"]
    pub ef_key_slot_6_w2: EF_KEY_SLOT_6_W2,
    #[doc = "0xac - ef_key_slot_6_w3"]
    pub ef_key_slot_6_w3: EF_KEY_SLOT_6_W3,
    #[doc = "0xb0 - ef_key_slot_7_w0"]
    pub ef_key_slot_7_w0: EF_KEY_SLOT_7_W0,
    #[doc = "0xb4 - ef_key_slot_7_w1"]
    pub ef_key_slot_7_w1: EF_KEY_SLOT_7_W1,
    #[doc = "0xb8 - ef_key_slot_7_w2"]
    pub ef_key_slot_7_w2: EF_KEY_SLOT_7_W2,
    #[doc = "0xbc - ef_key_slot_7_w3"]
    pub ef_key_slot_7_w3: EF_KEY_SLOT_7_W3,
    #[doc = "0xc0 - ef_key_slot_8_w0"]
    pub ef_key_slot_8_w0: EF_KEY_SLOT_8_W0,
    #[doc = "0xc4 - ef_key_slot_8_w1"]
    pub ef_key_slot_8_w1: EF_KEY_SLOT_8_W1,
    #[doc = "0xc8 - ef_key_slot_8_w2"]
    pub ef_key_slot_8_w2: EF_KEY_SLOT_8_W2,
    #[doc = "0xcc - ef_key_slot_8_w3"]
    pub ef_key_slot_8_w3: EF_KEY_SLOT_8_W3,
    #[doc = "0xd0 - ef_key_slot_9_w0"]
    pub ef_key_slot_9_w0: EF_KEY_SLOT_9_W0,
    #[doc = "0xd4 - ef_key_slot_9_w1"]
    pub ef_key_slot_9_w1: EF_KEY_SLOT_9_W1,
    #[doc = "0xd8 - ef_key_slot_9_w2"]
    pub ef_key_slot_9_w2: EF_KEY_SLOT_9_W2,
    #[doc = "0xdc - ef_key_slot_9_w3"]
    pub ef_key_slot_9_w3: EF_KEY_SLOT_9_W3,
    #[doc = "0xe0 - ef_key_slot_10_w0"]
    pub ef_key_slot_10_w0: EF_KEY_SLOT_10_W0,
    #[doc = "0xe4 - ef_key_slot_10_w1"]
    pub ef_key_slot_10_w1: EF_KEY_SLOT_10_W1,
    #[doc = "0xe8 - ef_key_slot_10_w2"]
    pub ef_key_slot_10_w2: EF_KEY_SLOT_10_W2,
    #[doc = "0xec - ef_key_slot_10_w3"]
    pub ef_key_slot_10_w3: EF_KEY_SLOT_10_W3,
    #[doc = "0xf0 - ef_dat_1_rsvd_0"]
    pub ef_dat_1_rsvd_0: EF_DAT_1_RSVD_0,
    #[doc = "0xf4 - ef_dat_1_rsvd_1"]
    pub ef_dat_1_rsvd_1: EF_DAT_1_RSVD_1,
    #[doc = "0xf8 - ef_dat_1_rsvd_2"]
    pub ef_dat_1_rsvd_2: EF_DAT_1_RSVD_2,
    _reserved63: [u8; 0x0704],
    #[doc = "0x800 - ef_if_ctrl_0"]
    pub ef_if_ctrl_0: EF_IF_CTRL_0,
    #[doc = "0x804 - ef_if_cyc_0"]
    pub ef_if_cyc_0: EF_IF_CYC_0,
    #[doc = "0x808 - ef_if_cyc_1"]
    pub ef_if_cyc_1: EF_IF_CYC_1,
    #[doc = "0x80c - ef_if_0_manual"]
    pub ef_if_0_manual: EF_IF_0_MANUAL,
    #[doc = "0x810 - ef_if_0_status"]
    pub ef_if_0_status: EF_IF_0_STATUS,
    #[doc = "0x814 - ef_if_cfg_0"]
    pub ef_if_cfg_0: EF_IF_CFG_0,
    #[doc = "0x818 - ef_sw_cfg_0"]
    pub ef_sw_cfg_0: EF_SW_CFG_0,
    #[doc = "0x81c - ef_reserved"]
    pub ef_reserved: EF_RESERVED,
    #[doc = "0x820 - ef_if_sw_usage_0"]
    pub ef_if_sw_usage_0: EF_IF_SW_USAGE_0,
    #[doc = "0x824 - ef_if_sw_usage_1"]
    pub ef_if_sw_usage_1: EF_IF_SW_USAGE_1,
    _reserved73: [u8; 0xd8],
    #[doc = "0x900 - ef_if_ctrl_1"]
    pub ef_if_ctrl_1: EF_IF_CTRL_1,
    #[doc = "0x904 - ef_if_1_manual"]
    pub ef_if_1_manual: EF_IF_1_MANUAL,
    #[doc = "0x908 - ef_if_1_status"]
    pub ef_if_1_status: EF_IF_1_STATUS,
    _reserved76: [u8; 0x04],
    #[doc = "0x910 - ef_if_ctrl_2"]
    pub ef_if_ctrl_2: EF_IF_CTRL_2,
    #[doc = "0x914 - ef_if_2_manual"]
    pub ef_if_2_manual: EF_IF_2_MANUAL,
    #[doc = "0x918 - ef_if_2_status"]
    pub ef_if_2_status: EF_IF_2_STATUS,
    _reserved79: [u8; 0xe4],
    #[doc = "0xa00 - ef_crc_ctrl_0"]
    pub ef_crc_ctrl_0: EF_CRC_CTRL_0,
    #[doc = "0xa04 - ef_crc_ctrl_1"]
    pub ef_crc_ctrl_1: EF_CRC_CTRL_1,
    #[doc = "0xa08 - ef_crc_ctrl_2"]
    pub ef_crc_ctrl_2: EF_CRC_CTRL_2,
    #[doc = "0xa0c - ef_crc_ctrl_3"]
    pub ef_crc_ctrl_3: EF_CRC_CTRL_3,
    #[doc = "0xa10 - ef_crc_ctrl_4"]
    pub ef_crc_ctrl_4: EF_CRC_CTRL_4,
    #[doc = "0xa14 - ef_crc_ctrl_5"]
    pub ef_crc_ctrl_5: EF_CRC_CTRL_5,
}
#[doc = "ef_if_ctrl_0 (rw) register accessor: an alias for `Reg<EF_IF_CTRL_0_SPEC>`"]
pub type EF_IF_CTRL_0 = crate::Reg<ef_if_ctrl_0::EF_IF_CTRL_0_SPEC>;
#[doc = "ef_if_ctrl_0"]
pub mod ef_if_ctrl_0;
#[doc = "ef_if_cyc_0 (rw) register accessor: an alias for `Reg<EF_IF_CYC_0_SPEC>`"]
pub type EF_IF_CYC_0 = crate::Reg<ef_if_cyc_0::EF_IF_CYC_0_SPEC>;
#[doc = "ef_if_cyc_0"]
pub mod ef_if_cyc_0;
#[doc = "ef_if_cyc_1 (rw) register accessor: an alias for `Reg<EF_IF_CYC_1_SPEC>`"]
pub type EF_IF_CYC_1 = crate::Reg<ef_if_cyc_1::EF_IF_CYC_1_SPEC>;
#[doc = "ef_if_cyc_1"]
pub mod ef_if_cyc_1;
#[doc = "ef_if_0_manual (rw) register accessor: an alias for `Reg<EF_IF_0_MANUAL_SPEC>`"]
pub type EF_IF_0_MANUAL = crate::Reg<ef_if_0_manual::EF_IF_0_MANUAL_SPEC>;
#[doc = "ef_if_0_manual"]
pub mod ef_if_0_manual;
#[doc = "ef_if_0_status (rw) register accessor: an alias for `Reg<EF_IF_0_STATUS_SPEC>`"]
pub type EF_IF_0_STATUS = crate::Reg<ef_if_0_status::EF_IF_0_STATUS_SPEC>;
#[doc = "ef_if_0_status"]
pub mod ef_if_0_status;
#[doc = "ef_if_cfg_0 (rw) register accessor: an alias for `Reg<EF_IF_CFG_0_SPEC>`"]
pub type EF_IF_CFG_0 = crate::Reg<ef_if_cfg_0::EF_IF_CFG_0_SPEC>;
#[doc = "ef_if_cfg_0"]
pub mod ef_if_cfg_0;
#[doc = "ef_sw_cfg_0 (rw) register accessor: an alias for `Reg<EF_SW_CFG_0_SPEC>`"]
pub type EF_SW_CFG_0 = crate::Reg<ef_sw_cfg_0::EF_SW_CFG_0_SPEC>;
#[doc = "ef_sw_cfg_0"]
pub mod ef_sw_cfg_0;
#[doc = "ef_reserved (rw) register accessor: an alias for `Reg<EF_RESERVED_SPEC>`"]
pub type EF_RESERVED = crate::Reg<ef_reserved::EF_RESERVED_SPEC>;
#[doc = "ef_reserved"]
pub mod ef_reserved;
#[doc = "ef_if_sw_usage_0 (rw) register accessor: an alias for `Reg<EF_IF_SW_USAGE_0_SPEC>`"]
pub type EF_IF_SW_USAGE_0 = crate::Reg<ef_if_sw_usage_0::EF_IF_SW_USAGE_0_SPEC>;
#[doc = "ef_if_sw_usage_0"]
pub mod ef_if_sw_usage_0;
#[doc = "ef_if_sw_usage_1 (rw) register accessor: an alias for `Reg<EF_IF_SW_USAGE_1_SPEC>`"]
pub type EF_IF_SW_USAGE_1 = crate::Reg<ef_if_sw_usage_1::EF_IF_SW_USAGE_1_SPEC>;
#[doc = "ef_if_sw_usage_1"]
pub mod ef_if_sw_usage_1;
#[doc = "ef_if_ctrl_1 (rw) register accessor: an alias for `Reg<EF_IF_CTRL_1_SPEC>`"]
pub type EF_IF_CTRL_1 = crate::Reg<ef_if_ctrl_1::EF_IF_CTRL_1_SPEC>;
#[doc = "ef_if_ctrl_1"]
pub mod ef_if_ctrl_1;
#[doc = "ef_if_1_manual (rw) register accessor: an alias for `Reg<EF_IF_1_MANUAL_SPEC>`"]
pub type EF_IF_1_MANUAL = crate::Reg<ef_if_1_manual::EF_IF_1_MANUAL_SPEC>;
#[doc = "ef_if_1_manual"]
pub mod ef_if_1_manual;
#[doc = "ef_if_1_status (rw) register accessor: an alias for `Reg<EF_IF_1_STATUS_SPEC>`"]
pub type EF_IF_1_STATUS = crate::Reg<ef_if_1_status::EF_IF_1_STATUS_SPEC>;
#[doc = "ef_if_1_status"]
pub mod ef_if_1_status;
#[doc = "ef_if_ctrl_2 (rw) register accessor: an alias for `Reg<EF_IF_CTRL_2_SPEC>`"]
pub type EF_IF_CTRL_2 = crate::Reg<ef_if_ctrl_2::EF_IF_CTRL_2_SPEC>;
#[doc = "ef_if_ctrl_2"]
pub mod ef_if_ctrl_2;
#[doc = "ef_if_2_manual (rw) register accessor: an alias for `Reg<EF_IF_2_MANUAL_SPEC>`"]
pub type EF_IF_2_MANUAL = crate::Reg<ef_if_2_manual::EF_IF_2_MANUAL_SPEC>;
#[doc = "ef_if_2_manual"]
pub mod ef_if_2_manual;
#[doc = "ef_if_2_status (rw) register accessor: an alias for `Reg<EF_IF_2_STATUS_SPEC>`"]
pub type EF_IF_2_STATUS = crate::Reg<ef_if_2_status::EF_IF_2_STATUS_SPEC>;
#[doc = "ef_if_2_status"]
pub mod ef_if_2_status;
#[doc = "ef_crc_ctrl_0 (rw) register accessor: an alias for `Reg<EF_CRC_CTRL_0_SPEC>`"]
pub type EF_CRC_CTRL_0 = crate::Reg<ef_crc_ctrl_0::EF_CRC_CTRL_0_SPEC>;
#[doc = "ef_crc_ctrl_0"]
pub mod ef_crc_ctrl_0;
#[doc = "ef_crc_ctrl_1 (rw) register accessor: an alias for `Reg<EF_CRC_CTRL_1_SPEC>`"]
pub type EF_CRC_CTRL_1 = crate::Reg<ef_crc_ctrl_1::EF_CRC_CTRL_1_SPEC>;
#[doc = "ef_crc_ctrl_1"]
pub mod ef_crc_ctrl_1;
#[doc = "ef_crc_ctrl_2 (rw) register accessor: an alias for `Reg<EF_CRC_CTRL_2_SPEC>`"]
pub type EF_CRC_CTRL_2 = crate::Reg<ef_crc_ctrl_2::EF_CRC_CTRL_2_SPEC>;
#[doc = "ef_crc_ctrl_2"]
pub mod ef_crc_ctrl_2;
#[doc = "ef_crc_ctrl_3 (rw) register accessor: an alias for `Reg<EF_CRC_CTRL_3_SPEC>`"]
pub type EF_CRC_CTRL_3 = crate::Reg<ef_crc_ctrl_3::EF_CRC_CTRL_3_SPEC>;
#[doc = "ef_crc_ctrl_3"]
pub mod ef_crc_ctrl_3;
#[doc = "ef_crc_ctrl_4 (rw) register accessor: an alias for `Reg<EF_CRC_CTRL_4_SPEC>`"]
pub type EF_CRC_CTRL_4 = crate::Reg<ef_crc_ctrl_4::EF_CRC_CTRL_4_SPEC>;
#[doc = "ef_crc_ctrl_4"]
pub mod ef_crc_ctrl_4;
#[doc = "ef_crc_ctrl_5 (rw) register accessor: an alias for `Reg<EF_CRC_CTRL_5_SPEC>`"]
pub type EF_CRC_CTRL_5 = crate::Reg<ef_crc_ctrl_5::EF_CRC_CTRL_5_SPEC>;
#[doc = "ef_crc_ctrl_5"]
pub mod ef_crc_ctrl_5;
#[doc = "ef_cfg_0 (rw) register accessor: an alias for `Reg<EF_CFG_0_SPEC>`"]
pub type EF_CFG_0 = crate::Reg<ef_cfg_0::EF_CFG_0_SPEC>;
#[doc = "ef_cfg_0"]
pub mod ef_cfg_0;
#[doc = "ef_dbg_pwd_low (rw) register accessor: an alias for `Reg<EF_DBG_PWD_LOW_SPEC>`"]
pub type EF_DBG_PWD_LOW = crate::Reg<ef_dbg_pwd_low::EF_DBG_PWD_LOW_SPEC>;
#[doc = "ef_dbg_pwd_low"]
pub mod ef_dbg_pwd_low;
#[doc = "ef_dbg_pwd_high (rw) register accessor: an alias for `Reg<EF_DBG_PWD_HIGH_SPEC>`"]
pub type EF_DBG_PWD_HIGH = crate::Reg<ef_dbg_pwd_high::EF_DBG_PWD_HIGH_SPEC>;
#[doc = "ef_dbg_pwd_high"]
pub mod ef_dbg_pwd_high;
#[doc = "ef_dbg_pwd2_low (rw) register accessor: an alias for `Reg<EF_DBG_PWD2_LOW_SPEC>`"]
pub type EF_DBG_PWD2_LOW = crate::Reg<ef_dbg_pwd2_low::EF_DBG_PWD2_LOW_SPEC>;
#[doc = "ef_dbg_pwd2_low"]
pub mod ef_dbg_pwd2_low;
#[doc = "ef_dbg_pwd2_high (rw) register accessor: an alias for `Reg<EF_DBG_PWD2_HIGH_SPEC>`"]
pub type EF_DBG_PWD2_HIGH = crate::Reg<ef_dbg_pwd2_high::EF_DBG_PWD2_HIGH_SPEC>;
#[doc = "ef_dbg_pwd2_high"]
pub mod ef_dbg_pwd2_high;
#[doc = "ef_wifi_mac_low (rw) register accessor: an alias for `Reg<EF_WIFI_MAC_LOW_SPEC>`"]
pub type EF_WIFI_MAC_LOW = crate::Reg<ef_wifi_mac_low::EF_WIFI_MAC_LOW_SPEC>;
#[doc = "ef_wifi_mac_low"]
pub mod ef_wifi_mac_low;
#[doc = "ef_wifi_mac_high (rw) register accessor: an alias for `Reg<EF_WIFI_MAC_HIGH_SPEC>`"]
pub type EF_WIFI_MAC_HIGH = crate::Reg<ef_wifi_mac_high::EF_WIFI_MAC_HIGH_SPEC>;
#[doc = "ef_wifi_mac_high"]
pub mod ef_wifi_mac_high;
#[doc = "ef_key_slot_0_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W0_SPEC>`"]
pub type EF_KEY_SLOT_0_W0 = crate::Reg<ef_key_slot_0_w0::EF_KEY_SLOT_0_W0_SPEC>;
#[doc = "ef_key_slot_0_w0"]
pub mod ef_key_slot_0_w0;
#[doc = "ef_key_slot_0_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W1_SPEC>`"]
pub type EF_KEY_SLOT_0_W1 = crate::Reg<ef_key_slot_0_w1::EF_KEY_SLOT_0_W1_SPEC>;
#[doc = "ef_key_slot_0_w1"]
pub mod ef_key_slot_0_w1;
#[doc = "ef_key_slot_0_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W2_SPEC>`"]
pub type EF_KEY_SLOT_0_W2 = crate::Reg<ef_key_slot_0_w2::EF_KEY_SLOT_0_W2_SPEC>;
#[doc = "ef_key_slot_0_w2"]
pub mod ef_key_slot_0_w2;
#[doc = "ef_key_slot_0_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W3_SPEC>`"]
pub type EF_KEY_SLOT_0_W3 = crate::Reg<ef_key_slot_0_w3::EF_KEY_SLOT_0_W3_SPEC>;
#[doc = "ef_key_slot_0_w3"]
pub mod ef_key_slot_0_w3;
#[doc = "ef_key_slot_1_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W0_SPEC>`"]
pub type EF_KEY_SLOT_1_W0 = crate::Reg<ef_key_slot_1_w0::EF_KEY_SLOT_1_W0_SPEC>;
#[doc = "ef_key_slot_1_w0"]
pub mod ef_key_slot_1_w0;
#[doc = "ef_key_slot_1_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W1_SPEC>`"]
pub type EF_KEY_SLOT_1_W1 = crate::Reg<ef_key_slot_1_w1::EF_KEY_SLOT_1_W1_SPEC>;
#[doc = "ef_key_slot_1_w1"]
pub mod ef_key_slot_1_w1;
#[doc = "ef_key_slot_1_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W2_SPEC>`"]
pub type EF_KEY_SLOT_1_W2 = crate::Reg<ef_key_slot_1_w2::EF_KEY_SLOT_1_W2_SPEC>;
#[doc = "ef_key_slot_1_w2"]
pub mod ef_key_slot_1_w2;
#[doc = "ef_key_slot_1_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W3_SPEC>`"]
pub type EF_KEY_SLOT_1_W3 = crate::Reg<ef_key_slot_1_w3::EF_KEY_SLOT_1_W3_SPEC>;
#[doc = "ef_key_slot_1_w3"]
pub mod ef_key_slot_1_w3;
#[doc = "ef_key_slot_2_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W0_SPEC>`"]
pub type EF_KEY_SLOT_2_W0 = crate::Reg<ef_key_slot_2_w0::EF_KEY_SLOT_2_W0_SPEC>;
#[doc = "ef_key_slot_2_w0"]
pub mod ef_key_slot_2_w0;
#[doc = "ef_key_slot_2_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W1_SPEC>`"]
pub type EF_KEY_SLOT_2_W1 = crate::Reg<ef_key_slot_2_w1::EF_KEY_SLOT_2_W1_SPEC>;
#[doc = "ef_key_slot_2_w1"]
pub mod ef_key_slot_2_w1;
#[doc = "ef_key_slot_2_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W2_SPEC>`"]
pub type EF_KEY_SLOT_2_W2 = crate::Reg<ef_key_slot_2_w2::EF_KEY_SLOT_2_W2_SPEC>;
#[doc = "ef_key_slot_2_w2"]
pub mod ef_key_slot_2_w2;
#[doc = "ef_key_slot_2_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W3_SPEC>`"]
pub type EF_KEY_SLOT_2_W3 = crate::Reg<ef_key_slot_2_w3::EF_KEY_SLOT_2_W3_SPEC>;
#[doc = "ef_key_slot_2_w3"]
pub mod ef_key_slot_2_w3;
#[doc = "ef_key_slot_3_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W0_SPEC>`"]
pub type EF_KEY_SLOT_3_W0 = crate::Reg<ef_key_slot_3_w0::EF_KEY_SLOT_3_W0_SPEC>;
#[doc = "ef_key_slot_3_w0"]
pub mod ef_key_slot_3_w0;
#[doc = "ef_key_slot_3_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W1_SPEC>`"]
pub type EF_KEY_SLOT_3_W1 = crate::Reg<ef_key_slot_3_w1::EF_KEY_SLOT_3_W1_SPEC>;
#[doc = "ef_key_slot_3_w1"]
pub mod ef_key_slot_3_w1;
#[doc = "ef_key_slot_3_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W2_SPEC>`"]
pub type EF_KEY_SLOT_3_W2 = crate::Reg<ef_key_slot_3_w2::EF_KEY_SLOT_3_W2_SPEC>;
#[doc = "ef_key_slot_3_w2"]
pub mod ef_key_slot_3_w2;
#[doc = "ef_key_slot_3_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W3_SPEC>`"]
pub type EF_KEY_SLOT_3_W3 = crate::Reg<ef_key_slot_3_w3::EF_KEY_SLOT_3_W3_SPEC>;
#[doc = "ef_key_slot_3_w3"]
pub mod ef_key_slot_3_w3;
#[doc = "ef_sw_usage_0 (rw) register accessor: an alias for `Reg<EF_SW_USAGE_0_SPEC>`"]
pub type EF_SW_USAGE_0 = crate::Reg<ef_sw_usage_0::EF_SW_USAGE_0_SPEC>;
#[doc = "ef_sw_usage_0"]
pub mod ef_sw_usage_0;
#[doc = "ef_sw_usage_1 (rw) register accessor: an alias for `Reg<EF_SW_USAGE_1_SPEC>`"]
pub type EF_SW_USAGE_1 = crate::Reg<ef_sw_usage_1::EF_SW_USAGE_1_SPEC>;
#[doc = "ef_sw_usage_1"]
pub mod ef_sw_usage_1;
#[doc = "ef_sw_usage_2 (rw) register accessor: an alias for `Reg<EF_SW_USAGE_2_SPEC>`"]
pub type EF_SW_USAGE_2 = crate::Reg<ef_sw_usage_2::EF_SW_USAGE_2_SPEC>;
#[doc = "ef_sw_usage_2"]
pub mod ef_sw_usage_2;
#[doc = "ef_sw_usage_3 (rw) register accessor: an alias for `Reg<EF_SW_USAGE_3_SPEC>`"]
pub type EF_SW_USAGE_3 = crate::Reg<ef_sw_usage_3::EF_SW_USAGE_3_SPEC>;
#[doc = "ef_sw_usage_3"]
pub mod ef_sw_usage_3;
#[doc = "ef_key_slot_11_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_11_W0_SPEC>`"]
pub type EF_KEY_SLOT_11_W0 = crate::Reg<ef_key_slot_11_w0::EF_KEY_SLOT_11_W0_SPEC>;
#[doc = "ef_key_slot_11_w0"]
pub mod ef_key_slot_11_w0;
#[doc = "ef_key_slot_11_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_11_W1_SPEC>`"]
pub type EF_KEY_SLOT_11_W1 = crate::Reg<ef_key_slot_11_w1::EF_KEY_SLOT_11_W1_SPEC>;
#[doc = "ef_key_slot_11_w1"]
pub mod ef_key_slot_11_w1;
#[doc = "ef_key_slot_11_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_11_W2_SPEC>`"]
pub type EF_KEY_SLOT_11_W2 = crate::Reg<ef_key_slot_11_w2::EF_KEY_SLOT_11_W2_SPEC>;
#[doc = "ef_key_slot_11_w2"]
pub mod ef_key_slot_11_w2;
#[doc = "ef_key_slot_11_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_11_W3_SPEC>`"]
pub type EF_KEY_SLOT_11_W3 = crate::Reg<ef_key_slot_11_w3::EF_KEY_SLOT_11_W3_SPEC>;
#[doc = "ef_key_slot_11_w3"]
pub mod ef_key_slot_11_w3;
#[doc = "ef_data_0_lock (rw) register accessor: an alias for `Reg<EF_DATA_0_LOCK_SPEC>`"]
pub type EF_DATA_0_LOCK = crate::Reg<ef_data_0_lock::EF_DATA_0_LOCK_SPEC>;
#[doc = "ef_data_0_lock"]
pub mod ef_data_0_lock;
#[doc = "ef_key_slot_4_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W0_SPEC>`"]
pub type EF_KEY_SLOT_4_W0 = crate::Reg<ef_key_slot_4_w0::EF_KEY_SLOT_4_W0_SPEC>;
#[doc = "ef_key_slot_4_w0"]
pub mod ef_key_slot_4_w0;
#[doc = "ef_key_slot_4_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W1_SPEC>`"]
pub type EF_KEY_SLOT_4_W1 = crate::Reg<ef_key_slot_4_w1::EF_KEY_SLOT_4_W1_SPEC>;
#[doc = "ef_key_slot_4_w1"]
pub mod ef_key_slot_4_w1;
#[doc = "ef_key_slot_4_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W2_SPEC>`"]
pub type EF_KEY_SLOT_4_W2 = crate::Reg<ef_key_slot_4_w2::EF_KEY_SLOT_4_W2_SPEC>;
#[doc = "ef_key_slot_4_w2"]
pub mod ef_key_slot_4_w2;
#[doc = "ef_key_slot_4_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W3_SPEC>`"]
pub type EF_KEY_SLOT_4_W3 = crate::Reg<ef_key_slot_4_w3::EF_KEY_SLOT_4_W3_SPEC>;
#[doc = "ef_key_slot_4_w3"]
pub mod ef_key_slot_4_w3;
#[doc = "ef_key_slot_5_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W0_SPEC>`"]
pub type EF_KEY_SLOT_5_W0 = crate::Reg<ef_key_slot_5_w0::EF_KEY_SLOT_5_W0_SPEC>;
#[doc = "ef_key_slot_5_w0"]
pub mod ef_key_slot_5_w0;
#[doc = "ef_key_slot_5_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W1_SPEC>`"]
pub type EF_KEY_SLOT_5_W1 = crate::Reg<ef_key_slot_5_w1::EF_KEY_SLOT_5_W1_SPEC>;
#[doc = "ef_key_slot_5_w1"]
pub mod ef_key_slot_5_w1;
#[doc = "ef_key_slot_5_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W2_SPEC>`"]
pub type EF_KEY_SLOT_5_W2 = crate::Reg<ef_key_slot_5_w2::EF_KEY_SLOT_5_W2_SPEC>;
#[doc = "ef_key_slot_5_w2"]
pub mod ef_key_slot_5_w2;
#[doc = "ef_key_slot_5_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W3_SPEC>`"]
pub type EF_KEY_SLOT_5_W3 = crate::Reg<ef_key_slot_5_w3::EF_KEY_SLOT_5_W3_SPEC>;
#[doc = "ef_key_slot_5_w3"]
pub mod ef_key_slot_5_w3;
#[doc = "ef_key_slot_6_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_6_W0_SPEC>`"]
pub type EF_KEY_SLOT_6_W0 = crate::Reg<ef_key_slot_6_w0::EF_KEY_SLOT_6_W0_SPEC>;
#[doc = "ef_key_slot_6_w0"]
pub mod ef_key_slot_6_w0;
#[doc = "ef_key_slot_6_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_6_W1_SPEC>`"]
pub type EF_KEY_SLOT_6_W1 = crate::Reg<ef_key_slot_6_w1::EF_KEY_SLOT_6_W1_SPEC>;
#[doc = "ef_key_slot_6_w1"]
pub mod ef_key_slot_6_w1;
#[doc = "ef_key_slot_6_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_6_W2_SPEC>`"]
pub type EF_KEY_SLOT_6_W2 = crate::Reg<ef_key_slot_6_w2::EF_KEY_SLOT_6_W2_SPEC>;
#[doc = "ef_key_slot_6_w2"]
pub mod ef_key_slot_6_w2;
#[doc = "ef_key_slot_6_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_6_W3_SPEC>`"]
pub type EF_KEY_SLOT_6_W3 = crate::Reg<ef_key_slot_6_w3::EF_KEY_SLOT_6_W3_SPEC>;
#[doc = "ef_key_slot_6_w3"]
pub mod ef_key_slot_6_w3;
#[doc = "ef_key_slot_7_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_7_W0_SPEC>`"]
pub type EF_KEY_SLOT_7_W0 = crate::Reg<ef_key_slot_7_w0::EF_KEY_SLOT_7_W0_SPEC>;
#[doc = "ef_key_slot_7_w0"]
pub mod ef_key_slot_7_w0;
#[doc = "ef_key_slot_7_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_7_W1_SPEC>`"]
pub type EF_KEY_SLOT_7_W1 = crate::Reg<ef_key_slot_7_w1::EF_KEY_SLOT_7_W1_SPEC>;
#[doc = "ef_key_slot_7_w1"]
pub mod ef_key_slot_7_w1;
#[doc = "ef_key_slot_7_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_7_W2_SPEC>`"]
pub type EF_KEY_SLOT_7_W2 = crate::Reg<ef_key_slot_7_w2::EF_KEY_SLOT_7_W2_SPEC>;
#[doc = "ef_key_slot_7_w2"]
pub mod ef_key_slot_7_w2;
#[doc = "ef_key_slot_7_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_7_W3_SPEC>`"]
pub type EF_KEY_SLOT_7_W3 = crate::Reg<ef_key_slot_7_w3::EF_KEY_SLOT_7_W3_SPEC>;
#[doc = "ef_key_slot_7_w3"]
pub mod ef_key_slot_7_w3;
#[doc = "ef_key_slot_8_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_8_W0_SPEC>`"]
pub type EF_KEY_SLOT_8_W0 = crate::Reg<ef_key_slot_8_w0::EF_KEY_SLOT_8_W0_SPEC>;
#[doc = "ef_key_slot_8_w0"]
pub mod ef_key_slot_8_w0;
#[doc = "ef_key_slot_8_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_8_W1_SPEC>`"]
pub type EF_KEY_SLOT_8_W1 = crate::Reg<ef_key_slot_8_w1::EF_KEY_SLOT_8_W1_SPEC>;
#[doc = "ef_key_slot_8_w1"]
pub mod ef_key_slot_8_w1;
#[doc = "ef_key_slot_8_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_8_W2_SPEC>`"]
pub type EF_KEY_SLOT_8_W2 = crate::Reg<ef_key_slot_8_w2::EF_KEY_SLOT_8_W2_SPEC>;
#[doc = "ef_key_slot_8_w2"]
pub mod ef_key_slot_8_w2;
#[doc = "ef_key_slot_8_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_8_W3_SPEC>`"]
pub type EF_KEY_SLOT_8_W3 = crate::Reg<ef_key_slot_8_w3::EF_KEY_SLOT_8_W3_SPEC>;
#[doc = "ef_key_slot_8_w3"]
pub mod ef_key_slot_8_w3;
#[doc = "ef_key_slot_9_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_9_W0_SPEC>`"]
pub type EF_KEY_SLOT_9_W0 = crate::Reg<ef_key_slot_9_w0::EF_KEY_SLOT_9_W0_SPEC>;
#[doc = "ef_key_slot_9_w0"]
pub mod ef_key_slot_9_w0;
#[doc = "ef_key_slot_9_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_9_W1_SPEC>`"]
pub type EF_KEY_SLOT_9_W1 = crate::Reg<ef_key_slot_9_w1::EF_KEY_SLOT_9_W1_SPEC>;
#[doc = "ef_key_slot_9_w1"]
pub mod ef_key_slot_9_w1;
#[doc = "ef_key_slot_9_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_9_W2_SPEC>`"]
pub type EF_KEY_SLOT_9_W2 = crate::Reg<ef_key_slot_9_w2::EF_KEY_SLOT_9_W2_SPEC>;
#[doc = "ef_key_slot_9_w2"]
pub mod ef_key_slot_9_w2;
#[doc = "ef_key_slot_9_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_9_W3_SPEC>`"]
pub type EF_KEY_SLOT_9_W3 = crate::Reg<ef_key_slot_9_w3::EF_KEY_SLOT_9_W3_SPEC>;
#[doc = "ef_key_slot_9_w3"]
pub mod ef_key_slot_9_w3;
#[doc = "ef_key_slot_10_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_10_W0_SPEC>`"]
pub type EF_KEY_SLOT_10_W0 = crate::Reg<ef_key_slot_10_w0::EF_KEY_SLOT_10_W0_SPEC>;
#[doc = "ef_key_slot_10_w0"]
pub mod ef_key_slot_10_w0;
#[doc = "ef_key_slot_10_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_10_W1_SPEC>`"]
pub type EF_KEY_SLOT_10_W1 = crate::Reg<ef_key_slot_10_w1::EF_KEY_SLOT_10_W1_SPEC>;
#[doc = "ef_key_slot_10_w1"]
pub mod ef_key_slot_10_w1;
#[doc = "ef_key_slot_10_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_10_W2_SPEC>`"]
pub type EF_KEY_SLOT_10_W2 = crate::Reg<ef_key_slot_10_w2::EF_KEY_SLOT_10_W2_SPEC>;
#[doc = "ef_key_slot_10_w2"]
pub mod ef_key_slot_10_w2;
#[doc = "ef_key_slot_10_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_10_W3_SPEC>`"]
pub type EF_KEY_SLOT_10_W3 = crate::Reg<ef_key_slot_10_w3::EF_KEY_SLOT_10_W3_SPEC>;
#[doc = "ef_key_slot_10_w3"]
pub mod ef_key_slot_10_w3;
#[doc = "ef_dat_1_rsvd_0 (rw) register accessor: an alias for `Reg<EF_DAT_1_RSVD_0_SPEC>`"]
pub type EF_DAT_1_RSVD_0 = crate::Reg<ef_dat_1_rsvd_0::EF_DAT_1_RSVD_0_SPEC>;
#[doc = "ef_dat_1_rsvd_0"]
pub mod ef_dat_1_rsvd_0;
#[doc = "ef_dat_1_rsvd_1 (rw) register accessor: an alias for `Reg<EF_DAT_1_RSVD_1_SPEC>`"]
pub type EF_DAT_1_RSVD_1 = crate::Reg<ef_dat_1_rsvd_1::EF_DAT_1_RSVD_1_SPEC>;
#[doc = "ef_dat_1_rsvd_1"]
pub mod ef_dat_1_rsvd_1;
#[doc = "ef_dat_1_rsvd_2 (rw) register accessor: an alias for `Reg<EF_DAT_1_RSVD_2_SPEC>`"]
pub type EF_DAT_1_RSVD_2 = crate::Reg<ef_dat_1_rsvd_2::EF_DAT_1_RSVD_2_SPEC>;
#[doc = "ef_dat_1_rsvd_2"]
pub mod ef_dat_1_rsvd_2;
