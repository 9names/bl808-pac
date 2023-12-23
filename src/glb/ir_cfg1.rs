#[doc = "Register `ir_cfg1` reader"]
pub struct R(crate::R<IR_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ir_cfg1` writer"]
pub struct W(crate::W<IR_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_CFG1_SPEC>;
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
impl From<crate::W<IR_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `led_din_reg` reader - "]
pub type LED_DIN_REG_R = crate::BitReader<bool>;
#[doc = "Field `led_din_reg` writer - "]
pub type LED_DIN_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_CFG1_SPEC, bool, O>;
#[doc = "Field `led_din_sel` reader - "]
pub type LED_DIN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `led_din_sel` writer - "]
pub type LED_DIN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_CFG1_SPEC, bool, O>;
#[doc = "Field `led_din_polarity_sel` reader - "]
pub type LED_DIN_POLARITY_SEL_R = crate::BitReader<bool>;
#[doc = "Field `led_din_polarity_sel` writer - "]
pub type LED_DIN_POLARITY_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `leddrv_ibias` reader - "]
pub type LEDDRV_IBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `leddrv_ibias` writer - "]
pub type LEDDRV_IBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IR_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ir_rx_gpio_sel` reader - "]
pub type IR_RX_GPIO_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ir_rx_gpio_sel` writer - "]
pub type IR_RX_GPIO_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IR_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_12_30` reader - "]
pub type RESERVED_12_30_R = crate::FieldReader<u32, u32>;
#[doc = "Field `pu_leddrv` reader - "]
pub type PU_LEDDRV_R = crate::BitReader<bool>;
#[doc = "Field `pu_leddrv` writer - "]
pub type PU_LEDDRV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_din_reg(&self) -> LED_DIN_REG_R {
        LED_DIN_REG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_din_sel(&self) -> LED_DIN_SEL_R {
        LED_DIN_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_din_polarity_sel(&self) -> LED_DIN_POLARITY_SEL_R {
        LED_DIN_POLARITY_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn leddrv_ibias(&self) -> LEDDRV_IBIAS_R {
        LEDDRV_IBIAS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&self) -> IR_RX_GPIO_SEL_R {
        IR_RX_GPIO_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:30"]
    #[inline(always)]
    pub fn reserved_12_30(&self) -> RESERVED_12_30_R {
        RESERVED_12_30_R::new((self.bits >> 12) & 0x0007_ffff)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_leddrv(&self) -> PU_LEDDRV_R {
        PU_LEDDRV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn led_din_reg(&mut self) -> LED_DIN_REG_W<0> {
        LED_DIN_REG_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn led_din_sel(&mut self) -> LED_DIN_SEL_W<1> {
        LED_DIN_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn led_din_polarity_sel(&mut self) -> LED_DIN_POLARITY_SEL_W<2> {
        LED_DIN_POLARITY_SEL_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn leddrv_ibias(&mut self) -> LEDDRV_IBIAS_W<4> {
        LEDDRV_IBIAS_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn ir_rx_gpio_sel(&mut self) -> IR_RX_GPIO_SEL_W<8> {
        IR_RX_GPIO_SEL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pu_leddrv(&mut self) -> PU_LEDDRV_W<31> {
        PU_LEDDRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ir_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir_cfg1](index.html) module"]
pub struct IR_CFG1_SPEC;
impl crate::RegisterSpec for IR_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir_cfg1::R](R) reader structure"]
impl crate::Readable for IR_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir_cfg1::W](W) writer structure"]
impl crate::Writable for IR_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ir_cfg1 to value 0x80"]
impl crate::Resettable for IR_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
