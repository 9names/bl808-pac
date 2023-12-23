#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Address Low Register"]
    pub sd_sys_addr_low: SD_SYS_ADDR_LOW,
    #[doc = "0x02 - System Address High Register"]
    pub sd_sys_addr_high: SD_SYS_ADDR_HIGH,
    #[doc = "0x04 - Block Size Register"]
    pub sd_block_size: SD_BLOCK_SIZE,
    #[doc = "0x06 - Block Count Register"]
    pub sd_block_count: SD_BLOCK_COUNT,
    #[doc = "0x08 - Argument Low Register"]
    pub sd_arg_low: SD_ARG_LOW,
    #[doc = "0x0a - Argument High Register"]
    pub sd_arg_high: SD_ARG_HIGH,
    #[doc = "0x0c - Transfer Mode Register"]
    pub sd_transfer_mode: SD_TRANSFER_MODE,
    #[doc = "0x0e - Command Register"]
    pub sd_cmd: SD_CMD,
    #[doc = "0x10 - Response Register 0"]
    pub sd_resp_0: SD_RESP_0,
    #[doc = "0x12 - Response Register 1"]
    pub sd_resp_1: SD_RESP_1,
    #[doc = "0x14 - Response Register 2"]
    pub sd_resp_2: SD_RESP_2,
    #[doc = "0x16 - Response Register 3"]
    pub sd_resp_3: SD_RESP_3,
    #[doc = "0x18 - Response Register 4"]
    pub sd_resp_4: SD_RESP_4,
    #[doc = "0x1a - Response Register 5"]
    pub sd_resp_5: SD_RESP_5,
    #[doc = "0x1c - Response Register 6"]
    pub sd_resp_6: SD_RESP_6,
    #[doc = "0x1e - Response Register 7"]
    pub sd_resp_7: SD_RESP_7,
    #[doc = "0x20 - Buffer Data Port 0 Register"]
    pub sd_buffer_data_port_0: SD_BUFFER_DATA_PORT_0,
    #[doc = "0x22 - Buffer Data Port 1 Register"]
    pub sd_buffer_data_port_1: SD_BUFFER_DATA_PORT_1,
    #[doc = "0x24 - Present State Register 1"]
    pub sd_present_state_1: SD_PRESENT_STATE_1,
    #[doc = "0x26 - Present State Register 2"]
    pub sd_present_state_2: SD_PRESENT_STATE_2,
    #[doc = "0x28 - Host Control Register"]
    pub sd_host_ctrl: SD_HOST_CTRL,
    #[doc = "0x2a - Block Gap Control Register"]
    pub sd_block_gap_ctrl: SD_BLOCK_GAP_CTRL,
    #[doc = "0x2c - Clock Control Register"]
    pub sd_clock_ctrl: SD_CLOCK_CTRL,
    #[doc = "0x2e - Timeout Control/Software Reset Register"]
    pub sd_timeout_ctrl_sw_reset: SD_TIMEOUT_CTRL_SW_RESET,
    #[doc = "0x30 - Normal Interrupt Status Register"]
    pub sd_normal_int_status: SD_NORMAL_INT_STATUS,
    #[doc = "0x32 - Error Interrupt Status Register"]
    pub sd_error_int_status: SD_ERROR_INT_STATUS,
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    pub sd_normal_int_status_en: SD_NORMAL_INT_STATUS_EN,
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    pub sd_error_int_status_en: SD_ERROR_INT_STATUS_EN,
    #[doc = "0x38 - Normal Interrupt Status Interrupt Enable Register"]
    pub sd_normal_int_status_int_en: SD_NORMAL_INT_STATUS_INT_EN,
    #[doc = "0x3a - Error Interrupt Status Interrupt Enable Register"]
    pub sd_error_int_status_int_en: SD_ERROR_INT_STATUS_INT_EN,
    #[doc = "0x3c - Auto CMD12 Error Status Register"]
    pub sd_auto_cmd12_error_status: SD_AUTO_CMD12_ERROR_STATUS,
    #[doc = "0x3e - Host Control 2 Register"]
    pub host_ctrl_2: HOST_CTRL_2,
    #[doc = "0x40 - Capabilities Register 1"]
    pub sd_capabilities_1: SD_CAPABILITIES_1,
    #[doc = "0x42 - Capabilities Register 2"]
    pub sd_capabilities_2: SD_CAPABILITIES_2,
    #[doc = "0x44 - Capabilities Register 3"]
    pub sd_capabilities_3: SD_CAPABILITIES_3,
    #[doc = "0x46 - Capabilities Register 4"]
    pub sd_capabilities_4: SD_CAPABILITIES_4,
    #[doc = "0x48 - Maximum Current Register 1"]
    pub sd_max_current_1: SD_MAX_CURRENT_1,
    #[doc = "0x4a - Maximum Current Register 2"]
    pub sd_max_current_2: SD_MAX_CURRENT_2,
    #[doc = "0x4c - Maximum Current Register 3"]
    pub sd_max_current_3: SD_MAX_CURRENT_3,
    #[doc = "0x4e - Maximum Current Register 4"]
    pub sd_max_current_4: SD_MAX_CURRENT_4,
    #[doc = "0x50 - Force Event Auto cmd12 Error Register"]
    pub sd_force_event_auto_cmd12_error: SD_FORCE_EVENT_AUTO_CMD12_ERROR,
    #[doc = "0x52 - Force Event for Error Status Register"]
    pub sd_force_event_for_error_status: SD_FORCE_EVENT_FOR_ERROR_STATUS,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub sd_adma_error_status: SD_ADMA_ERROR_STATUS,
    _reserved43: [u8; 0x02],
    #[doc = "0x58 - ADMA System Address Register 1"]
    pub sd_adma_sys_addr_1: SD_ADMA_SYS_ADDR_1,
    #[doc = "0x5a - ADMA System Address Register 2"]
    pub sd_adma_sys_addr_2: SD_ADMA_SYS_ADDR_2,
    #[doc = "0x5c - ADMA System Address Register 3"]
    pub sd_adma_sys_addr_3: SD_ADMA_SYS_ADDR_3,
    #[doc = "0x5e - ADMA System Address Register 4"]
    pub sd_adma_sys_addr_4: SD_ADMA_SYS_ADDR_4,
    #[doc = "0x60 - Preset Value Register for Initialization"]
    pub preset_value_for_init: PRESET_VALUE_FOR_INIT,
    #[doc = "0x62 - Preset Value Register for Default Speed"]
    pub preset_value_for_ds: PRESET_VALUE_FOR_DS,
    #[doc = "0x64 - Preset Value Register for High Speed"]
    pub preset_value_for_hs: PRESET_VALUE_FOR_HS,
    #[doc = "0x66 - Preset Value Register for SDR12"]
    pub preset_value_for_sdr12: PRESET_VALUE_FOR_SDR12,
    #[doc = "0x68 - Preset Value Register for SDR25"]
    pub preset_value_for_sdr25: PRESET_VALUE_FOR_SDR25,
    #[doc = "0x6a - Preset Value Register for SDR50"]
    pub preset_value_for_sdr50: PRESET_VALUE_FOR_SDR50,
    #[doc = "0x6c - Preset Value Register for SDR104"]
    pub preset_value_for_sdr104: PRESET_VALUE_FOR_SDR104,
    #[doc = "0x6e - Preset Value Register for DDR50"]
    pub preset_value_for_ddr50: PRESET_VALUE_FOR_DDR50,
    _reserved55: [u8; 0x70],
    #[doc = "0xe0 - Shared Bus Control Register"]
    pub shared_bus_ctrl: SHARED_BUS_CTRL,
    _reserved56: [u8; 0x18],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub sd_slot_int_status: SD_SLOT_INT_STATUS,
    #[doc = "0xfe - Host Control Version Register"]
    pub sd_host_ctrl_ver: SD_HOST_CTRL_VER,
    #[doc = "0x100 - SD Extra Parameters Register"]
    pub sd_cfg_fifo_param: SD_CFG_FIFO_PARAM,
    #[doc = "0x104 - FIFO Parameters Register"]
    pub sd_fifo_param: SD_FIFO_PARAM,
    #[doc = "0x108 - SPI Mode Register"]
    pub sd_spi_mode: SD_SPI_MODE,
    #[doc = "0x10a - Clock and Burst Size Setup Register"]
    pub sd_clock_and_burst_size_setup: SD_CLOCK_AND_BURST_SIZE_SETUP,
    #[doc = "0x10c - CE-ATA Register 1"]
    pub sd_ce_ata_1: SD_CE_ATA_1,
    #[doc = "0x10e - CE-ATA Register 2"]
    pub sd_ce_ata_2: SD_CE_ATA_2,
    #[doc = "0x110 - PAD I/O Setup Register"]
    pub sd_pad_io_setup: SD_PAD_IO_SETUP,
    #[doc = "0x114 - RX Configuration Register"]
    pub rx_cfg_reg: RX_CFG_REG,
    #[doc = "0x118 - TX Configuration Register"]
    pub tx_cfg_reg: TX_CFG_REG,
    #[doc = "0x11c - TUNING CONFIG Register"]
    pub tuning_cfg_reg: TUNING_CFG_REG,
}
#[doc = "sd_sys_addr_low (rw) register accessor: an alias for `Reg<SD_SYS_ADDR_LOW_SPEC>`"]
pub type SD_SYS_ADDR_LOW = crate::Reg<sd_sys_addr_low::SD_SYS_ADDR_LOW_SPEC>;
#[doc = "System Address Low Register"]
pub mod sd_sys_addr_low;
#[doc = "sd_sys_addr_high (rw) register accessor: an alias for `Reg<SD_SYS_ADDR_HIGH_SPEC>`"]
pub type SD_SYS_ADDR_HIGH = crate::Reg<sd_sys_addr_high::SD_SYS_ADDR_HIGH_SPEC>;
#[doc = "System Address High Register"]
pub mod sd_sys_addr_high;
#[doc = "sd_block_size (rw) register accessor: an alias for `Reg<SD_BLOCK_SIZE_SPEC>`"]
pub type SD_BLOCK_SIZE = crate::Reg<sd_block_size::SD_BLOCK_SIZE_SPEC>;
#[doc = "Block Size Register"]
pub mod sd_block_size;
#[doc = "sd_block_count (rw) register accessor: an alias for `Reg<SD_BLOCK_COUNT_SPEC>`"]
pub type SD_BLOCK_COUNT = crate::Reg<sd_block_count::SD_BLOCK_COUNT_SPEC>;
#[doc = "Block Count Register"]
pub mod sd_block_count;
#[doc = "sd_arg_low (rw) register accessor: an alias for `Reg<SD_ARG_LOW_SPEC>`"]
pub type SD_ARG_LOW = crate::Reg<sd_arg_low::SD_ARG_LOW_SPEC>;
#[doc = "Argument Low Register"]
pub mod sd_arg_low;
#[doc = "sd_arg_high (rw) register accessor: an alias for `Reg<SD_ARG_HIGH_SPEC>`"]
pub type SD_ARG_HIGH = crate::Reg<sd_arg_high::SD_ARG_HIGH_SPEC>;
#[doc = "Argument High Register"]
pub mod sd_arg_high;
#[doc = "sd_transfer_mode (rw) register accessor: an alias for `Reg<SD_TRANSFER_MODE_SPEC>`"]
pub type SD_TRANSFER_MODE = crate::Reg<sd_transfer_mode::SD_TRANSFER_MODE_SPEC>;
#[doc = "Transfer Mode Register"]
pub mod sd_transfer_mode;
#[doc = "sd_cmd (rw) register accessor: an alias for `Reg<SD_CMD_SPEC>`"]
pub type SD_CMD = crate::Reg<sd_cmd::SD_CMD_SPEC>;
#[doc = "Command Register"]
pub mod sd_cmd;
#[doc = "sd_resp_0 (rw) register accessor: an alias for `Reg<SD_RESP_0_SPEC>`"]
pub type SD_RESP_0 = crate::Reg<sd_resp_0::SD_RESP_0_SPEC>;
#[doc = "Response Register 0"]
pub mod sd_resp_0;
#[doc = "sd_resp_1 (rw) register accessor: an alias for `Reg<SD_RESP_1_SPEC>`"]
pub type SD_RESP_1 = crate::Reg<sd_resp_1::SD_RESP_1_SPEC>;
#[doc = "Response Register 1"]
pub mod sd_resp_1;
#[doc = "sd_resp_2 (rw) register accessor: an alias for `Reg<SD_RESP_2_SPEC>`"]
pub type SD_RESP_2 = crate::Reg<sd_resp_2::SD_RESP_2_SPEC>;
#[doc = "Response Register 2"]
pub mod sd_resp_2;
#[doc = "sd_resp_3 (rw) register accessor: an alias for `Reg<SD_RESP_3_SPEC>`"]
pub type SD_RESP_3 = crate::Reg<sd_resp_3::SD_RESP_3_SPEC>;
#[doc = "Response Register 3"]
pub mod sd_resp_3;
#[doc = "sd_resp_4 (rw) register accessor: an alias for `Reg<SD_RESP_4_SPEC>`"]
pub type SD_RESP_4 = crate::Reg<sd_resp_4::SD_RESP_4_SPEC>;
#[doc = "Response Register 4"]
pub mod sd_resp_4;
#[doc = "sd_resp_5 (rw) register accessor: an alias for `Reg<SD_RESP_5_SPEC>`"]
pub type SD_RESP_5 = crate::Reg<sd_resp_5::SD_RESP_5_SPEC>;
#[doc = "Response Register 5"]
pub mod sd_resp_5;
#[doc = "sd_resp_6 (rw) register accessor: an alias for `Reg<SD_RESP_6_SPEC>`"]
pub type SD_RESP_6 = crate::Reg<sd_resp_6::SD_RESP_6_SPEC>;
#[doc = "Response Register 6"]
pub mod sd_resp_6;
#[doc = "sd_resp_7 (rw) register accessor: an alias for `Reg<SD_RESP_7_SPEC>`"]
pub type SD_RESP_7 = crate::Reg<sd_resp_7::SD_RESP_7_SPEC>;
#[doc = "Response Register 7"]
pub mod sd_resp_7;
#[doc = "sd_buffer_data_port_0 (rw) register accessor: an alias for `Reg<SD_BUFFER_DATA_PORT_0_SPEC>`"]
pub type SD_BUFFER_DATA_PORT_0 = crate::Reg<sd_buffer_data_port_0::SD_BUFFER_DATA_PORT_0_SPEC>;
#[doc = "Buffer Data Port 0 Register"]
pub mod sd_buffer_data_port_0;
#[doc = "sd_buffer_data_port_1 (rw) register accessor: an alias for `Reg<SD_BUFFER_DATA_PORT_1_SPEC>`"]
pub type SD_BUFFER_DATA_PORT_1 = crate::Reg<sd_buffer_data_port_1::SD_BUFFER_DATA_PORT_1_SPEC>;
#[doc = "Buffer Data Port 1 Register"]
pub mod sd_buffer_data_port_1;
#[doc = "sd_present_state_1 (rw) register accessor: an alias for `Reg<SD_PRESENT_STATE_1_SPEC>`"]
pub type SD_PRESENT_STATE_1 = crate::Reg<sd_present_state_1::SD_PRESENT_STATE_1_SPEC>;
#[doc = "Present State Register 1"]
pub mod sd_present_state_1;
#[doc = "sd_present_state_2 (rw) register accessor: an alias for `Reg<SD_PRESENT_STATE_2_SPEC>`"]
pub type SD_PRESENT_STATE_2 = crate::Reg<sd_present_state_2::SD_PRESENT_STATE_2_SPEC>;
#[doc = "Present State Register 2"]
pub mod sd_present_state_2;
#[doc = "sd_host_ctrl (rw) register accessor: an alias for `Reg<SD_HOST_CTRL_SPEC>`"]
pub type SD_HOST_CTRL = crate::Reg<sd_host_ctrl::SD_HOST_CTRL_SPEC>;
#[doc = "Host Control Register"]
pub mod sd_host_ctrl;
#[doc = "sd_block_gap_ctrl (rw) register accessor: an alias for `Reg<SD_BLOCK_GAP_CTRL_SPEC>`"]
pub type SD_BLOCK_GAP_CTRL = crate::Reg<sd_block_gap_ctrl::SD_BLOCK_GAP_CTRL_SPEC>;
#[doc = "Block Gap Control Register"]
pub mod sd_block_gap_ctrl;
#[doc = "sd_clock_ctrl (rw) register accessor: an alias for `Reg<SD_CLOCK_CTRL_SPEC>`"]
pub type SD_CLOCK_CTRL = crate::Reg<sd_clock_ctrl::SD_CLOCK_CTRL_SPEC>;
#[doc = "Clock Control Register"]
pub mod sd_clock_ctrl;
#[doc = "sd_timeout_ctrl_sw_reset (rw) register accessor: an alias for `Reg<SD_TIMEOUT_CTRL_SW_RESET_SPEC>`"]
pub type SD_TIMEOUT_CTRL_SW_RESET =
    crate::Reg<sd_timeout_ctrl_sw_reset::SD_TIMEOUT_CTRL_SW_RESET_SPEC>;
#[doc = "Timeout Control/Software Reset Register"]
pub mod sd_timeout_ctrl_sw_reset;
#[doc = "sd_normal_int_status (rw) register accessor: an alias for `Reg<SD_NORMAL_INT_STATUS_SPEC>`"]
pub type SD_NORMAL_INT_STATUS = crate::Reg<sd_normal_int_status::SD_NORMAL_INT_STATUS_SPEC>;
#[doc = "Normal Interrupt Status Register"]
pub mod sd_normal_int_status;
#[doc = "sd_error_int_status (rw) register accessor: an alias for `Reg<SD_ERROR_INT_STATUS_SPEC>`"]
pub type SD_ERROR_INT_STATUS = crate::Reg<sd_error_int_status::SD_ERROR_INT_STATUS_SPEC>;
#[doc = "Error Interrupt Status Register"]
pub mod sd_error_int_status;
#[doc = "sd_normal_int_status_en (rw) register accessor: an alias for `Reg<SD_NORMAL_INT_STATUS_EN_SPEC>`"]
pub type SD_NORMAL_INT_STATUS_EN =
    crate::Reg<sd_normal_int_status_en::SD_NORMAL_INT_STATUS_EN_SPEC>;
#[doc = "Normal Interrupt Status Enable Register"]
pub mod sd_normal_int_status_en;
#[doc = "sd_error_int_status_en (rw) register accessor: an alias for `Reg<SD_ERROR_INT_STATUS_EN_SPEC>`"]
pub type SD_ERROR_INT_STATUS_EN = crate::Reg<sd_error_int_status_en::SD_ERROR_INT_STATUS_EN_SPEC>;
#[doc = "Error Interrupt Status Enable Register"]
pub mod sd_error_int_status_en;
#[doc = "sd_normal_int_status_int_en (rw) register accessor: an alias for `Reg<SD_NORMAL_INT_STATUS_INT_EN_SPEC>`"]
pub type SD_NORMAL_INT_STATUS_INT_EN =
    crate::Reg<sd_normal_int_status_int_en::SD_NORMAL_INT_STATUS_INT_EN_SPEC>;
#[doc = "Normal Interrupt Status Interrupt Enable Register"]
pub mod sd_normal_int_status_int_en;
#[doc = "sd_error_int_status_int_en (rw) register accessor: an alias for `Reg<SD_ERROR_INT_STATUS_INT_EN_SPEC>`"]
pub type SD_ERROR_INT_STATUS_INT_EN =
    crate::Reg<sd_error_int_status_int_en::SD_ERROR_INT_STATUS_INT_EN_SPEC>;
#[doc = "Error Interrupt Status Interrupt Enable Register"]
pub mod sd_error_int_status_int_en;
#[doc = "sd_auto_cmd12_error_status (rw) register accessor: an alias for `Reg<SD_AUTO_CMD12_ERROR_STATUS_SPEC>`"]
pub type SD_AUTO_CMD12_ERROR_STATUS =
    crate::Reg<sd_auto_cmd12_error_status::SD_AUTO_CMD12_ERROR_STATUS_SPEC>;
#[doc = "Auto CMD12 Error Status Register"]
pub mod sd_auto_cmd12_error_status;
#[doc = "host_ctrl_2 (rw) register accessor: an alias for `Reg<HOST_CTRL_2_SPEC>`"]
pub type HOST_CTRL_2 = crate::Reg<host_ctrl_2::HOST_CTRL_2_SPEC>;
#[doc = "Host Control 2 Register"]
pub mod host_ctrl_2;
#[doc = "sd_capabilities_1 (rw) register accessor: an alias for `Reg<SD_CAPABILITIES_1_SPEC>`"]
pub type SD_CAPABILITIES_1 = crate::Reg<sd_capabilities_1::SD_CAPABILITIES_1_SPEC>;
#[doc = "Capabilities Register 1"]
pub mod sd_capabilities_1;
#[doc = "sd_capabilities_2 (rw) register accessor: an alias for `Reg<SD_CAPABILITIES_2_SPEC>`"]
pub type SD_CAPABILITIES_2 = crate::Reg<sd_capabilities_2::SD_CAPABILITIES_2_SPEC>;
#[doc = "Capabilities Register 2"]
pub mod sd_capabilities_2;
#[doc = "sd_capabilities_3 (rw) register accessor: an alias for `Reg<SD_CAPABILITIES_3_SPEC>`"]
pub type SD_CAPABILITIES_3 = crate::Reg<sd_capabilities_3::SD_CAPABILITIES_3_SPEC>;
#[doc = "Capabilities Register 3"]
pub mod sd_capabilities_3;
#[doc = "sd_capabilities_4 (rw) register accessor: an alias for `Reg<SD_CAPABILITIES_4_SPEC>`"]
pub type SD_CAPABILITIES_4 = crate::Reg<sd_capabilities_4::SD_CAPABILITIES_4_SPEC>;
#[doc = "Capabilities Register 4"]
pub mod sd_capabilities_4;
#[doc = "sd_max_current_1 (rw) register accessor: an alias for `Reg<SD_MAX_CURRENT_1_SPEC>`"]
pub type SD_MAX_CURRENT_1 = crate::Reg<sd_max_current_1::SD_MAX_CURRENT_1_SPEC>;
#[doc = "Maximum Current Register 1"]
pub mod sd_max_current_1;
#[doc = "sd_max_current_2 (rw) register accessor: an alias for `Reg<SD_MAX_CURRENT_2_SPEC>`"]
pub type SD_MAX_CURRENT_2 = crate::Reg<sd_max_current_2::SD_MAX_CURRENT_2_SPEC>;
#[doc = "Maximum Current Register 2"]
pub mod sd_max_current_2;
#[doc = "sd_max_current_3 (rw) register accessor: an alias for `Reg<SD_MAX_CURRENT_3_SPEC>`"]
pub type SD_MAX_CURRENT_3 = crate::Reg<sd_max_current_3::SD_MAX_CURRENT_3_SPEC>;
#[doc = "Maximum Current Register 3"]
pub mod sd_max_current_3;
#[doc = "sd_max_current_4 (rw) register accessor: an alias for `Reg<SD_MAX_CURRENT_4_SPEC>`"]
pub type SD_MAX_CURRENT_4 = crate::Reg<sd_max_current_4::SD_MAX_CURRENT_4_SPEC>;
#[doc = "Maximum Current Register 4"]
pub mod sd_max_current_4;
#[doc = "sd_force_event_auto_cmd12_error (rw) register accessor: an alias for `Reg<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>`"]
pub type SD_FORCE_EVENT_AUTO_CMD12_ERROR =
    crate::Reg<sd_force_event_auto_cmd12_error::SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>;
#[doc = "Force Event Auto cmd12 Error Register"]
pub mod sd_force_event_auto_cmd12_error;
#[doc = "sd_force_event_for_error_status (rw) register accessor: an alias for `Reg<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>`"]
pub type SD_FORCE_EVENT_FOR_ERROR_STATUS =
    crate::Reg<sd_force_event_for_error_status::SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>;
#[doc = "Force Event for Error Status Register"]
pub mod sd_force_event_for_error_status;
#[doc = "sd_adma_error_status (rw) register accessor: an alias for `Reg<SD_ADMA_ERROR_STATUS_SPEC>`"]
pub type SD_ADMA_ERROR_STATUS = crate::Reg<sd_adma_error_status::SD_ADMA_ERROR_STATUS_SPEC>;
#[doc = "ADMA Error Status Register"]
pub mod sd_adma_error_status;
#[doc = "sd_adma_sys_addr_1 (rw) register accessor: an alias for `Reg<SD_ADMA_SYS_ADDR_1_SPEC>`"]
pub type SD_ADMA_SYS_ADDR_1 = crate::Reg<sd_adma_sys_addr_1::SD_ADMA_SYS_ADDR_1_SPEC>;
#[doc = "ADMA System Address Register 1"]
pub mod sd_adma_sys_addr_1;
#[doc = "sd_adma_sys_addr_2 (rw) register accessor: an alias for `Reg<SD_ADMA_SYS_ADDR_2_SPEC>`"]
pub type SD_ADMA_SYS_ADDR_2 = crate::Reg<sd_adma_sys_addr_2::SD_ADMA_SYS_ADDR_2_SPEC>;
#[doc = "ADMA System Address Register 2"]
pub mod sd_adma_sys_addr_2;
#[doc = "sd_adma_sys_addr_3 (rw) register accessor: an alias for `Reg<SD_ADMA_SYS_ADDR_3_SPEC>`"]
pub type SD_ADMA_SYS_ADDR_3 = crate::Reg<sd_adma_sys_addr_3::SD_ADMA_SYS_ADDR_3_SPEC>;
#[doc = "ADMA System Address Register 3"]
pub mod sd_adma_sys_addr_3;
#[doc = "sd_adma_sys_addr_4 (rw) register accessor: an alias for `Reg<SD_ADMA_SYS_ADDR_4_SPEC>`"]
pub type SD_ADMA_SYS_ADDR_4 = crate::Reg<sd_adma_sys_addr_4::SD_ADMA_SYS_ADDR_4_SPEC>;
#[doc = "ADMA System Address Register 4"]
pub mod sd_adma_sys_addr_4;
#[doc = "preset_value_for_init (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_INIT_SPEC>`"]
pub type PRESET_VALUE_FOR_INIT = crate::Reg<preset_value_for_init::PRESET_VALUE_FOR_INIT_SPEC>;
#[doc = "Preset Value Register for Initialization"]
pub mod preset_value_for_init;
#[doc = "preset_value_for_ds (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_DS_SPEC>`"]
pub type PRESET_VALUE_FOR_DS = crate::Reg<preset_value_for_ds::PRESET_VALUE_FOR_DS_SPEC>;
#[doc = "Preset Value Register for Default Speed"]
pub mod preset_value_for_ds;
#[doc = "preset_value_for_hs (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_HS_SPEC>`"]
pub type PRESET_VALUE_FOR_HS = crate::Reg<preset_value_for_hs::PRESET_VALUE_FOR_HS_SPEC>;
#[doc = "Preset Value Register for High Speed"]
pub mod preset_value_for_hs;
#[doc = "preset_value_for_sdr12 (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_SDR12_SPEC>`"]
pub type PRESET_VALUE_FOR_SDR12 = crate::Reg<preset_value_for_sdr12::PRESET_VALUE_FOR_SDR12_SPEC>;
#[doc = "Preset Value Register for SDR12"]
pub mod preset_value_for_sdr12;
#[doc = "preset_value_for_sdr25 (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_SDR25_SPEC>`"]
pub type PRESET_VALUE_FOR_SDR25 = crate::Reg<preset_value_for_sdr25::PRESET_VALUE_FOR_SDR25_SPEC>;
#[doc = "Preset Value Register for SDR25"]
pub mod preset_value_for_sdr25;
#[doc = "preset_value_for_sdr50 (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_SDR50_SPEC>`"]
pub type PRESET_VALUE_FOR_SDR50 = crate::Reg<preset_value_for_sdr50::PRESET_VALUE_FOR_SDR50_SPEC>;
#[doc = "Preset Value Register for SDR50"]
pub mod preset_value_for_sdr50;
#[doc = "preset_value_for_sdr104 (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_SDR104_SPEC>`"]
pub type PRESET_VALUE_FOR_SDR104 =
    crate::Reg<preset_value_for_sdr104::PRESET_VALUE_FOR_SDR104_SPEC>;
#[doc = "Preset Value Register for SDR104"]
pub mod preset_value_for_sdr104;
#[doc = "preset_value_for_ddr50 (rw) register accessor: an alias for `Reg<PRESET_VALUE_FOR_DDR50_SPEC>`"]
pub type PRESET_VALUE_FOR_DDR50 = crate::Reg<preset_value_for_ddr50::PRESET_VALUE_FOR_DDR50_SPEC>;
#[doc = "Preset Value Register for DDR50"]
pub mod preset_value_for_ddr50;
#[doc = "shared_bus_ctrl (rw) register accessor: an alias for `Reg<SHARED_BUS_CTRL_SPEC>`"]
pub type SHARED_BUS_CTRL = crate::Reg<shared_bus_ctrl::SHARED_BUS_CTRL_SPEC>;
#[doc = "Shared Bus Control Register"]
pub mod shared_bus_ctrl;
#[doc = "sd_slot_int_status (rw) register accessor: an alias for `Reg<SD_SLOT_INT_STATUS_SPEC>`"]
pub type SD_SLOT_INT_STATUS = crate::Reg<sd_slot_int_status::SD_SLOT_INT_STATUS_SPEC>;
#[doc = "Slot Interrupt Status Register"]
pub mod sd_slot_int_status;
#[doc = "sd_host_ctrl_ver (rw) register accessor: an alias for `Reg<SD_HOST_CTRL_VER_SPEC>`"]
pub type SD_HOST_CTRL_VER = crate::Reg<sd_host_ctrl_ver::SD_HOST_CTRL_VER_SPEC>;
#[doc = "Host Control Version Register"]
pub mod sd_host_ctrl_ver;
#[doc = "sd_cfg_fifo_param (rw) register accessor: an alias for `Reg<SD_CFG_FIFO_PARAM_SPEC>`"]
pub type SD_CFG_FIFO_PARAM = crate::Reg<sd_cfg_fifo_param::SD_CFG_FIFO_PARAM_SPEC>;
#[doc = "SD Extra Parameters Register"]
pub mod sd_cfg_fifo_param;
#[doc = "sd_fifo_param (rw) register accessor: an alias for `Reg<SD_FIFO_PARAM_SPEC>`"]
pub type SD_FIFO_PARAM = crate::Reg<sd_fifo_param::SD_FIFO_PARAM_SPEC>;
#[doc = "FIFO Parameters Register"]
pub mod sd_fifo_param;
#[doc = "sd_spi_mode (rw) register accessor: an alias for `Reg<SD_SPI_MODE_SPEC>`"]
pub type SD_SPI_MODE = crate::Reg<sd_spi_mode::SD_SPI_MODE_SPEC>;
#[doc = "SPI Mode Register"]
pub mod sd_spi_mode;
#[doc = "sd_clock_and_burst_size_setup (rw) register accessor: an alias for `Reg<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>`"]
pub type SD_CLOCK_AND_BURST_SIZE_SETUP =
    crate::Reg<sd_clock_and_burst_size_setup::SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>;
#[doc = "Clock and Burst Size Setup Register"]
pub mod sd_clock_and_burst_size_setup;
#[doc = "sd_ce_ata_1 (rw) register accessor: an alias for `Reg<SD_CE_ATA_1_SPEC>`"]
pub type SD_CE_ATA_1 = crate::Reg<sd_ce_ata_1::SD_CE_ATA_1_SPEC>;
#[doc = "CE-ATA Register 1"]
pub mod sd_ce_ata_1;
#[doc = "sd_ce_ata_2 (rw) register accessor: an alias for `Reg<SD_CE_ATA_2_SPEC>`"]
pub type SD_CE_ATA_2 = crate::Reg<sd_ce_ata_2::SD_CE_ATA_2_SPEC>;
#[doc = "CE-ATA Register 2"]
pub mod sd_ce_ata_2;
#[doc = "sd_pad_io_setup (rw) register accessor: an alias for `Reg<SD_PAD_IO_SETUP_SPEC>`"]
pub type SD_PAD_IO_SETUP = crate::Reg<sd_pad_io_setup::SD_PAD_IO_SETUP_SPEC>;
#[doc = "PAD I/O Setup Register"]
pub mod sd_pad_io_setup;
#[doc = "rx_cfg_reg (rw) register accessor: an alias for `Reg<RX_CFG_REG_SPEC>`"]
pub type RX_CFG_REG = crate::Reg<rx_cfg_reg::RX_CFG_REG_SPEC>;
#[doc = "RX Configuration Register"]
pub mod rx_cfg_reg;
#[doc = "tx_cfg_reg (rw) register accessor: an alias for `Reg<TX_CFG_REG_SPEC>`"]
pub type TX_CFG_REG = crate::Reg<tx_cfg_reg::TX_CFG_REG_SPEC>;
#[doc = "TX Configuration Register"]
pub mod tx_cfg_reg;
#[doc = "tuning_cfg_reg (rw) register accessor: an alias for `Reg<TUNING_CFG_REG_SPEC>`"]
pub type TUNING_CFG_REG = crate::Reg<tuning_cfg_reg::TUNING_CFG_REG_SPEC>;
#[doc = "TUNING CONFIG Register"]
pub mod tuning_cfg_reg;
