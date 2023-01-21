#[doc = "Register `cgen_cfg1` reader"]
pub struct R(crate::R<CGEN_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg1` writer"]
pub struct W(crate::W<CGEN_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CGEN_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_s1_rsvd0` reader - "]
pub type CGEN_S1_RSVD0_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd0` writer - "]
pub type CGEN_S1_RSVD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_1` reader - "]
pub type RESERVED_1_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_gpip` reader - "]
pub type CGEN_S1_GPIP_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_gpip` writer - "]
pub type CGEN_S1_GPIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_sec_dbg` reader - "]
pub type CGEN_S1_SEC_DBG_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_sec_dbg` writer - "]
pub type CGEN_S1_SEC_DBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_sec_eng` reader - "]
pub type CGEN_S1_SEC_ENG_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_sec_eng` writer - "]
pub type CGEN_S1_SEC_ENG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_tz` reader - "]
pub type CGEN_S1_TZ_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_tz` writer - "]
pub type CGEN_S1_TZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_rsvd6` reader - "]
pub type CGEN_S1_RSVD6_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd6` writer - "]
pub type CGEN_S1_RSVD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ef_ctrl` reader - "]
pub type CGEN_S1_EF_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ef_ctrl` writer - "]
pub type CGEN_S1_EF_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_rsvd8` reader - "]
pub type CGEN_S1_RSVD8_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd8` writer - "]
pub type CGEN_S1_RSVD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_rsvd9` reader - "]
pub type CGEN_S1_RSVD9_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd9` writer - "]
pub type CGEN_S1_RSVD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_rsvd10` reader - "]
pub type CGEN_S1_RSVD10_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd10` writer - "]
pub type CGEN_S1_RSVD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_sf_ctrl` reader - "]
pub type CGEN_S1_SF_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_sf_ctrl` writer - "]
pub type CGEN_S1_SF_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_dma` reader - "]
pub type CGEN_S1_DMA_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_dma` writer - "]
pub type CGEN_S1_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_rsvd13` reader - "]
pub type CGEN_S1_RSVD13_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd13` writer - "]
pub type CGEN_S1_RSVD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_rsvd14` reader - "]
pub type CGEN_S1_RSVD14_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd14` writer - "]
pub type CGEN_S1_RSVD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1_rsvd15` reader - "]
pub type CGEN_S1_RSVD15_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_rsvd15` writer - "]
pub type CGEN_S1_RSVD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_uart0` reader - "]
pub type CGEN_S1A_UART0_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_uart0` writer - "]
pub type CGEN_S1A_UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_uart1` reader - "]
pub type CGEN_S1A_UART1_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_uart1` writer - "]
pub type CGEN_S1A_UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_spi` reader - "]
pub type CGEN_S1A_SPI_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_spi` writer - "]
pub type CGEN_S1A_SPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_i2c` reader - "]
pub type CGEN_S1A_I2C_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_i2c` writer - "]
pub type CGEN_S1A_I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_pwm` reader - "]
pub type CGEN_S1A_PWM_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_pwm` writer - "]
pub type CGEN_S1A_PWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_timer` reader - "]
pub type CGEN_S1A_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_timer` writer - "]
pub type CGEN_S1A_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_ir` reader - "]
pub type CGEN_S1A_IR_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_ir` writer - "]
pub type CGEN_S1A_IR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_cks` reader - "]
pub type CGEN_S1A_CKS_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_cks` writer - "]
pub type CGEN_S1A_CKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_rsvd8` reader - "]
pub type CGEN_S1A_RSVD8_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_rsvd8` writer - "]
pub type CGEN_S1A_RSVD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_i2c1` reader - "]
pub type CGEN_S1A_I2C1_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_i2c1` writer - "]
pub type CGEN_S1A_I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_uart2` reader - "]
pub type CGEN_S1A_UART2_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_uart2` writer - "]
pub type CGEN_S1A_UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_rsvd11` reader - "]
pub type CGEN_S1A_RSVD11_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_rsvd11` writer - "]
pub type CGEN_S1A_RSVD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_rsvd12` reader - "]
pub type CGEN_S1A_RSVD12_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_rsvd12` writer - "]
pub type CGEN_S1A_RSVD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_rsvd13` reader - "]
pub type CGEN_S1A_RSVD13_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_rsvd13` writer - "]
pub type CGEN_S1A_RSVD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_rsvd14` reader - "]
pub type CGEN_S1A_RSVD14_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_rsvd14` writer - "]
pub type CGEN_S1A_RSVD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `cgen_s1a_rsvd15` reader - "]
pub type CGEN_S1A_RSVD15_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1a_rsvd15` writer - "]
pub type CGEN_S1A_RSVD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s1_rsvd0(&self) -> CGEN_S1_RSVD0_R {
        CGEN_S1_RSVD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reserved_1(&self) -> RESERVED_1_R {
        RESERVED_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_s1_gpip(&self) -> CGEN_S1_GPIP_R {
        CGEN_S1_GPIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_s1_sec_dbg(&self) -> CGEN_S1_SEC_DBG_R {
        CGEN_S1_SEC_DBG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s1_sec_eng(&self) -> CGEN_S1_SEC_ENG_R {
        CGEN_S1_SEC_ENG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cgen_s1_tz(&self) -> CGEN_S1_TZ_R {
        CGEN_S1_TZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cgen_s1_rsvd6(&self) -> CGEN_S1_RSVD6_R {
        CGEN_S1_RSVD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cgen_s1_ef_ctrl(&self) -> CGEN_S1_EF_CTRL_R {
        CGEN_S1_EF_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cgen_s1_rsvd8(&self) -> CGEN_S1_RSVD8_R {
        CGEN_S1_RSVD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cgen_s1_rsvd9(&self) -> CGEN_S1_RSVD9_R {
        CGEN_S1_RSVD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_s1_rsvd10(&self) -> CGEN_S1_RSVD10_R {
        CGEN_S1_RSVD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_s1_sf_ctrl(&self) -> CGEN_S1_SF_CTRL_R {
        CGEN_S1_SF_CTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cgen_s1_dma(&self) -> CGEN_S1_DMA_R {
        CGEN_S1_DMA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cgen_s1_rsvd13(&self) -> CGEN_S1_RSVD13_R {
        CGEN_S1_RSVD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cgen_s1_rsvd14(&self) -> CGEN_S1_RSVD14_R {
        CGEN_S1_RSVD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cgen_s1_rsvd15(&self) -> CGEN_S1_RSVD15_R {
        CGEN_S1_RSVD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_s1a_uart0(&self) -> CGEN_S1A_UART0_R {
        CGEN_S1A_UART0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cgen_s1a_uart1(&self) -> CGEN_S1A_UART1_R {
        CGEN_S1A_UART1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cgen_s1a_spi(&self) -> CGEN_S1A_SPI_R {
        CGEN_S1A_SPI_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cgen_s1a_i2c(&self) -> CGEN_S1A_I2C_R {
        CGEN_S1A_I2C_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cgen_s1a_pwm(&self) -> CGEN_S1A_PWM_R {
        CGEN_S1A_PWM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cgen_s1a_timer(&self) -> CGEN_S1A_TIMER_R {
        CGEN_S1A_TIMER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cgen_s1a_ir(&self) -> CGEN_S1A_IR_R {
        CGEN_S1A_IR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cgen_s1a_cks(&self) -> CGEN_S1A_CKS_R {
        CGEN_S1A_CKS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cgen_s1a_rsvd8(&self) -> CGEN_S1A_RSVD8_R {
        CGEN_S1A_RSVD8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cgen_s1a_i2c1(&self) -> CGEN_S1A_I2C1_R {
        CGEN_S1A_I2C1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cgen_s1a_uart2(&self) -> CGEN_S1A_UART2_R {
        CGEN_S1A_UART2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cgen_s1a_rsvd11(&self) -> CGEN_S1A_RSVD11_R {
        CGEN_S1A_RSVD11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cgen_s1a_rsvd12(&self) -> CGEN_S1A_RSVD12_R {
        CGEN_S1A_RSVD12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cgen_s1a_rsvd13(&self) -> CGEN_S1A_RSVD13_R {
        CGEN_S1A_RSVD13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cgen_s1a_rsvd14(&self) -> CGEN_S1A_RSVD14_R {
        CGEN_S1A_RSVD14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cgen_s1a_rsvd15(&self) -> CGEN_S1A_RSVD15_R {
        CGEN_S1A_RSVD15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd0(&mut self) -> CGEN_S1_RSVD0_W<0> {
        CGEN_S1_RSVD0_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_gpip(&mut self) -> CGEN_S1_GPIP_W<2> {
        CGEN_S1_GPIP_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_sec_dbg(&mut self) -> CGEN_S1_SEC_DBG_W<3> {
        CGEN_S1_SEC_DBG_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_sec_eng(&mut self) -> CGEN_S1_SEC_ENG_W<4> {
        CGEN_S1_SEC_ENG_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_tz(&mut self) -> CGEN_S1_TZ_W<5> {
        CGEN_S1_TZ_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd6(&mut self) -> CGEN_S1_RSVD6_W<6> {
        CGEN_S1_RSVD6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ef_ctrl(&mut self) -> CGEN_S1_EF_CTRL_W<7> {
        CGEN_S1_EF_CTRL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd8(&mut self) -> CGEN_S1_RSVD8_W<8> {
        CGEN_S1_RSVD8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd9(&mut self) -> CGEN_S1_RSVD9_W<9> {
        CGEN_S1_RSVD9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd10(&mut self) -> CGEN_S1_RSVD10_W<10> {
        CGEN_S1_RSVD10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_sf_ctrl(&mut self) -> CGEN_S1_SF_CTRL_W<11> {
        CGEN_S1_SF_CTRL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_dma(&mut self) -> CGEN_S1_DMA_W<12> {
        CGEN_S1_DMA_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd13(&mut self) -> CGEN_S1_RSVD13_W<13> {
        CGEN_S1_RSVD13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd14(&mut self) -> CGEN_S1_RSVD14_W<14> {
        CGEN_S1_RSVD14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_rsvd15(&mut self) -> CGEN_S1_RSVD15_W<15> {
        CGEN_S1_RSVD15_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_uart0(&mut self) -> CGEN_S1A_UART0_W<16> {
        CGEN_S1A_UART0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_uart1(&mut self) -> CGEN_S1A_UART1_W<17> {
        CGEN_S1A_UART1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_spi(&mut self) -> CGEN_S1A_SPI_W<18> {
        CGEN_S1A_SPI_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_i2c(&mut self) -> CGEN_S1A_I2C_W<19> {
        CGEN_S1A_I2C_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_pwm(&mut self) -> CGEN_S1A_PWM_W<20> {
        CGEN_S1A_PWM_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_timer(&mut self) -> CGEN_S1A_TIMER_W<21> {
        CGEN_S1A_TIMER_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_ir(&mut self) -> CGEN_S1A_IR_W<22> {
        CGEN_S1A_IR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_cks(&mut self) -> CGEN_S1A_CKS_W<23> {
        CGEN_S1A_CKS_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_rsvd8(&mut self) -> CGEN_S1A_RSVD8_W<24> {
        CGEN_S1A_RSVD8_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_i2c1(&mut self) -> CGEN_S1A_I2C1_W<25> {
        CGEN_S1A_I2C1_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_uart2(&mut self) -> CGEN_S1A_UART2_W<26> {
        CGEN_S1A_UART2_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_rsvd11(&mut self) -> CGEN_S1A_RSVD11_W<27> {
        CGEN_S1A_RSVD11_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_rsvd12(&mut self) -> CGEN_S1A_RSVD12_W<28> {
        CGEN_S1A_RSVD12_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_rsvd13(&mut self) -> CGEN_S1A_RSVD13_W<29> {
        CGEN_S1A_RSVD13_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_rsvd14(&mut self) -> CGEN_S1A_RSVD14_W<30> {
        CGEN_S1A_RSVD14_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a_rsvd15(&mut self) -> CGEN_S1A_RSVD15_W<31> {
        CGEN_S1A_RSVD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_s1a + cgen_s1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg1](index.html) module"]
pub struct CGEN_CFG1_SPEC;
impl crate::RegisterSpec for CGEN_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg1::R](R) reader structure"]
impl crate::Readable for CGEN_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg1::W](W) writer structure"]
impl crate::Writable for CGEN_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg1 to value 0x9b23_cffd"]
impl crate::Resettable for CGEN_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x9b23_cffd;
}
