#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - soc_info0"]
    pub soc_info0: SOC_INFO0,
    _reserved1: [u8; 0x4c],
    #[doc = "0x50 - core_cfg16"]
    pub core_cfg16: CORE_CFG16,
    #[doc = "0x54 - core_cfg17"]
    pub core_cfg17: CORE_CFG17,
    #[doc = "0x58 - core_cfg18"]
    pub core_cfg18: CORE_CFG18,
    #[doc = "0x5c - core_cfg19"]
    pub core_cfg19: CORE_CFG19,
    #[doc = "0x60 - core_cfg20"]
    pub core_cfg20: CORE_CFG20,
    #[doc = "0x64 - core_cfg21"]
    pub core_cfg21: CORE_CFG21,
    #[doc = "0x68 - core_cfg22"]
    pub core_cfg22: CORE_CFG22,
    #[doc = "0x6c - core_cfg23"]
    pub core_cfg23: CORE_CFG23,
    #[doc = "0x70 - core_cfg24"]
    pub core_cfg24: CORE_CFG24,
    #[doc = "0x74 - core_cfg25"]
    pub core_cfg25: CORE_CFG25,
    _reserved11: [u8; 0x18],
    #[doc = "0x90 - sys_cfg0"]
    pub sys_cfg0: SYS_CFG0,
    #[doc = "0x94 - sys_cfg1"]
    pub sys_cfg1: SYS_CFG1,
    _reserved13: [u8; 0x08],
    #[doc = "0xa0 - bus_cfg0"]
    pub bus_cfg0: BUS_CFG0,
    _reserved14: [u8; 0x3c],
    #[doc = "0xe0 - emi_cfg0"]
    pub emi_cfg0: EMI_CFG0,
    _reserved15: [u8; 0x0c],
    #[doc = "0xf0 - rtc_cfg0"]
    pub rtc_cfg0: RTC_CFG0,
    _reserved16: [u8; 0x1c],
    #[doc = "0x110 - adc_cfg0"]
    pub adc_cfg0: ADC_CFG0,
    _reserved17: [u8; 0x0c],
    #[doc = "0x120 - dac_cfg0"]
    pub dac_cfg0: DAC_CFG0,
    #[doc = "0x124 - dac_cfg1"]
    pub dac_cfg1: DAC_CFG1,
    #[doc = "0x128 - dac_cfg2"]
    pub dac_cfg2: DAC_CFG2,
    #[doc = "0x12c - dac_cfg3"]
    pub dac_cfg3: DAC_CFG3,
    #[doc = "0x130 - dma_cfg0"]
    pub dma_cfg0: DMA_CFG0,
    #[doc = "0x134 - dma_cfg1"]
    pub dma_cfg1: DMA_CFG1,
    #[doc = "0x138 - dma_cfg2"]
    pub dma_cfg2: DMA_CFG2,
    _reserved24: [u8; 0x04],
    #[doc = "0x140 - ir_cfg0"]
    pub ir_cfg0: IR_CFG0,
    #[doc = "0x144 - ir_cfg1"]
    pub ir_cfg1: IR_CFG1,
    _reserved26: [u8; 0x08],
    #[doc = "0x150 - uart_cfg0"]
    pub uart_cfg0: UART_CFG0,
    #[doc = "0x154 - uart_cfg1"]
    pub uart_cfg1: UART_CFG1,
    #[doc = "0x158 - uart_cfg2"]
    pub uart_cfg2: UART_CFG2,
    _reserved29: [u8; 0x14],
    #[doc = "0x170 - sf_cfg0"]
    pub sf_cfg0: SF_CFG0,
    _reserved30: [u8; 0x0c],
    #[doc = "0x180 - i2c_cfg0"]
    pub i2c_cfg0: I2C_CFG0,
    _reserved31: [u8; 0x0c],
    #[doc = "0x190 - i2s_cfg0"]
    pub i2s_cfg0: I2S_CFG0,
    _reserved32: [u8; 0x1c],
    #[doc = "0x1b0 - spi_cfg0"]
    pub spi_cfg0: SPI_CFG0,
    _reserved33: [u8; 0x0c],
    #[doc = "0x1c0 - qdec_cfg0"]
    pub qdec_cfg0: QDEC_CFG0,
    _reserved34: [u8; 0x0c],
    #[doc = "0x1d0 - pwm_cfg0"]
    pub pwm_cfg0: PWM_CFG0,
    _reserved35: [u8; 0x0c],
    #[doc = "0x1e0 - pdm_cfg0"]
    pub pdm_cfg0: PDM_CFG0,
    _reserved36: [u8; 0x6c],
    #[doc = "0x250 - dig_clk_cfg0"]
    pub dig_clk_cfg0: DIG_CLK_CFG0,
    #[doc = "0x254 - dig_clk_cfg1"]
    pub dig_clk_cfg1: DIG_CLK_CFG1,
    #[doc = "0x258 - dig_clk_cfg2"]
    pub dig_clk_cfg2: DIG_CLK_CFG2,
    #[doc = "0x25c - dig_clk_cfg3"]
    pub dig_clk_cfg3: DIG_CLK_CFG3,
    #[doc = "0x260 - rf_cfg0"]
    pub rf_cfg0: RF_CFG0,
    _reserved41: [u8; 0x7c],
    #[doc = "0x2e0 - dbg_cfg0"]
    pub dbg_cfg0: DBG_CFG0,
    #[doc = "0x2e4 - dbg_cfg1"]
    pub dbg_cfg1: DBG_CFG1,
    #[doc = "0x2e8 - dbg_cfg2"]
    pub dbg_cfg2: DBG_CFG2,
    #[doc = "0x2ec - dbg_cfg3"]
    pub dbg_cfg3: DBG_CFG3,
    #[doc = "0x2f0 - dbg_cfg4"]
    pub dbg_cfg4: DBG_CFG4,
    _reserved46: [u8; 0x0c],
    #[doc = "0x300 - mbist_cfg0"]
    pub mbist_cfg0: MBIST_CFG0,
    _reserved47: [u8; 0x1c],
    #[doc = "0x320 - bmx_cfg0"]
    pub bmx_cfg0: BMX_CFG0,
    #[doc = "0x324 - bmx_cfg1"]
    pub bmx_cfg1: BMX_CFG1,
    #[doc = "0x328 - bmx_cfg2"]
    pub bmx_cfg2: BMX_CFG2,
    #[doc = "0x32c - bmx_cfg3"]
    pub bmx_cfg3: BMX_CFG3,
    #[doc = "0x330 - bmx_cfg4"]
    pub bmx_cfg4: BMX_CFG4,
    #[doc = "0x334 - bmx_cfg5"]
    pub bmx_cfg5: BMX_CFG5,
    #[doc = "0x338 - bmx_cfg6"]
    pub bmx_cfg6: BMX_CFG6,
    _reserved54: [u8; 0x04],
    #[doc = "0x340 - audio_cfg0"]
    pub audio_cfg0: AUDIO_CFG0,
    #[doc = "0x344 - audio_cfg1"]
    pub audio_cfg1: AUDIO_CFG1,
    _reserved56: [u8; 0x48],
    #[doc = "0x390 - eth_cfg0"]
    pub eth_cfg0: ETH_CFG0,
    _reserved57: [u8; 0x8c],
    #[doc = "0x420 - cam_cfg0"]
    pub cam_cfg0: CAM_CFG0,
    _reserved58: [u8; 0x0c],
    #[doc = "0x430 - sdh_cfg0"]
    pub sdh_cfg0: SDH_CFG0,
    _reserved59: [u8; 0x5c],
    #[doc = "0x490 - tzc_cfg0"]
    pub tzc_cfg0: TZC_CFG0,
    _reserved60: [u8; 0x7c],
    #[doc = "0x510 - glb_parm_cfg0"]
    pub glb_parm_cfg0: GLB_PARM_CFG0,
    _reserved61: [u8; 0x0c],
    #[doc = "0x520 - debug_cfg0"]
    pub debug_cfg0: DEBUG_CFG0,
    #[doc = "0x524 - debug_cfg1"]
    pub debug_cfg1: DEBUG_CFG1,
    _reserved63: [u8; 0x08],
    #[doc = "0x530 - reset_sts0"]
    pub reset_sts0: RESET_STS0,
    _reserved64: [u8; 0x0c],
    #[doc = "0x540 - swrst_s1_ext + swrst_s3 + swrst_s2"]
    pub swrst_cfg0: SWRST_CFG0,
    #[doc = "0x544 - swrst_s1"]
    pub swrst_cfg1: SWRST_CFG1,
    #[doc = "0x548 - swrst_cfg2"]
    pub swrst_cfg2: SWRST_CFG2,
    #[doc = "0x54c - Disable hreset"]
    pub swrst_cfg3: SWRST_CFG3,
    _reserved68: [u8; 0x30],
    #[doc = "0x580 - cgen_m"]
    pub cgen_cfg0: CGEN_CFG0,
    #[doc = "0x584 - cgen_s1a + cgen_s1"]
    pub cgen_cfg1: CGEN_CFG1,
    #[doc = "0x588 - cgen_s1_ext + cgen_s3"]
    pub cgen_cfg2: CGEN_CFG2,
    #[doc = "0x58c - cgen_cfg3"]
    pub cgen_cfg3: CGEN_CFG3,
    _reserved72: [u8; 0x30],
    #[doc = "0x5c0 - hw_rsv0"]
    pub hw_rsv0: HW_RSV0,
    #[doc = "0x5c4 - hw_rsv1"]
    pub hw_rsv1: HW_RSV1,
    #[doc = "0x5c8 - hw_rsv2"]
    pub hw_rsv2: HW_RSV2,
    #[doc = "0x5cc - hw_rsv3"]
    pub hw_rsv3: HW_RSV3,
    _reserved76: [u8; 0x30],
    #[doc = "0x600 - reg_sram_ret"]
    pub sram_cfg0: SRAM_CFG0,
    #[doc = "0x604 - reg_sram_slp"]
    pub sram_cfg1: SRAM_CFG1,
    #[doc = "0x608 - reg_sram_parm"]
    pub sram_cfg2: SRAM_CFG2,
    #[doc = "0x60c - sram_cfg3"]
    pub sram_cfg3: SRAM_CFG3,
    #[doc = "0x610 - reg_sram_parm2"]
    pub sram_cfg4: SRAM_CFG4,
    _reserved81: [u8; 0x0c],
    #[doc = "0x620 - psram_cfg0"]
    pub psram_cfg0: PSRAM_CFG0,
    _reserved82: [u8; 0x9c],
    #[doc = "0x6c0 - ldo28cis"]
    pub ldo28cis: LDO28CIS,
    #[doc = "0x6c4 - ldo18io"]
    pub ldo18io: LDO18IO,
    #[doc = "0x6c8 - ldo15cis"]
    pub ldo15cis: LDO15CIS,
    #[doc = "0x6cc - ldo18flash"]
    pub ldo18flash: LDO18FLASH,
    #[doc = "0x6d0 - ldo12uhs"]
    pub ldo12uhs: LDO12UHS,
    _reserved87: [u8; 0x1c],
    #[doc = "0x6f0 - proc_mon"]
    pub proc_mon: PROC_MON,
    _reserved88: [u8; 0x0c],
    #[doc = "0x700 - dll_cfg0"]
    pub dll_cfg0: DLL_CFG0,
    _reserved89: [u8; 0x8c],
    #[doc = "0x790 - mipi_pll_cfg0"]
    pub mipi_pll_cfg0: MIPI_PLL_CFG0,
    #[doc = "0x794 - mipi_pll_cfg1"]
    pub mipi_pll_cfg1: MIPI_PLL_CFG1,
    #[doc = "0x798 - mipi_pll_cfg2"]
    pub mipi_pll_cfg2: MIPI_PLL_CFG2,
    #[doc = "0x79c - mipi_pll_cfg3"]
    pub mipi_pll_cfg3: MIPI_PLL_CFG3,
    #[doc = "0x7a0 - mipi_pll_cfg4"]
    pub mipi_pll_cfg4: MIPI_PLL_CFG4,
    #[doc = "0x7a4 - mipi_pll_cfg5"]
    pub mipi_pll_cfg5: MIPI_PLL_CFG5,
    #[doc = "0x7a8 - mipi_pll_cfg6"]
    pub mipi_pll_cfg6: MIPI_PLL_CFG6,
    #[doc = "0x7ac - mipi_pll_cfg7"]
    pub mipi_pll_cfg7: MIPI_PLL_CFG7,
    #[doc = "0x7b0 - mipi_pll_cfg8"]
    pub mipi_pll_cfg8: MIPI_PLL_CFG8,
    #[doc = "0x7b4 - mipi_pll_cfg9"]
    pub mipi_pll_cfg9: MIPI_PLL_CFG9,
    _reserved99: [u8; 0x18],
    #[doc = "0x7d0 - uhs_pll_cfg0"]
    pub uhs_pll_cfg0: UHS_PLL_CFG0,
    #[doc = "0x7d4 - uhs_pll_cfg1"]
    pub uhs_pll_cfg1: UHS_PLL_CFG1,
    #[doc = "0x7d8 - uhs_pll_cfg2"]
    pub uhs_pll_cfg2: UHS_PLL_CFG2,
    #[doc = "0x7dc - uhs_pll_cfg3"]
    pub uhs_pll_cfg3: UHS_PLL_CFG3,
    #[doc = "0x7e0 - uhs_pll_cfg4"]
    pub uhs_pll_cfg4: UHS_PLL_CFG4,
    #[doc = "0x7e4 - uhs_pll_cfg5"]
    pub uhs_pll_cfg5: UHS_PLL_CFG5,
    #[doc = "0x7e8 - uhs_pll_cfg6"]
    pub uhs_pll_cfg6: UHS_PLL_CFG6,
    #[doc = "0x7ec - uhs_pll_cfg7"]
    pub uhs_pll_cfg7: UHS_PLL_CFG7,
    #[doc = "0x7f0 - uhs_pll_cfg8"]
    pub uhs_pll_cfg8: UHS_PLL_CFG8,
    #[doc = "0x7f4 - uhs_pll_cfg9"]
    pub uhs_pll_cfg9: UHS_PLL_CFG9,
    _reserved109: [u8; 0x18],
    #[doc = "0x810 - wifi_pll_cfg0"]
    pub wifi_pll_cfg0: WIFI_PLL_CFG0,
    #[doc = "0x814 - wifi_pll_cfg1"]
    pub wifi_pll_cfg1: WIFI_PLL_CFG1,
    #[doc = "0x818 - wifi_pll_cfg2"]
    pub wifi_pll_cfg2: WIFI_PLL_CFG2,
    #[doc = "0x81c - wifi_pll_cfg3"]
    pub wifi_pll_cfg3: WIFI_PLL_CFG3,
    #[doc = "0x820 - wifi_pll_cfg4"]
    pub wifi_pll_cfg4: WIFI_PLL_CFG4,
    #[doc = "0x824 - wifi_pll_cfg5"]
    pub wifi_pll_cfg5: WIFI_PLL_CFG5,
    #[doc = "0x828 - wifi_pll_cfg6"]
    pub wifi_pll_cfg6: WIFI_PLL_CFG6,
    #[doc = "0x82c - wifi_pll_cfg7"]
    pub wifi_pll_cfg7: WIFI_PLL_CFG7,
    #[doc = "0x830 - wifi_pll_cfg8"]
    pub wifi_pll_cfg8: WIFI_PLL_CFG8,
    #[doc = "0x834 - wifi_pll_cfg9"]
    pub wifi_pll_cfg9: WIFI_PLL_CFG9,
    #[doc = "0x838 - wifi_pll_cfg10"]
    pub wifi_pll_cfg10: WIFI_PLL_CFG10,
    #[doc = "0x83c - wifi_pll_cfg11"]
    pub wifi_pll_cfg11: WIFI_PLL_CFG11,
    #[doc = "0x840 - wifi_pll_cfg12"]
    pub wifi_pll_cfg12: WIFI_PLL_CFG12,
    #[doc = "0x844 - wifi_pll_cfg13"]
    pub wifi_pll_cfg13: WIFI_PLL_CFG13,
    _reserved123: [u8; 0x5c],
    #[doc = "0x8a4 - gauge"]
    pub gauge: GAUGE,
    _reserved124: [u8; 0x10],
    #[doc = "0x8b8 - gauge_rx_fifo_ctrl"]
    pub gauge_rx_fifo_ctrl: GAUGE_RX_FIFO_CTRL,
    #[doc = "0x8bc - gauge_rx_fifo_status"]
    pub gauge_rx_fifo_status: GAUGE_RX_FIFO_STATUS,
    #[doc = "0x8c0 - gauge_rx_fifo_data"]
    pub gauge_rx_fifo_data: GAUGE_RX_FIFO_DATA,
    #[doc = "0x8c4 - gpio_cfg0"]
    pub gpio_cfg0: GPIO_CFG0,
    #[doc = "0x8c8 - gpio_cfg1"]
    pub gpio_cfg1: GPIO_CFG1,
    #[doc = "0x8cc - gpio_cfg2"]
    pub gpio_cfg2: GPIO_CFG2,
    #[doc = "0x8d0 - gpio_cfg3"]
    pub gpio_cfg3: GPIO_CFG3,
    #[doc = "0x8d4 - gpio_cfg4"]
    pub gpio_cfg4: GPIO_CFG4,
    #[doc = "0x8d8 - gpio_cfg5"]
    pub gpio_cfg5: GPIO_CFG5,
    #[doc = "0x8dc - gpio_cfg6"]
    pub gpio_cfg6: GPIO_CFG6,
    #[doc = "0x8e0 - gpio_cfg7"]
    pub gpio_cfg7: GPIO_CFG7,
    #[doc = "0x8e4 - gpio_cfg8"]
    pub gpio_cfg8: GPIO_CFG8,
    #[doc = "0x8e8 - gpio_cfg9"]
    pub gpio_cfg9: GPIO_CFG9,
    #[doc = "0x8ec - gpio_cfg10"]
    pub gpio_cfg10: GPIO_CFG10,
    #[doc = "0x8f0 - gpio_cfg11"]
    pub gpio_cfg11: GPIO_CFG11,
    #[doc = "0x8f4 - gpio_cfg12"]
    pub gpio_cfg12: GPIO_CFG12,
    #[doc = "0x8f8 - gpio_cfg13"]
    pub gpio_cfg13: GPIO_CFG13,
    #[doc = "0x8fc - gpio_cfg14"]
    pub gpio_cfg14: GPIO_CFG14,
    #[doc = "0x900 - gpio_cfg15"]
    pub gpio_cfg15: GPIO_CFG15,
    #[doc = "0x904 - gpio_cfg16"]
    pub gpio_cfg16: GPIO_CFG16,
    #[doc = "0x908 - gpio_cfg17"]
    pub gpio_cfg17: GPIO_CFG17,
    #[doc = "0x90c - gpio_cfg18"]
    pub gpio_cfg18: GPIO_CFG18,
    #[doc = "0x910 - gpio_cfg19"]
    pub gpio_cfg19: GPIO_CFG19,
    #[doc = "0x914 - gpio_cfg20"]
    pub gpio_cfg20: GPIO_CFG20,
    #[doc = "0x918 - gpio_cfg21"]
    pub gpio_cfg21: GPIO_CFG21,
    #[doc = "0x91c - gpio_cfg22"]
    pub gpio_cfg22: GPIO_CFG22,
    #[doc = "0x920 - gpio_cfg23"]
    pub gpio_cfg23: GPIO_CFG23,
    #[doc = "0x924 - gpio_cfg24"]
    pub gpio_cfg24: GPIO_CFG24,
    #[doc = "0x928 - gpio_cfg25"]
    pub gpio_cfg25: GPIO_CFG25,
    #[doc = "0x92c - gpio_cfg26"]
    pub gpio_cfg26: GPIO_CFG26,
    #[doc = "0x930 - gpio_cfg27"]
    pub gpio_cfg27: GPIO_CFG27,
    #[doc = "0x934 - gpio_cfg28"]
    pub gpio_cfg28: GPIO_CFG28,
    #[doc = "0x938 - gpio_cfg29"]
    pub gpio_cfg29: GPIO_CFG29,
    #[doc = "0x93c - gpio_cfg30"]
    pub gpio_cfg30: GPIO_CFG30,
    #[doc = "0x940 - gpio_cfg31"]
    pub gpio_cfg31: GPIO_CFG31,
    #[doc = "0x944 - gpio_cfg32"]
    pub gpio_cfg32: GPIO_CFG32,
    #[doc = "0x948 - gpio_cfg33"]
    pub gpio_cfg33: GPIO_CFG33,
    #[doc = "0x94c - gpio_cfg34"]
    pub gpio_cfg34: GPIO_CFG34,
    #[doc = "0x950 - gpio_cfg35"]
    pub gpio_cfg35: GPIO_CFG35,
    #[doc = "0x954 - gpio_cfg36"]
    pub gpio_cfg36: GPIO_CFG36,
    #[doc = "0x958 - gpio_cfg37"]
    pub gpio_cfg37: GPIO_CFG37,
    #[doc = "0x95c - gpio_cfg38"]
    pub gpio_cfg38: GPIO_CFG38,
    #[doc = "0x960 - gpio_cfg39"]
    pub gpio_cfg39: GPIO_CFG39,
    #[doc = "0x964 - gpio_cfg40"]
    pub gpio_cfg40: GPIO_CFG40,
    #[doc = "0x968 - gpio_cfg41"]
    pub gpio_cfg41: GPIO_CFG41,
    #[doc = "0x96c - gpio_cfg42"]
    pub gpio_cfg42: GPIO_CFG42,
    #[doc = "0x970 - gpio_cfg43"]
    pub gpio_cfg43: GPIO_CFG43,
    #[doc = "0x974 - gpio_cfg44"]
    pub gpio_cfg44: GPIO_CFG44,
    #[doc = "0x978 - gpio_cfg45"]
    pub gpio_cfg45: GPIO_CFG45,
    #[doc = "0x97c - gpio_cfg46"]
    pub gpio_cfg46: GPIO_CFG46,
    #[doc = "0x980 - gpio_cfg47"]
    pub gpio_cfg47: GPIO_CFG47,
    #[doc = "0x984 - gpio_cfg48"]
    pub gpio_cfg48: GPIO_CFG48,
    #[doc = "0x988 - gpio_cfg49"]
    pub gpio_cfg49: GPIO_CFG49,
    #[doc = "0x98c - gpio_cfg50"]
    pub gpio_cfg50: GPIO_CFG50,
    #[doc = "0x990 - gpio_cfg51"]
    pub gpio_cfg51: GPIO_CFG51,
    #[doc = "0x994 - gpio_cfg52"]
    pub gpio_cfg52: GPIO_CFG52,
    #[doc = "0x998 - gpio_cfg53"]
    pub gpio_cfg53: GPIO_CFG53,
    #[doc = "0x99c - gpio_cfg54"]
    pub gpio_cfg54: GPIO_CFG54,
    #[doc = "0x9a0 - gpio_cfg55"]
    pub gpio_cfg55: GPIO_CFG55,
    #[doc = "0x9a4 - gpio_cfg56"]
    pub gpio_cfg56: GPIO_CFG56,
    #[doc = "0x9a8 - gpio_cfg57"]
    pub gpio_cfg57: GPIO_CFG57,
    #[doc = "0x9ac - gpio_cfg58"]
    pub gpio_cfg58: GPIO_CFG58,
    #[doc = "0x9b0 - gpio_cfg59"]
    pub gpio_cfg59: GPIO_CFG59,
    #[doc = "0x9b4 - gpio_cfg60"]
    pub gpio_cfg60: GPIO_CFG60,
    #[doc = "0x9b8 - gpio_cfg61"]
    pub gpio_cfg61: GPIO_CFG61,
    #[doc = "0x9bc - gpio_cfg62"]
    pub gpio_cfg62: GPIO_CFG62,
    #[doc = "0x9c0 - gpio_cfg63"]
    pub gpio_cfg63: GPIO_CFG63,
    _reserved191: [u8; 0x0100],
    #[doc = "0xac4 - gpio_cfg128"]
    pub gpio_cfg128: GPIO_CFG128,
    #[doc = "0xac8 - gpio_cfg129"]
    pub gpio_cfg129: GPIO_CFG129,
    _reserved193: [u8; 0x18],
    #[doc = "0xae4 - gpio_cfg136"]
    pub gpio_cfg136: GPIO_CFG136,
    #[doc = "0xae8 - gpio_cfg137"]
    pub gpio_cfg137: GPIO_CFG137,
    #[doc = "0xaec - gpio_cfg138"]
    pub gpio_cfg138: GPIO_CFG138,
    #[doc = "0xaf0 - gpio_cfg139"]
    pub gpio_cfg139: GPIO_CFG139,
    #[doc = "0xaf4 - gpio_cfg140"]
    pub gpio_cfg140: GPIO_CFG140,
    #[doc = "0xaf8 - gpio_cfg141"]
    pub gpio_cfg141: GPIO_CFG141,
    #[doc = "0xafc - gpio_cfg142"]
    pub gpio_cfg142: GPIO_CFG142,
    #[doc = "0xb00 - gpio_cfg143"]
    pub gpio_cfg143: GPIO_CFG143,
}
#[doc = "soc_info0 (rw) register accessor: an alias for `Reg<SOC_INFO0_SPEC>`"]
pub type SOC_INFO0 = crate::Reg<soc_info0::SOC_INFO0_SPEC>;
#[doc = "soc_info0"]
pub mod soc_info0;
#[doc = "core_cfg16 (rw) register accessor: an alias for `Reg<CORE_CFG16_SPEC>`"]
pub type CORE_CFG16 = crate::Reg<core_cfg16::CORE_CFG16_SPEC>;
#[doc = "core_cfg16"]
pub mod core_cfg16;
#[doc = "core_cfg17 (rw) register accessor: an alias for `Reg<CORE_CFG17_SPEC>`"]
pub type CORE_CFG17 = crate::Reg<core_cfg17::CORE_CFG17_SPEC>;
#[doc = "core_cfg17"]
pub mod core_cfg17;
#[doc = "core_cfg18 (rw) register accessor: an alias for `Reg<CORE_CFG18_SPEC>`"]
pub type CORE_CFG18 = crate::Reg<core_cfg18::CORE_CFG18_SPEC>;
#[doc = "core_cfg18"]
pub mod core_cfg18;
#[doc = "core_cfg19 (rw) register accessor: an alias for `Reg<CORE_CFG19_SPEC>`"]
pub type CORE_CFG19 = crate::Reg<core_cfg19::CORE_CFG19_SPEC>;
#[doc = "core_cfg19"]
pub mod core_cfg19;
#[doc = "core_cfg20 (rw) register accessor: an alias for `Reg<CORE_CFG20_SPEC>`"]
pub type CORE_CFG20 = crate::Reg<core_cfg20::CORE_CFG20_SPEC>;
#[doc = "core_cfg20"]
pub mod core_cfg20;
#[doc = "core_cfg21 (rw) register accessor: an alias for `Reg<CORE_CFG21_SPEC>`"]
pub type CORE_CFG21 = crate::Reg<core_cfg21::CORE_CFG21_SPEC>;
#[doc = "core_cfg21"]
pub mod core_cfg21;
#[doc = "core_cfg22 (rw) register accessor: an alias for `Reg<CORE_CFG22_SPEC>`"]
pub type CORE_CFG22 = crate::Reg<core_cfg22::CORE_CFG22_SPEC>;
#[doc = "core_cfg22"]
pub mod core_cfg22;
#[doc = "core_cfg23 (rw) register accessor: an alias for `Reg<CORE_CFG23_SPEC>`"]
pub type CORE_CFG23 = crate::Reg<core_cfg23::CORE_CFG23_SPEC>;
#[doc = "core_cfg23"]
pub mod core_cfg23;
#[doc = "core_cfg24 (rw) register accessor: an alias for `Reg<CORE_CFG24_SPEC>`"]
pub type CORE_CFG24 = crate::Reg<core_cfg24::CORE_CFG24_SPEC>;
#[doc = "core_cfg24"]
pub mod core_cfg24;
#[doc = "core_cfg25 (rw) register accessor: an alias for `Reg<CORE_CFG25_SPEC>`"]
pub type CORE_CFG25 = crate::Reg<core_cfg25::CORE_CFG25_SPEC>;
#[doc = "core_cfg25"]
pub mod core_cfg25;
#[doc = "sys_cfg0 (rw) register accessor: an alias for `Reg<SYS_CFG0_SPEC>`"]
pub type SYS_CFG0 = crate::Reg<sys_cfg0::SYS_CFG0_SPEC>;
#[doc = "sys_cfg0"]
pub mod sys_cfg0;
#[doc = "sys_cfg1 (rw) register accessor: an alias for `Reg<SYS_CFG1_SPEC>`"]
pub type SYS_CFG1 = crate::Reg<sys_cfg1::SYS_CFG1_SPEC>;
#[doc = "sys_cfg1"]
pub mod sys_cfg1;
#[doc = "bus_cfg0 (rw) register accessor: an alias for `Reg<BUS_CFG0_SPEC>`"]
pub type BUS_CFG0 = crate::Reg<bus_cfg0::BUS_CFG0_SPEC>;
#[doc = "bus_cfg0"]
pub mod bus_cfg0;
#[doc = "emi_cfg0 (rw) register accessor: an alias for `Reg<EMI_CFG0_SPEC>`"]
pub type EMI_CFG0 = crate::Reg<emi_cfg0::EMI_CFG0_SPEC>;
#[doc = "emi_cfg0"]
pub mod emi_cfg0;
#[doc = "rtc_cfg0 (rw) register accessor: an alias for `Reg<RTC_CFG0_SPEC>`"]
pub type RTC_CFG0 = crate::Reg<rtc_cfg0::RTC_CFG0_SPEC>;
#[doc = "rtc_cfg0"]
pub mod rtc_cfg0;
#[doc = "adc_cfg0 (rw) register accessor: an alias for `Reg<ADC_CFG0_SPEC>`"]
pub type ADC_CFG0 = crate::Reg<adc_cfg0::ADC_CFG0_SPEC>;
#[doc = "adc_cfg0"]
pub mod adc_cfg0;
#[doc = "dac_cfg0 (rw) register accessor: an alias for `Reg<DAC_CFG0_SPEC>`"]
pub type DAC_CFG0 = crate::Reg<dac_cfg0::DAC_CFG0_SPEC>;
#[doc = "dac_cfg0"]
pub mod dac_cfg0;
#[doc = "dac_cfg1 (rw) register accessor: an alias for `Reg<DAC_CFG1_SPEC>`"]
pub type DAC_CFG1 = crate::Reg<dac_cfg1::DAC_CFG1_SPEC>;
#[doc = "dac_cfg1"]
pub mod dac_cfg1;
#[doc = "dac_cfg2 (rw) register accessor: an alias for `Reg<DAC_CFG2_SPEC>`"]
pub type DAC_CFG2 = crate::Reg<dac_cfg2::DAC_CFG2_SPEC>;
#[doc = "dac_cfg2"]
pub mod dac_cfg2;
#[doc = "dac_cfg3 (rw) register accessor: an alias for `Reg<DAC_CFG3_SPEC>`"]
pub type DAC_CFG3 = crate::Reg<dac_cfg3::DAC_CFG3_SPEC>;
#[doc = "dac_cfg3"]
pub mod dac_cfg3;
#[doc = "dma_cfg0 (rw) register accessor: an alias for `Reg<DMA_CFG0_SPEC>`"]
pub type DMA_CFG0 = crate::Reg<dma_cfg0::DMA_CFG0_SPEC>;
#[doc = "dma_cfg0"]
pub mod dma_cfg0;
#[doc = "dma_cfg1 (rw) register accessor: an alias for `Reg<DMA_CFG1_SPEC>`"]
pub type DMA_CFG1 = crate::Reg<dma_cfg1::DMA_CFG1_SPEC>;
#[doc = "dma_cfg1"]
pub mod dma_cfg1;
#[doc = "dma_cfg2 (rw) register accessor: an alias for `Reg<DMA_CFG2_SPEC>`"]
pub type DMA_CFG2 = crate::Reg<dma_cfg2::DMA_CFG2_SPEC>;
#[doc = "dma_cfg2"]
pub mod dma_cfg2;
#[doc = "ir_cfg0 (rw) register accessor: an alias for `Reg<IR_CFG0_SPEC>`"]
pub type IR_CFG0 = crate::Reg<ir_cfg0::IR_CFG0_SPEC>;
#[doc = "ir_cfg0"]
pub mod ir_cfg0;
#[doc = "ir_cfg1 (rw) register accessor: an alias for `Reg<IR_CFG1_SPEC>`"]
pub type IR_CFG1 = crate::Reg<ir_cfg1::IR_CFG1_SPEC>;
#[doc = "ir_cfg1"]
pub mod ir_cfg1;
#[doc = "uart_cfg0 (rw) register accessor: an alias for `Reg<UART_CFG0_SPEC>`"]
pub type UART_CFG0 = crate::Reg<uart_cfg0::UART_CFG0_SPEC>;
#[doc = "uart_cfg0"]
pub mod uart_cfg0;
#[doc = "uart_cfg1 (rw) register accessor: an alias for `Reg<UART_CFG1_SPEC>`"]
pub type UART_CFG1 = crate::Reg<uart_cfg1::UART_CFG1_SPEC>;
#[doc = "uart_cfg1"]
pub mod uart_cfg1;
#[doc = "uart_cfg2 (rw) register accessor: an alias for `Reg<UART_CFG2_SPEC>`"]
pub type UART_CFG2 = crate::Reg<uart_cfg2::UART_CFG2_SPEC>;
#[doc = "uart_cfg2"]
pub mod uart_cfg2;
#[doc = "sf_cfg0 (rw) register accessor: an alias for `Reg<SF_CFG0_SPEC>`"]
pub type SF_CFG0 = crate::Reg<sf_cfg0::SF_CFG0_SPEC>;
#[doc = "sf_cfg0"]
pub mod sf_cfg0;
#[doc = "i2c_cfg0 (rw) register accessor: an alias for `Reg<I2C_CFG0_SPEC>`"]
pub type I2C_CFG0 = crate::Reg<i2c_cfg0::I2C_CFG0_SPEC>;
#[doc = "i2c_cfg0"]
pub mod i2c_cfg0;
#[doc = "i2s_cfg0 (rw) register accessor: an alias for `Reg<I2S_CFG0_SPEC>`"]
pub type I2S_CFG0 = crate::Reg<i2s_cfg0::I2S_CFG0_SPEC>;
#[doc = "i2s_cfg0"]
pub mod i2s_cfg0;
#[doc = "spi_cfg0 (rw) register accessor: an alias for `Reg<SPI_CFG0_SPEC>`"]
pub type SPI_CFG0 = crate::Reg<spi_cfg0::SPI_CFG0_SPEC>;
#[doc = "spi_cfg0"]
pub mod spi_cfg0;
#[doc = "qdec_cfg0 (rw) register accessor: an alias for `Reg<QDEC_CFG0_SPEC>`"]
pub type QDEC_CFG0 = crate::Reg<qdec_cfg0::QDEC_CFG0_SPEC>;
#[doc = "qdec_cfg0"]
pub mod qdec_cfg0;
#[doc = "pwm_cfg0 (rw) register accessor: an alias for `Reg<PWM_CFG0_SPEC>`"]
pub type PWM_CFG0 = crate::Reg<pwm_cfg0::PWM_CFG0_SPEC>;
#[doc = "pwm_cfg0"]
pub mod pwm_cfg0;
#[doc = "pdm_cfg0 (rw) register accessor: an alias for `Reg<PDM_CFG0_SPEC>`"]
pub type PDM_CFG0 = crate::Reg<pdm_cfg0::PDM_CFG0_SPEC>;
#[doc = "pdm_cfg0"]
pub mod pdm_cfg0;
#[doc = "dig_clk_cfg0 (rw) register accessor: an alias for `Reg<DIG_CLK_CFG0_SPEC>`"]
pub type DIG_CLK_CFG0 = crate::Reg<dig_clk_cfg0::DIG_CLK_CFG0_SPEC>;
#[doc = "dig_clk_cfg0"]
pub mod dig_clk_cfg0;
#[doc = "dig_clk_cfg1 (rw) register accessor: an alias for `Reg<DIG_CLK_CFG1_SPEC>`"]
pub type DIG_CLK_CFG1 = crate::Reg<dig_clk_cfg1::DIG_CLK_CFG1_SPEC>;
#[doc = "dig_clk_cfg1"]
pub mod dig_clk_cfg1;
#[doc = "dig_clk_cfg2 (rw) register accessor: an alias for `Reg<DIG_CLK_CFG2_SPEC>`"]
pub type DIG_CLK_CFG2 = crate::Reg<dig_clk_cfg2::DIG_CLK_CFG2_SPEC>;
#[doc = "dig_clk_cfg2"]
pub mod dig_clk_cfg2;
#[doc = "dig_clk_cfg3 (rw) register accessor: an alias for `Reg<DIG_CLK_CFG3_SPEC>`"]
pub type DIG_CLK_CFG3 = crate::Reg<dig_clk_cfg3::DIG_CLK_CFG3_SPEC>;
#[doc = "dig_clk_cfg3"]
pub mod dig_clk_cfg3;
#[doc = "rf_cfg0 (rw) register accessor: an alias for `Reg<RF_CFG0_SPEC>`"]
pub type RF_CFG0 = crate::Reg<rf_cfg0::RF_CFG0_SPEC>;
#[doc = "rf_cfg0"]
pub mod rf_cfg0;
#[doc = "dbg_cfg0 (rw) register accessor: an alias for `Reg<DBG_CFG0_SPEC>`"]
pub type DBG_CFG0 = crate::Reg<dbg_cfg0::DBG_CFG0_SPEC>;
#[doc = "dbg_cfg0"]
pub mod dbg_cfg0;
#[doc = "dbg_cfg1 (rw) register accessor: an alias for `Reg<DBG_CFG1_SPEC>`"]
pub type DBG_CFG1 = crate::Reg<dbg_cfg1::DBG_CFG1_SPEC>;
#[doc = "dbg_cfg1"]
pub mod dbg_cfg1;
#[doc = "dbg_cfg2 (rw) register accessor: an alias for `Reg<DBG_CFG2_SPEC>`"]
pub type DBG_CFG2 = crate::Reg<dbg_cfg2::DBG_CFG2_SPEC>;
#[doc = "dbg_cfg2"]
pub mod dbg_cfg2;
#[doc = "dbg_cfg3 (rw) register accessor: an alias for `Reg<DBG_CFG3_SPEC>`"]
pub type DBG_CFG3 = crate::Reg<dbg_cfg3::DBG_CFG3_SPEC>;
#[doc = "dbg_cfg3"]
pub mod dbg_cfg3;
#[doc = "dbg_cfg4 (rw) register accessor: an alias for `Reg<DBG_CFG4_SPEC>`"]
pub type DBG_CFG4 = crate::Reg<dbg_cfg4::DBG_CFG4_SPEC>;
#[doc = "dbg_cfg4"]
pub mod dbg_cfg4;
#[doc = "mbist_cfg0 (rw) register accessor: an alias for `Reg<MBIST_CFG0_SPEC>`"]
pub type MBIST_CFG0 = crate::Reg<mbist_cfg0::MBIST_CFG0_SPEC>;
#[doc = "mbist_cfg0"]
pub mod mbist_cfg0;
#[doc = "bmx_cfg0 (rw) register accessor: an alias for `Reg<BMX_CFG0_SPEC>`"]
pub type BMX_CFG0 = crate::Reg<bmx_cfg0::BMX_CFG0_SPEC>;
#[doc = "bmx_cfg0"]
pub mod bmx_cfg0;
#[doc = "bmx_cfg1 (rw) register accessor: an alias for `Reg<BMX_CFG1_SPEC>`"]
pub type BMX_CFG1 = crate::Reg<bmx_cfg1::BMX_CFG1_SPEC>;
#[doc = "bmx_cfg1"]
pub mod bmx_cfg1;
#[doc = "bmx_cfg2 (rw) register accessor: an alias for `Reg<BMX_CFG2_SPEC>`"]
pub type BMX_CFG2 = crate::Reg<bmx_cfg2::BMX_CFG2_SPEC>;
#[doc = "bmx_cfg2"]
pub mod bmx_cfg2;
#[doc = "bmx_cfg3 (rw) register accessor: an alias for `Reg<BMX_CFG3_SPEC>`"]
pub type BMX_CFG3 = crate::Reg<bmx_cfg3::BMX_CFG3_SPEC>;
#[doc = "bmx_cfg3"]
pub mod bmx_cfg3;
#[doc = "bmx_cfg4 (rw) register accessor: an alias for `Reg<BMX_CFG4_SPEC>`"]
pub type BMX_CFG4 = crate::Reg<bmx_cfg4::BMX_CFG4_SPEC>;
#[doc = "bmx_cfg4"]
pub mod bmx_cfg4;
#[doc = "bmx_cfg5 (rw) register accessor: an alias for `Reg<BMX_CFG5_SPEC>`"]
pub type BMX_CFG5 = crate::Reg<bmx_cfg5::BMX_CFG5_SPEC>;
#[doc = "bmx_cfg5"]
pub mod bmx_cfg5;
#[doc = "bmx_cfg6 (rw) register accessor: an alias for `Reg<BMX_CFG6_SPEC>`"]
pub type BMX_CFG6 = crate::Reg<bmx_cfg6::BMX_CFG6_SPEC>;
#[doc = "bmx_cfg6"]
pub mod bmx_cfg6;
#[doc = "audio_cfg0 (rw) register accessor: an alias for `Reg<AUDIO_CFG0_SPEC>`"]
pub type AUDIO_CFG0 = crate::Reg<audio_cfg0::AUDIO_CFG0_SPEC>;
#[doc = "audio_cfg0"]
pub mod audio_cfg0;
#[doc = "audio_cfg1 (rw) register accessor: an alias for `Reg<AUDIO_CFG1_SPEC>`"]
pub type AUDIO_CFG1 = crate::Reg<audio_cfg1::AUDIO_CFG1_SPEC>;
#[doc = "audio_cfg1"]
pub mod audio_cfg1;
#[doc = "eth_cfg0 (rw) register accessor: an alias for `Reg<ETH_CFG0_SPEC>`"]
pub type ETH_CFG0 = crate::Reg<eth_cfg0::ETH_CFG0_SPEC>;
#[doc = "eth_cfg0"]
pub mod eth_cfg0;
#[doc = "cam_cfg0 (rw) register accessor: an alias for `Reg<CAM_CFG0_SPEC>`"]
pub type CAM_CFG0 = crate::Reg<cam_cfg0::CAM_CFG0_SPEC>;
#[doc = "cam_cfg0"]
pub mod cam_cfg0;
#[doc = "sdh_cfg0 (rw) register accessor: an alias for `Reg<SDH_CFG0_SPEC>`"]
pub type SDH_CFG0 = crate::Reg<sdh_cfg0::SDH_CFG0_SPEC>;
#[doc = "sdh_cfg0"]
pub mod sdh_cfg0;
#[doc = "tzc_cfg0 (rw) register accessor: an alias for `Reg<TZC_CFG0_SPEC>`"]
pub type TZC_CFG0 = crate::Reg<tzc_cfg0::TZC_CFG0_SPEC>;
#[doc = "tzc_cfg0"]
pub mod tzc_cfg0;
#[doc = "glb_parm_cfg0 (rw) register accessor: an alias for `Reg<GLB_PARM_CFG0_SPEC>`"]
pub type GLB_PARM_CFG0 = crate::Reg<glb_parm_cfg0::GLB_PARM_CFG0_SPEC>;
#[doc = "glb_parm_cfg0"]
pub mod glb_parm_cfg0;
#[doc = "debug_cfg0 (rw) register accessor: an alias for `Reg<DEBUG_CFG0_SPEC>`"]
pub type DEBUG_CFG0 = crate::Reg<debug_cfg0::DEBUG_CFG0_SPEC>;
#[doc = "debug_cfg0"]
pub mod debug_cfg0;
#[doc = "debug_cfg1 (rw) register accessor: an alias for `Reg<DEBUG_CFG1_SPEC>`"]
pub type DEBUG_CFG1 = crate::Reg<debug_cfg1::DEBUG_CFG1_SPEC>;
#[doc = "debug_cfg1"]
pub mod debug_cfg1;
#[doc = "reset_sts0 (rw) register accessor: an alias for `Reg<RESET_STS0_SPEC>`"]
pub type RESET_STS0 = crate::Reg<reset_sts0::RESET_STS0_SPEC>;
#[doc = "reset_sts0"]
pub mod reset_sts0;
#[doc = "swrst_cfg0 (rw) register accessor: an alias for `Reg<SWRST_CFG0_SPEC>`"]
pub type SWRST_CFG0 = crate::Reg<swrst_cfg0::SWRST_CFG0_SPEC>;
#[doc = "swrst_s1_ext + swrst_s3 + swrst_s2"]
pub mod swrst_cfg0;
#[doc = "swrst_cfg1 (rw) register accessor: an alias for `Reg<SWRST_CFG1_SPEC>`"]
pub type SWRST_CFG1 = crate::Reg<swrst_cfg1::SWRST_CFG1_SPEC>;
#[doc = "swrst_s1"]
pub mod swrst_cfg1;
#[doc = "swrst_cfg2 (rw) register accessor: an alias for `Reg<SWRST_CFG2_SPEC>`"]
pub type SWRST_CFG2 = crate::Reg<swrst_cfg2::SWRST_CFG2_SPEC>;
#[doc = "swrst_cfg2"]
pub mod swrst_cfg2;
#[doc = "swrst_cfg3 (rw) register accessor: an alias for `Reg<SWRST_CFG3_SPEC>`"]
pub type SWRST_CFG3 = crate::Reg<swrst_cfg3::SWRST_CFG3_SPEC>;
#[doc = "Disable hreset"]
pub mod swrst_cfg3;
#[doc = "cgen_cfg0 (rw) register accessor: an alias for `Reg<CGEN_CFG0_SPEC>`"]
pub type CGEN_CFG0 = crate::Reg<cgen_cfg0::CGEN_CFG0_SPEC>;
#[doc = "cgen_m"]
pub mod cgen_cfg0;
#[doc = "cgen_cfg1 (rw) register accessor: an alias for `Reg<CGEN_CFG1_SPEC>`"]
pub type CGEN_CFG1 = crate::Reg<cgen_cfg1::CGEN_CFG1_SPEC>;
#[doc = "cgen_s1a + cgen_s1"]
pub mod cgen_cfg1;
#[doc = "cgen_cfg2 (rw) register accessor: an alias for `Reg<CGEN_CFG2_SPEC>`"]
pub type CGEN_CFG2 = crate::Reg<cgen_cfg2::CGEN_CFG2_SPEC>;
#[doc = "cgen_s1_ext + cgen_s3"]
pub mod cgen_cfg2;
#[doc = "cgen_cfg3 (rw) register accessor: an alias for `Reg<CGEN_CFG3_SPEC>`"]
pub type CGEN_CFG3 = crate::Reg<cgen_cfg3::CGEN_CFG3_SPEC>;
#[doc = "cgen_cfg3"]
pub mod cgen_cfg3;
#[doc = "hw_rsv0 (rw) register accessor: an alias for `Reg<HW_RSV0_SPEC>`"]
pub type HW_RSV0 = crate::Reg<hw_rsv0::HW_RSV0_SPEC>;
#[doc = "hw_rsv0"]
pub mod hw_rsv0;
#[doc = "hw_rsv1 (rw) register accessor: an alias for `Reg<HW_RSV1_SPEC>`"]
pub type HW_RSV1 = crate::Reg<hw_rsv1::HW_RSV1_SPEC>;
#[doc = "hw_rsv1"]
pub mod hw_rsv1;
#[doc = "hw_rsv2 (rw) register accessor: an alias for `Reg<HW_RSV2_SPEC>`"]
pub type HW_RSV2 = crate::Reg<hw_rsv2::HW_RSV2_SPEC>;
#[doc = "hw_rsv2"]
pub mod hw_rsv2;
#[doc = "hw_rsv3 (rw) register accessor: an alias for `Reg<HW_RSV3_SPEC>`"]
pub type HW_RSV3 = crate::Reg<hw_rsv3::HW_RSV3_SPEC>;
#[doc = "hw_rsv3"]
pub mod hw_rsv3;
#[doc = "sram_cfg0 (rw) register accessor: an alias for `Reg<SRAM_CFG0_SPEC>`"]
pub type SRAM_CFG0 = crate::Reg<sram_cfg0::SRAM_CFG0_SPEC>;
#[doc = "reg_sram_ret"]
pub mod sram_cfg0;
#[doc = "sram_cfg1 (rw) register accessor: an alias for `Reg<SRAM_CFG1_SPEC>`"]
pub type SRAM_CFG1 = crate::Reg<sram_cfg1::SRAM_CFG1_SPEC>;
#[doc = "reg_sram_slp"]
pub mod sram_cfg1;
#[doc = "sram_cfg2 (rw) register accessor: an alias for `Reg<SRAM_CFG2_SPEC>`"]
pub type SRAM_CFG2 = crate::Reg<sram_cfg2::SRAM_CFG2_SPEC>;
#[doc = "reg_sram_parm"]
pub mod sram_cfg2;
#[doc = "sram_cfg3 (rw) register accessor: an alias for `Reg<SRAM_CFG3_SPEC>`"]
pub type SRAM_CFG3 = crate::Reg<sram_cfg3::SRAM_CFG3_SPEC>;
#[doc = "sram_cfg3"]
pub mod sram_cfg3;
#[doc = "sram_cfg4 (rw) register accessor: an alias for `Reg<SRAM_CFG4_SPEC>`"]
pub type SRAM_CFG4 = crate::Reg<sram_cfg4::SRAM_CFG4_SPEC>;
#[doc = "reg_sram_parm2"]
pub mod sram_cfg4;
#[doc = "psram_cfg0 (rw) register accessor: an alias for `Reg<PSRAM_CFG0_SPEC>`"]
pub type PSRAM_CFG0 = crate::Reg<psram_cfg0::PSRAM_CFG0_SPEC>;
#[doc = "psram_cfg0"]
pub mod psram_cfg0;
#[doc = "ldo28cis (rw) register accessor: an alias for `Reg<LDO28CIS_SPEC>`"]
pub type LDO28CIS = crate::Reg<ldo28cis::LDO28CIS_SPEC>;
#[doc = "ldo28cis"]
pub mod ldo28cis;
#[doc = "ldo18io (rw) register accessor: an alias for `Reg<LDO18IO_SPEC>`"]
pub type LDO18IO = crate::Reg<ldo18io::LDO18IO_SPEC>;
#[doc = "ldo18io"]
pub mod ldo18io;
#[doc = "ldo15cis (rw) register accessor: an alias for `Reg<LDO15CIS_SPEC>`"]
pub type LDO15CIS = crate::Reg<ldo15cis::LDO15CIS_SPEC>;
#[doc = "ldo15cis"]
pub mod ldo15cis;
#[doc = "ldo18flash (rw) register accessor: an alias for `Reg<LDO18FLASH_SPEC>`"]
pub type LDO18FLASH = crate::Reg<ldo18flash::LDO18FLASH_SPEC>;
#[doc = "ldo18flash"]
pub mod ldo18flash;
#[doc = "ldo12uhs (rw) register accessor: an alias for `Reg<LDO12UHS_SPEC>`"]
pub type LDO12UHS = crate::Reg<ldo12uhs::LDO12UHS_SPEC>;
#[doc = "ldo12uhs"]
pub mod ldo12uhs;
#[doc = "proc_mon (rw) register accessor: an alias for `Reg<PROC_MON_SPEC>`"]
pub type PROC_MON = crate::Reg<proc_mon::PROC_MON_SPEC>;
#[doc = "proc_mon"]
pub mod proc_mon;
#[doc = "dll_cfg0 (rw) register accessor: an alias for `Reg<DLL_CFG0_SPEC>`"]
pub type DLL_CFG0 = crate::Reg<dll_cfg0::DLL_CFG0_SPEC>;
#[doc = "dll_cfg0"]
pub mod dll_cfg0;
#[doc = "mipi_pll_cfg0 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG0_SPEC>`"]
pub type MIPI_PLL_CFG0 = crate::Reg<mipi_pll_cfg0::MIPI_PLL_CFG0_SPEC>;
#[doc = "mipi_pll_cfg0"]
pub mod mipi_pll_cfg0;
#[doc = "mipi_pll_cfg1 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG1_SPEC>`"]
pub type MIPI_PLL_CFG1 = crate::Reg<mipi_pll_cfg1::MIPI_PLL_CFG1_SPEC>;
#[doc = "mipi_pll_cfg1"]
pub mod mipi_pll_cfg1;
#[doc = "mipi_pll_cfg2 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG2_SPEC>`"]
pub type MIPI_PLL_CFG2 = crate::Reg<mipi_pll_cfg2::MIPI_PLL_CFG2_SPEC>;
#[doc = "mipi_pll_cfg2"]
pub mod mipi_pll_cfg2;
#[doc = "mipi_pll_cfg3 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG3_SPEC>`"]
pub type MIPI_PLL_CFG3 = crate::Reg<mipi_pll_cfg3::MIPI_PLL_CFG3_SPEC>;
#[doc = "mipi_pll_cfg3"]
pub mod mipi_pll_cfg3;
#[doc = "mipi_pll_cfg4 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG4_SPEC>`"]
pub type MIPI_PLL_CFG4 = crate::Reg<mipi_pll_cfg4::MIPI_PLL_CFG4_SPEC>;
#[doc = "mipi_pll_cfg4"]
pub mod mipi_pll_cfg4;
#[doc = "mipi_pll_cfg5 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG5_SPEC>`"]
pub type MIPI_PLL_CFG5 = crate::Reg<mipi_pll_cfg5::MIPI_PLL_CFG5_SPEC>;
#[doc = "mipi_pll_cfg5"]
pub mod mipi_pll_cfg5;
#[doc = "mipi_pll_cfg6 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG6_SPEC>`"]
pub type MIPI_PLL_CFG6 = crate::Reg<mipi_pll_cfg6::MIPI_PLL_CFG6_SPEC>;
#[doc = "mipi_pll_cfg6"]
pub mod mipi_pll_cfg6;
#[doc = "mipi_pll_cfg7 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG7_SPEC>`"]
pub type MIPI_PLL_CFG7 = crate::Reg<mipi_pll_cfg7::MIPI_PLL_CFG7_SPEC>;
#[doc = "mipi_pll_cfg7"]
pub mod mipi_pll_cfg7;
#[doc = "mipi_pll_cfg8 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG8_SPEC>`"]
pub type MIPI_PLL_CFG8 = crate::Reg<mipi_pll_cfg8::MIPI_PLL_CFG8_SPEC>;
#[doc = "mipi_pll_cfg8"]
pub mod mipi_pll_cfg8;
#[doc = "mipi_pll_cfg9 (rw) register accessor: an alias for `Reg<MIPI_PLL_CFG9_SPEC>`"]
pub type MIPI_PLL_CFG9 = crate::Reg<mipi_pll_cfg9::MIPI_PLL_CFG9_SPEC>;
#[doc = "mipi_pll_cfg9"]
pub mod mipi_pll_cfg9;
#[doc = "uhs_pll_cfg0 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG0_SPEC>`"]
pub type UHS_PLL_CFG0 = crate::Reg<uhs_pll_cfg0::UHS_PLL_CFG0_SPEC>;
#[doc = "uhs_pll_cfg0"]
pub mod uhs_pll_cfg0;
#[doc = "uhs_pll_cfg1 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG1_SPEC>`"]
pub type UHS_PLL_CFG1 = crate::Reg<uhs_pll_cfg1::UHS_PLL_CFG1_SPEC>;
#[doc = "uhs_pll_cfg1"]
pub mod uhs_pll_cfg1;
#[doc = "uhs_pll_cfg2 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG2_SPEC>`"]
pub type UHS_PLL_CFG2 = crate::Reg<uhs_pll_cfg2::UHS_PLL_CFG2_SPEC>;
#[doc = "uhs_pll_cfg2"]
pub mod uhs_pll_cfg2;
#[doc = "uhs_pll_cfg3 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG3_SPEC>`"]
pub type UHS_PLL_CFG3 = crate::Reg<uhs_pll_cfg3::UHS_PLL_CFG3_SPEC>;
#[doc = "uhs_pll_cfg3"]
pub mod uhs_pll_cfg3;
#[doc = "uhs_pll_cfg4 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG4_SPEC>`"]
pub type UHS_PLL_CFG4 = crate::Reg<uhs_pll_cfg4::UHS_PLL_CFG4_SPEC>;
#[doc = "uhs_pll_cfg4"]
pub mod uhs_pll_cfg4;
#[doc = "uhs_pll_cfg5 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG5_SPEC>`"]
pub type UHS_PLL_CFG5 = crate::Reg<uhs_pll_cfg5::UHS_PLL_CFG5_SPEC>;
#[doc = "uhs_pll_cfg5"]
pub mod uhs_pll_cfg5;
#[doc = "uhs_pll_cfg6 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG6_SPEC>`"]
pub type UHS_PLL_CFG6 = crate::Reg<uhs_pll_cfg6::UHS_PLL_CFG6_SPEC>;
#[doc = "uhs_pll_cfg6"]
pub mod uhs_pll_cfg6;
#[doc = "uhs_pll_cfg7 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG7_SPEC>`"]
pub type UHS_PLL_CFG7 = crate::Reg<uhs_pll_cfg7::UHS_PLL_CFG7_SPEC>;
#[doc = "uhs_pll_cfg7"]
pub mod uhs_pll_cfg7;
#[doc = "uhs_pll_cfg8 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG8_SPEC>`"]
pub type UHS_PLL_CFG8 = crate::Reg<uhs_pll_cfg8::UHS_PLL_CFG8_SPEC>;
#[doc = "uhs_pll_cfg8"]
pub mod uhs_pll_cfg8;
#[doc = "uhs_pll_cfg9 (rw) register accessor: an alias for `Reg<UHS_PLL_CFG9_SPEC>`"]
pub type UHS_PLL_CFG9 = crate::Reg<uhs_pll_cfg9::UHS_PLL_CFG9_SPEC>;
#[doc = "uhs_pll_cfg9"]
pub mod uhs_pll_cfg9;
#[doc = "wifi_pll_cfg0 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG0_SPEC>`"]
pub type WIFI_PLL_CFG0 = crate::Reg<wifi_pll_cfg0::WIFI_PLL_CFG0_SPEC>;
#[doc = "wifi_pll_cfg0"]
pub mod wifi_pll_cfg0;
#[doc = "wifi_pll_cfg1 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG1_SPEC>`"]
pub type WIFI_PLL_CFG1 = crate::Reg<wifi_pll_cfg1::WIFI_PLL_CFG1_SPEC>;
#[doc = "wifi_pll_cfg1"]
pub mod wifi_pll_cfg1;
#[doc = "wifi_pll_cfg2 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG2_SPEC>`"]
pub type WIFI_PLL_CFG2 = crate::Reg<wifi_pll_cfg2::WIFI_PLL_CFG2_SPEC>;
#[doc = "wifi_pll_cfg2"]
pub mod wifi_pll_cfg2;
#[doc = "wifi_pll_cfg3 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG3_SPEC>`"]
pub type WIFI_PLL_CFG3 = crate::Reg<wifi_pll_cfg3::WIFI_PLL_CFG3_SPEC>;
#[doc = "wifi_pll_cfg3"]
pub mod wifi_pll_cfg3;
#[doc = "wifi_pll_cfg4 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG4_SPEC>`"]
pub type WIFI_PLL_CFG4 = crate::Reg<wifi_pll_cfg4::WIFI_PLL_CFG4_SPEC>;
#[doc = "wifi_pll_cfg4"]
pub mod wifi_pll_cfg4;
#[doc = "wifi_pll_cfg5 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG5_SPEC>`"]
pub type WIFI_PLL_CFG5 = crate::Reg<wifi_pll_cfg5::WIFI_PLL_CFG5_SPEC>;
#[doc = "wifi_pll_cfg5"]
pub mod wifi_pll_cfg5;
#[doc = "wifi_pll_cfg6 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG6_SPEC>`"]
pub type WIFI_PLL_CFG6 = crate::Reg<wifi_pll_cfg6::WIFI_PLL_CFG6_SPEC>;
#[doc = "wifi_pll_cfg6"]
pub mod wifi_pll_cfg6;
#[doc = "wifi_pll_cfg7 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG7_SPEC>`"]
pub type WIFI_PLL_CFG7 = crate::Reg<wifi_pll_cfg7::WIFI_PLL_CFG7_SPEC>;
#[doc = "wifi_pll_cfg7"]
pub mod wifi_pll_cfg7;
#[doc = "wifi_pll_cfg8 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG8_SPEC>`"]
pub type WIFI_PLL_CFG8 = crate::Reg<wifi_pll_cfg8::WIFI_PLL_CFG8_SPEC>;
#[doc = "wifi_pll_cfg8"]
pub mod wifi_pll_cfg8;
#[doc = "wifi_pll_cfg9 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG9_SPEC>`"]
pub type WIFI_PLL_CFG9 = crate::Reg<wifi_pll_cfg9::WIFI_PLL_CFG9_SPEC>;
#[doc = "wifi_pll_cfg9"]
pub mod wifi_pll_cfg9;
#[doc = "wifi_pll_cfg10 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG10_SPEC>`"]
pub type WIFI_PLL_CFG10 = crate::Reg<wifi_pll_cfg10::WIFI_PLL_CFG10_SPEC>;
#[doc = "wifi_pll_cfg10"]
pub mod wifi_pll_cfg10;
#[doc = "wifi_pll_cfg11 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG11_SPEC>`"]
pub type WIFI_PLL_CFG11 = crate::Reg<wifi_pll_cfg11::WIFI_PLL_CFG11_SPEC>;
#[doc = "wifi_pll_cfg11"]
pub mod wifi_pll_cfg11;
#[doc = "wifi_pll_cfg12 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG12_SPEC>`"]
pub type WIFI_PLL_CFG12 = crate::Reg<wifi_pll_cfg12::WIFI_PLL_CFG12_SPEC>;
#[doc = "wifi_pll_cfg12"]
pub mod wifi_pll_cfg12;
#[doc = "wifi_pll_cfg13 (rw) register accessor: an alias for `Reg<WIFI_PLL_CFG13_SPEC>`"]
pub type WIFI_PLL_CFG13 = crate::Reg<wifi_pll_cfg13::WIFI_PLL_CFG13_SPEC>;
#[doc = "wifi_pll_cfg13"]
pub mod wifi_pll_cfg13;
#[doc = "gauge (rw) register accessor: an alias for `Reg<GAUGE_SPEC>`"]
pub type GAUGE = crate::Reg<gauge::GAUGE_SPEC>;
#[doc = "gauge"]
pub mod gauge;
#[doc = "gauge_rx_fifo_ctrl (rw) register accessor: an alias for `Reg<GAUGE_RX_FIFO_CTRL_SPEC>`"]
pub type GAUGE_RX_FIFO_CTRL = crate::Reg<gauge_rx_fifo_ctrl::GAUGE_RX_FIFO_CTRL_SPEC>;
#[doc = "gauge_rx_fifo_ctrl"]
pub mod gauge_rx_fifo_ctrl;
#[doc = "gauge_rx_fifo_status (rw) register accessor: an alias for `Reg<GAUGE_RX_FIFO_STATUS_SPEC>`"]
pub type GAUGE_RX_FIFO_STATUS = crate::Reg<gauge_rx_fifo_status::GAUGE_RX_FIFO_STATUS_SPEC>;
#[doc = "gauge_rx_fifo_status"]
pub mod gauge_rx_fifo_status;
#[doc = "gauge_rx_fifo_data (rw) register accessor: an alias for `Reg<GAUGE_RX_FIFO_DATA_SPEC>`"]
pub type GAUGE_RX_FIFO_DATA = crate::Reg<gauge_rx_fifo_data::GAUGE_RX_FIFO_DATA_SPEC>;
#[doc = "gauge_rx_fifo_data"]
pub mod gauge_rx_fifo_data;
#[doc = "gpio_cfg0 (rw) register accessor: an alias for `Reg<GPIO_CFG0_SPEC>`"]
pub type GPIO_CFG0 = crate::Reg<gpio_cfg0::GPIO_CFG0_SPEC>;
#[doc = "gpio_cfg0"]
pub mod gpio_cfg0;
#[doc = "gpio_cfg1 (rw) register accessor: an alias for `Reg<GPIO_CFG1_SPEC>`"]
pub type GPIO_CFG1 = crate::Reg<gpio_cfg1::GPIO_CFG1_SPEC>;
#[doc = "gpio_cfg1"]
pub mod gpio_cfg1;
#[doc = "gpio_cfg2 (rw) register accessor: an alias for `Reg<GPIO_CFG2_SPEC>`"]
pub type GPIO_CFG2 = crate::Reg<gpio_cfg2::GPIO_CFG2_SPEC>;
#[doc = "gpio_cfg2"]
pub mod gpio_cfg2;
#[doc = "gpio_cfg3 (rw) register accessor: an alias for `Reg<GPIO_CFG3_SPEC>`"]
pub type GPIO_CFG3 = crate::Reg<gpio_cfg3::GPIO_CFG3_SPEC>;
#[doc = "gpio_cfg3"]
pub mod gpio_cfg3;
#[doc = "gpio_cfg4 (rw) register accessor: an alias for `Reg<GPIO_CFG4_SPEC>`"]
pub type GPIO_CFG4 = crate::Reg<gpio_cfg4::GPIO_CFG4_SPEC>;
#[doc = "gpio_cfg4"]
pub mod gpio_cfg4;
#[doc = "gpio_cfg5 (rw) register accessor: an alias for `Reg<GPIO_CFG5_SPEC>`"]
pub type GPIO_CFG5 = crate::Reg<gpio_cfg5::GPIO_CFG5_SPEC>;
#[doc = "gpio_cfg5"]
pub mod gpio_cfg5;
#[doc = "gpio_cfg6 (rw) register accessor: an alias for `Reg<GPIO_CFG6_SPEC>`"]
pub type GPIO_CFG6 = crate::Reg<gpio_cfg6::GPIO_CFG6_SPEC>;
#[doc = "gpio_cfg6"]
pub mod gpio_cfg6;
#[doc = "gpio_cfg7 (rw) register accessor: an alias for `Reg<GPIO_CFG7_SPEC>`"]
pub type GPIO_CFG7 = crate::Reg<gpio_cfg7::GPIO_CFG7_SPEC>;
#[doc = "gpio_cfg7"]
pub mod gpio_cfg7;
#[doc = "gpio_cfg8 (rw) register accessor: an alias for `Reg<GPIO_CFG8_SPEC>`"]
pub type GPIO_CFG8 = crate::Reg<gpio_cfg8::GPIO_CFG8_SPEC>;
#[doc = "gpio_cfg8"]
pub mod gpio_cfg8;
#[doc = "gpio_cfg9 (rw) register accessor: an alias for `Reg<GPIO_CFG9_SPEC>`"]
pub type GPIO_CFG9 = crate::Reg<gpio_cfg9::GPIO_CFG9_SPEC>;
#[doc = "gpio_cfg9"]
pub mod gpio_cfg9;
#[doc = "gpio_cfg10 (rw) register accessor: an alias for `Reg<GPIO_CFG10_SPEC>`"]
pub type GPIO_CFG10 = crate::Reg<gpio_cfg10::GPIO_CFG10_SPEC>;
#[doc = "gpio_cfg10"]
pub mod gpio_cfg10;
#[doc = "gpio_cfg11 (rw) register accessor: an alias for `Reg<GPIO_CFG11_SPEC>`"]
pub type GPIO_CFG11 = crate::Reg<gpio_cfg11::GPIO_CFG11_SPEC>;
#[doc = "gpio_cfg11"]
pub mod gpio_cfg11;
#[doc = "gpio_cfg12 (rw) register accessor: an alias for `Reg<GPIO_CFG12_SPEC>`"]
pub type GPIO_CFG12 = crate::Reg<gpio_cfg12::GPIO_CFG12_SPEC>;
#[doc = "gpio_cfg12"]
pub mod gpio_cfg12;
#[doc = "gpio_cfg13 (rw) register accessor: an alias for `Reg<GPIO_CFG13_SPEC>`"]
pub type GPIO_CFG13 = crate::Reg<gpio_cfg13::GPIO_CFG13_SPEC>;
#[doc = "gpio_cfg13"]
pub mod gpio_cfg13;
#[doc = "gpio_cfg14 (rw) register accessor: an alias for `Reg<GPIO_CFG14_SPEC>`"]
pub type GPIO_CFG14 = crate::Reg<gpio_cfg14::GPIO_CFG14_SPEC>;
#[doc = "gpio_cfg14"]
pub mod gpio_cfg14;
#[doc = "gpio_cfg15 (rw) register accessor: an alias for `Reg<GPIO_CFG15_SPEC>`"]
pub type GPIO_CFG15 = crate::Reg<gpio_cfg15::GPIO_CFG15_SPEC>;
#[doc = "gpio_cfg15"]
pub mod gpio_cfg15;
#[doc = "gpio_cfg16 (rw) register accessor: an alias for `Reg<GPIO_CFG16_SPEC>`"]
pub type GPIO_CFG16 = crate::Reg<gpio_cfg16::GPIO_CFG16_SPEC>;
#[doc = "gpio_cfg16"]
pub mod gpio_cfg16;
#[doc = "gpio_cfg17 (rw) register accessor: an alias for `Reg<GPIO_CFG17_SPEC>`"]
pub type GPIO_CFG17 = crate::Reg<gpio_cfg17::GPIO_CFG17_SPEC>;
#[doc = "gpio_cfg17"]
pub mod gpio_cfg17;
#[doc = "gpio_cfg18 (rw) register accessor: an alias for `Reg<GPIO_CFG18_SPEC>`"]
pub type GPIO_CFG18 = crate::Reg<gpio_cfg18::GPIO_CFG18_SPEC>;
#[doc = "gpio_cfg18"]
pub mod gpio_cfg18;
#[doc = "gpio_cfg19 (rw) register accessor: an alias for `Reg<GPIO_CFG19_SPEC>`"]
pub type GPIO_CFG19 = crate::Reg<gpio_cfg19::GPIO_CFG19_SPEC>;
#[doc = "gpio_cfg19"]
pub mod gpio_cfg19;
#[doc = "gpio_cfg20 (rw) register accessor: an alias for `Reg<GPIO_CFG20_SPEC>`"]
pub type GPIO_CFG20 = crate::Reg<gpio_cfg20::GPIO_CFG20_SPEC>;
#[doc = "gpio_cfg20"]
pub mod gpio_cfg20;
#[doc = "gpio_cfg21 (rw) register accessor: an alias for `Reg<GPIO_CFG21_SPEC>`"]
pub type GPIO_CFG21 = crate::Reg<gpio_cfg21::GPIO_CFG21_SPEC>;
#[doc = "gpio_cfg21"]
pub mod gpio_cfg21;
#[doc = "gpio_cfg22 (rw) register accessor: an alias for `Reg<GPIO_CFG22_SPEC>`"]
pub type GPIO_CFG22 = crate::Reg<gpio_cfg22::GPIO_CFG22_SPEC>;
#[doc = "gpio_cfg22"]
pub mod gpio_cfg22;
#[doc = "gpio_cfg23 (rw) register accessor: an alias for `Reg<GPIO_CFG23_SPEC>`"]
pub type GPIO_CFG23 = crate::Reg<gpio_cfg23::GPIO_CFG23_SPEC>;
#[doc = "gpio_cfg23"]
pub mod gpio_cfg23;
#[doc = "gpio_cfg24 (rw) register accessor: an alias for `Reg<GPIO_CFG24_SPEC>`"]
pub type GPIO_CFG24 = crate::Reg<gpio_cfg24::GPIO_CFG24_SPEC>;
#[doc = "gpio_cfg24"]
pub mod gpio_cfg24;
#[doc = "gpio_cfg25 (rw) register accessor: an alias for `Reg<GPIO_CFG25_SPEC>`"]
pub type GPIO_CFG25 = crate::Reg<gpio_cfg25::GPIO_CFG25_SPEC>;
#[doc = "gpio_cfg25"]
pub mod gpio_cfg25;
#[doc = "gpio_cfg26 (rw) register accessor: an alias for `Reg<GPIO_CFG26_SPEC>`"]
pub type GPIO_CFG26 = crate::Reg<gpio_cfg26::GPIO_CFG26_SPEC>;
#[doc = "gpio_cfg26"]
pub mod gpio_cfg26;
#[doc = "gpio_cfg27 (rw) register accessor: an alias for `Reg<GPIO_CFG27_SPEC>`"]
pub type GPIO_CFG27 = crate::Reg<gpio_cfg27::GPIO_CFG27_SPEC>;
#[doc = "gpio_cfg27"]
pub mod gpio_cfg27;
#[doc = "gpio_cfg28 (rw) register accessor: an alias for `Reg<GPIO_CFG28_SPEC>`"]
pub type GPIO_CFG28 = crate::Reg<gpio_cfg28::GPIO_CFG28_SPEC>;
#[doc = "gpio_cfg28"]
pub mod gpio_cfg28;
#[doc = "gpio_cfg29 (rw) register accessor: an alias for `Reg<GPIO_CFG29_SPEC>`"]
pub type GPIO_CFG29 = crate::Reg<gpio_cfg29::GPIO_CFG29_SPEC>;
#[doc = "gpio_cfg29"]
pub mod gpio_cfg29;
#[doc = "gpio_cfg30 (rw) register accessor: an alias for `Reg<GPIO_CFG30_SPEC>`"]
pub type GPIO_CFG30 = crate::Reg<gpio_cfg30::GPIO_CFG30_SPEC>;
#[doc = "gpio_cfg30"]
pub mod gpio_cfg30;
#[doc = "gpio_cfg31 (rw) register accessor: an alias for `Reg<GPIO_CFG31_SPEC>`"]
pub type GPIO_CFG31 = crate::Reg<gpio_cfg31::GPIO_CFG31_SPEC>;
#[doc = "gpio_cfg31"]
pub mod gpio_cfg31;
#[doc = "gpio_cfg32 (rw) register accessor: an alias for `Reg<GPIO_CFG32_SPEC>`"]
pub type GPIO_CFG32 = crate::Reg<gpio_cfg32::GPIO_CFG32_SPEC>;
#[doc = "gpio_cfg32"]
pub mod gpio_cfg32;
#[doc = "gpio_cfg33 (rw) register accessor: an alias for `Reg<GPIO_CFG33_SPEC>`"]
pub type GPIO_CFG33 = crate::Reg<gpio_cfg33::GPIO_CFG33_SPEC>;
#[doc = "gpio_cfg33"]
pub mod gpio_cfg33;
#[doc = "gpio_cfg34 (rw) register accessor: an alias for `Reg<GPIO_CFG34_SPEC>`"]
pub type GPIO_CFG34 = crate::Reg<gpio_cfg34::GPIO_CFG34_SPEC>;
#[doc = "gpio_cfg34"]
pub mod gpio_cfg34;
#[doc = "gpio_cfg35 (rw) register accessor: an alias for `Reg<GPIO_CFG35_SPEC>`"]
pub type GPIO_CFG35 = crate::Reg<gpio_cfg35::GPIO_CFG35_SPEC>;
#[doc = "gpio_cfg35"]
pub mod gpio_cfg35;
#[doc = "gpio_cfg36 (rw) register accessor: an alias for `Reg<GPIO_CFG36_SPEC>`"]
pub type GPIO_CFG36 = crate::Reg<gpio_cfg36::GPIO_CFG36_SPEC>;
#[doc = "gpio_cfg36"]
pub mod gpio_cfg36;
#[doc = "gpio_cfg37 (rw) register accessor: an alias for `Reg<GPIO_CFG37_SPEC>`"]
pub type GPIO_CFG37 = crate::Reg<gpio_cfg37::GPIO_CFG37_SPEC>;
#[doc = "gpio_cfg37"]
pub mod gpio_cfg37;
#[doc = "gpio_cfg38 (rw) register accessor: an alias for `Reg<GPIO_CFG38_SPEC>`"]
pub type GPIO_CFG38 = crate::Reg<gpio_cfg38::GPIO_CFG38_SPEC>;
#[doc = "gpio_cfg38"]
pub mod gpio_cfg38;
#[doc = "gpio_cfg39 (rw) register accessor: an alias for `Reg<GPIO_CFG39_SPEC>`"]
pub type GPIO_CFG39 = crate::Reg<gpio_cfg39::GPIO_CFG39_SPEC>;
#[doc = "gpio_cfg39"]
pub mod gpio_cfg39;
#[doc = "gpio_cfg40 (rw) register accessor: an alias for `Reg<GPIO_CFG40_SPEC>`"]
pub type GPIO_CFG40 = crate::Reg<gpio_cfg40::GPIO_CFG40_SPEC>;
#[doc = "gpio_cfg40"]
pub mod gpio_cfg40;
#[doc = "gpio_cfg41 (rw) register accessor: an alias for `Reg<GPIO_CFG41_SPEC>`"]
pub type GPIO_CFG41 = crate::Reg<gpio_cfg41::GPIO_CFG41_SPEC>;
#[doc = "gpio_cfg41"]
pub mod gpio_cfg41;
#[doc = "gpio_cfg42 (rw) register accessor: an alias for `Reg<GPIO_CFG42_SPEC>`"]
pub type GPIO_CFG42 = crate::Reg<gpio_cfg42::GPIO_CFG42_SPEC>;
#[doc = "gpio_cfg42"]
pub mod gpio_cfg42;
#[doc = "gpio_cfg43 (rw) register accessor: an alias for `Reg<GPIO_CFG43_SPEC>`"]
pub type GPIO_CFG43 = crate::Reg<gpio_cfg43::GPIO_CFG43_SPEC>;
#[doc = "gpio_cfg43"]
pub mod gpio_cfg43;
#[doc = "gpio_cfg44 (rw) register accessor: an alias for `Reg<GPIO_CFG44_SPEC>`"]
pub type GPIO_CFG44 = crate::Reg<gpio_cfg44::GPIO_CFG44_SPEC>;
#[doc = "gpio_cfg44"]
pub mod gpio_cfg44;
#[doc = "gpio_cfg45 (rw) register accessor: an alias for `Reg<GPIO_CFG45_SPEC>`"]
pub type GPIO_CFG45 = crate::Reg<gpio_cfg45::GPIO_CFG45_SPEC>;
#[doc = "gpio_cfg45"]
pub mod gpio_cfg45;
#[doc = "gpio_cfg46 (rw) register accessor: an alias for `Reg<GPIO_CFG46_SPEC>`"]
pub type GPIO_CFG46 = crate::Reg<gpio_cfg46::GPIO_CFG46_SPEC>;
#[doc = "gpio_cfg46"]
pub mod gpio_cfg46;
#[doc = "gpio_cfg47 (rw) register accessor: an alias for `Reg<GPIO_CFG47_SPEC>`"]
pub type GPIO_CFG47 = crate::Reg<gpio_cfg47::GPIO_CFG47_SPEC>;
#[doc = "gpio_cfg47"]
pub mod gpio_cfg47;
#[doc = "gpio_cfg48 (rw) register accessor: an alias for `Reg<GPIO_CFG48_SPEC>`"]
pub type GPIO_CFG48 = crate::Reg<gpio_cfg48::GPIO_CFG48_SPEC>;
#[doc = "gpio_cfg48"]
pub mod gpio_cfg48;
#[doc = "gpio_cfg49 (rw) register accessor: an alias for `Reg<GPIO_CFG49_SPEC>`"]
pub type GPIO_CFG49 = crate::Reg<gpio_cfg49::GPIO_CFG49_SPEC>;
#[doc = "gpio_cfg49"]
pub mod gpio_cfg49;
#[doc = "gpio_cfg50 (rw) register accessor: an alias for `Reg<GPIO_CFG50_SPEC>`"]
pub type GPIO_CFG50 = crate::Reg<gpio_cfg50::GPIO_CFG50_SPEC>;
#[doc = "gpio_cfg50"]
pub mod gpio_cfg50;
#[doc = "gpio_cfg51 (rw) register accessor: an alias for `Reg<GPIO_CFG51_SPEC>`"]
pub type GPIO_CFG51 = crate::Reg<gpio_cfg51::GPIO_CFG51_SPEC>;
#[doc = "gpio_cfg51"]
pub mod gpio_cfg51;
#[doc = "gpio_cfg52 (rw) register accessor: an alias for `Reg<GPIO_CFG52_SPEC>`"]
pub type GPIO_CFG52 = crate::Reg<gpio_cfg52::GPIO_CFG52_SPEC>;
#[doc = "gpio_cfg52"]
pub mod gpio_cfg52;
#[doc = "gpio_cfg53 (rw) register accessor: an alias for `Reg<GPIO_CFG53_SPEC>`"]
pub type GPIO_CFG53 = crate::Reg<gpio_cfg53::GPIO_CFG53_SPEC>;
#[doc = "gpio_cfg53"]
pub mod gpio_cfg53;
#[doc = "gpio_cfg54 (rw) register accessor: an alias for `Reg<GPIO_CFG54_SPEC>`"]
pub type GPIO_CFG54 = crate::Reg<gpio_cfg54::GPIO_CFG54_SPEC>;
#[doc = "gpio_cfg54"]
pub mod gpio_cfg54;
#[doc = "gpio_cfg55 (rw) register accessor: an alias for `Reg<GPIO_CFG55_SPEC>`"]
pub type GPIO_CFG55 = crate::Reg<gpio_cfg55::GPIO_CFG55_SPEC>;
#[doc = "gpio_cfg55"]
pub mod gpio_cfg55;
#[doc = "gpio_cfg56 (rw) register accessor: an alias for `Reg<GPIO_CFG56_SPEC>`"]
pub type GPIO_CFG56 = crate::Reg<gpio_cfg56::GPIO_CFG56_SPEC>;
#[doc = "gpio_cfg56"]
pub mod gpio_cfg56;
#[doc = "gpio_cfg57 (rw) register accessor: an alias for `Reg<GPIO_CFG57_SPEC>`"]
pub type GPIO_CFG57 = crate::Reg<gpio_cfg57::GPIO_CFG57_SPEC>;
#[doc = "gpio_cfg57"]
pub mod gpio_cfg57;
#[doc = "gpio_cfg58 (rw) register accessor: an alias for `Reg<GPIO_CFG58_SPEC>`"]
pub type GPIO_CFG58 = crate::Reg<gpio_cfg58::GPIO_CFG58_SPEC>;
#[doc = "gpio_cfg58"]
pub mod gpio_cfg58;
#[doc = "gpio_cfg59 (rw) register accessor: an alias for `Reg<GPIO_CFG59_SPEC>`"]
pub type GPIO_CFG59 = crate::Reg<gpio_cfg59::GPIO_CFG59_SPEC>;
#[doc = "gpio_cfg59"]
pub mod gpio_cfg59;
#[doc = "gpio_cfg60 (rw) register accessor: an alias for `Reg<GPIO_CFG60_SPEC>`"]
pub type GPIO_CFG60 = crate::Reg<gpio_cfg60::GPIO_CFG60_SPEC>;
#[doc = "gpio_cfg60"]
pub mod gpio_cfg60;
#[doc = "gpio_cfg61 (rw) register accessor: an alias for `Reg<GPIO_CFG61_SPEC>`"]
pub type GPIO_CFG61 = crate::Reg<gpio_cfg61::GPIO_CFG61_SPEC>;
#[doc = "gpio_cfg61"]
pub mod gpio_cfg61;
#[doc = "gpio_cfg62 (rw) register accessor: an alias for `Reg<GPIO_CFG62_SPEC>`"]
pub type GPIO_CFG62 = crate::Reg<gpio_cfg62::GPIO_CFG62_SPEC>;
#[doc = "gpio_cfg62"]
pub mod gpio_cfg62;
#[doc = "gpio_cfg63 (rw) register accessor: an alias for `Reg<GPIO_CFG63_SPEC>`"]
pub type GPIO_CFG63 = crate::Reg<gpio_cfg63::GPIO_CFG63_SPEC>;
#[doc = "gpio_cfg63"]
pub mod gpio_cfg63;
#[doc = "gpio_cfg128 (rw) register accessor: an alias for `Reg<GPIO_CFG128_SPEC>`"]
pub type GPIO_CFG128 = crate::Reg<gpio_cfg128::GPIO_CFG128_SPEC>;
#[doc = "gpio_cfg128"]
pub mod gpio_cfg128;
#[doc = "gpio_cfg129 (rw) register accessor: an alias for `Reg<GPIO_CFG129_SPEC>`"]
pub type GPIO_CFG129 = crate::Reg<gpio_cfg129::GPIO_CFG129_SPEC>;
#[doc = "gpio_cfg129"]
pub mod gpio_cfg129;
#[doc = "gpio_cfg136 (rw) register accessor: an alias for `Reg<GPIO_CFG136_SPEC>`"]
pub type GPIO_CFG136 = crate::Reg<gpio_cfg136::GPIO_CFG136_SPEC>;
#[doc = "gpio_cfg136"]
pub mod gpio_cfg136;
#[doc = "gpio_cfg137 (rw) register accessor: an alias for `Reg<GPIO_CFG137_SPEC>`"]
pub type GPIO_CFG137 = crate::Reg<gpio_cfg137::GPIO_CFG137_SPEC>;
#[doc = "gpio_cfg137"]
pub mod gpio_cfg137;
#[doc = "gpio_cfg138 (rw) register accessor: an alias for `Reg<GPIO_CFG138_SPEC>`"]
pub type GPIO_CFG138 = crate::Reg<gpio_cfg138::GPIO_CFG138_SPEC>;
#[doc = "gpio_cfg138"]
pub mod gpio_cfg138;
#[doc = "gpio_cfg139 (rw) register accessor: an alias for `Reg<GPIO_CFG139_SPEC>`"]
pub type GPIO_CFG139 = crate::Reg<gpio_cfg139::GPIO_CFG139_SPEC>;
#[doc = "gpio_cfg139"]
pub mod gpio_cfg139;
#[doc = "gpio_cfg140 (rw) register accessor: an alias for `Reg<GPIO_CFG140_SPEC>`"]
pub type GPIO_CFG140 = crate::Reg<gpio_cfg140::GPIO_CFG140_SPEC>;
#[doc = "gpio_cfg140"]
pub mod gpio_cfg140;
#[doc = "gpio_cfg141 (rw) register accessor: an alias for `Reg<GPIO_CFG141_SPEC>`"]
pub type GPIO_CFG141 = crate::Reg<gpio_cfg141::GPIO_CFG141_SPEC>;
#[doc = "gpio_cfg141"]
pub mod gpio_cfg141;
#[doc = "gpio_cfg142 (rw) register accessor: an alias for `Reg<GPIO_CFG142_SPEC>`"]
pub type GPIO_CFG142 = crate::Reg<gpio_cfg142::GPIO_CFG142_SPEC>;
#[doc = "gpio_cfg142"]
pub mod gpio_cfg142;
#[doc = "gpio_cfg143 (rw) register accessor: an alias for `Reg<GPIO_CFG143_SPEC>`"]
pub type GPIO_CFG143 = crate::Reg<gpio_cfg143::GPIO_CFG143_SPEC>;
#[doc = "gpio_cfg143"]
pub mod gpio_cfg143;
